use std::os::raw::*;

mod bindings {
    #![allow(dead_code)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(deref_nullptr)]

    include!("../bindings.rs");
}
use bindings::*;

type Result<T> = std::result::Result<T, GSLError>;

pub fn nonlinear_fit<F: Fn(f64, [f64; P]) -> Result<f64>, const P: usize>(
    max_iter: usize,
    xtol: f64,
    gtol: f64,
    ftol: f64,
    p0: [f64; P],
    data: &[(f64, f64)],
    f: F,
) -> Result<[f64; P]> {
    unsafe {
        // Define fit method and parameters
        let fit_type = gsl_multifit_nlinear_trust;
        let fit_params = gsl_multifit_nlinear_default_parameters();

        // Amount of datapoints
        let n = data.len() as u64;

        // Allocate workspace
        let workspace = gsl_multifit_nlinear_alloc(
            fit_type,
            &fit_params as *const _,
            n,        // amount of datapoints
            P as u64, // amount of parameters
        );
        assert!(!workspace.is_null());

        // Initial parameter guess
        let param_guess = gsl_vector_alloc(P as u64);
        assert!(!param_guess.is_null());
        for (i, &p) in p0.iter().enumerate() {
            gsl_vector_set(param_guess, i as u64, p);
        }

        // Information we need inside fit_f
        let ffi_params = (f, data);

        // Function to be optimized
        let mut fdf = gsl_multifit_function_fdf_struct {
            f: Some(fit_f::<F, P>),
            df: None,
            fdf: None,
            n,
            p: P as u64,
            params: &ffi_params as *const _ as *mut c_void,
            nevalf: 0,
            nevaldf: 0,
        };

        unsafe extern "C" fn fit_f<F: Fn(f64, [f64; P]) -> Result<f64>, const P: usize>(
            params: *const gsl_vector,
            ffi_params: *mut c_void,
            out: *mut gsl_vector,
        ) -> i32 {
            let mut param_cache = [0.0; P];
            for i in 0..P {
                param_cache[i] = gsl_vector_get(params, i as u64);
            }

            let (f, data): &(F, &[(f64, f64)]) = &*(ffi_params as *const _);

            for (i, &(x, y)) in data.iter().enumerate() {
                let err = y - match f(x, param_cache) {
                    Ok(y) => y,
                    Err(e) => return e.into(),
                };
                gsl_vector_set(out, i as u64, err);
            }

            GSL_SUCCESS
        }

        // Init workspace
        gsl_multifit_nlinear_init(
            param_guess,
            &mut fdf as *mut _ as *mut gsl_multifit_nlinear_fdf,
            workspace,
        );

        let mut _info = 0i32;
        let status = gsl_multifit_nlinear_driver(
            max_iter as u64,
            xtol,
            gtol,
            ftol,
            None,
            std::ptr::null_mut(),
            &mut _info as *mut _,
            workspace,
        );

        let mut param_cache = [0.0; P];
        for i in 0..P {
            param_cache[i] = gsl_vector_get((*workspace).x, i as u64);
        }
        // todo other statistics

        gsl_multifit_nlinear_free(workspace);
        gsl_vector_free(param_guess);

        GSLError::from_raw(status)?;
        Ok(param_cache)
    }
}

#[test]
fn test_fit() {
    for i in 0..10 {
        fn model(a: f64, b: f64, x: f64) -> f64 {
            a + b * x + (a * b) * x.powi(2)
        }

        let a = 10.0 + i as f64;
        let b = i as f64;

        let data = (0..1000)
            .map(|x| x as f64 / 100.0)
            .map(|x| (x, model(a, b, x)))
            .collect::<Vec<_>>();

        let fit_params = nonlinear_fit(
            1000,
            1.0e-9,
            1.0e-9,
            1.0e-9,
            [10.0, 5.0],
            &data,
            |x, params| {
                let a = params[0];
                let b = params[1];

                Ok(model(a, b, x))
            },
        )
        .unwrap();

        approx::assert_abs_diff_eq!(fit_params[0], a, epsilon = 1.0e-3);
        approx::assert_abs_diff_eq!(fit_params[1], b, epsilon = 1.0e-3);
    }
}

pub fn qag_gk61<F: Fn(f64) -> f64>(
    workspace_size: usize,
    a: f64,
    b: f64,
    epsabs: f64,
    epsrel: f64,
    f: F,
) -> Result<f64> {
    unsafe {
        let workspace = gsl_integration_workspace_alloc(workspace_size as u64);
        assert!(!workspace.is_null());

        let gsl_f = gsl_function_struct {
            function: Some(trampoline::<F>),
            params: &f as *const _ as *mut _,
        };

        let mut result = 0.0f64;
        let mut final_abserr = 0.0f64;

        let status = gsl_integration_qag(
            &gsl_f as *const _,
            a,
            b,
            epsabs,
            epsrel,
            workspace_size as u64,
            GSL_INTEG_GAUSS61 as c_int,
            workspace,
            &mut result as *mut _,
            &mut final_abserr as *mut _,
        );

        gsl_integration_workspace_free(workspace);

        GSLError::from_raw(status)?;
        Ok(result)
    }
}

#[test]
fn test_qag65() {
    approx::assert_abs_diff_eq!(
        qag_gk61(4, 0.0, 1.0, 1.0e-6, 0.0, |x| x.powi(3) + x).unwrap(),
        0.75,
        epsilon = 1.0e-6
    );
}

unsafe extern "C" fn trampoline<F: Fn(f64) -> f64>(x: f64, params: *mut c_void) -> f64 {
    let f: &F = &*(params as *const F);
    f(x)
}

pub fn disable_error_handler() {
    unsafe {
        gsl_set_error_handler_off();
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum GSLError {
    Failure,
    /// iteration has not converged
    Continue,
    /// input domain error, e.g sqrt(-1)
    Domain,
    /// output range error, e.g. exp(1e100)
    Range,
    /// invalid pointer
    Fault,
    /// invalid argument supplied by user
    Invalid,
    /// generic failure
    Failed,
    /// factorization failed
    Factorization,
    /// sanity check failed - shouldn't happen
    Sanity,
    /// malloc failed
    NoMemory,
    /// problem with user-supplied function
    BadFunction,
    /// iterative process is out of control
    RunAway,
    /// exceeded max number of iterations
    MaxIteration,
    /// tried to divide by zero
    ZeroDiv,
    /// user specified an invalid tolerance
    BadTolerance,
    /// failed to reach the specified tolerance
    Tolerance,
    /// underflow
    UnderFlow,
    /// overflow
    OverFlow,
    /// loss of accuracy
    Loss,
    /// failed because of roundoff error
    Round,
    /// matrix, vector lengths are not conformant
    BadLength,
    /// matrix not square
    NotSquare,
    /// apparent singularity detected
    Singularity,
    /// integral or series is divergent
    Diverge,
    /// requested feature is not supported by the hardware
    Unsupported,
    /// requested feature not (yet) implemented
    Unimplemented,
    /// cache limit exceeded
    Cache,
    /// table limit exceeded
    Table,
    /// iteration is not making progress towards solution
    NoProgress,
    /// jacobian evaluations are not improving the solution
    NoProgressJacobian,
    /// cannot reach the specified tolerance in F
    ToleranceF,
    /// cannot reach the specified tolerance in X
    ToleranceX,
    /// cannot reach the specified tolerance in gradient
    ToleranceG,
    /// cannot reach the specified tolerance in gradient
    #[allow(clippy::upper_case_acronyms)]
    EOF,
    /// Unknown value.
    Unknown(i32),
}

impl Into<c_int> for GSLError {
    fn into(self) -> c_int {
        match self {
            Self::Failure => GSL_FAILURE,
            Self::Continue => GSL_CONTINUE,
            Self::Domain => GSL_EDOM,
            Self::Range => GSL_ERANGE,
            Self::Fault => GSL_EFAULT,
            Self::Invalid => GSL_EINVAL,
            Self::Failed => GSL_EFAILED,
            Self::Factorization => GSL_EFACTOR,
            Self::Sanity => GSL_ESANITY,
            Self::NoMemory => GSL_ENOMEM,
            Self::BadFunction => GSL_EBADFUNC,
            Self::RunAway => GSL_ERUNAWAY,
            Self::MaxIteration => GSL_EMAXITER,
            Self::ZeroDiv => GSL_EZERODIV,
            Self::BadTolerance => GSL_EBADTOL,
            Self::Tolerance => GSL_ETOL,
            Self::UnderFlow => GSL_EUNDRFLW,
            Self::OverFlow => GSL_EOVRFLW,
            Self::Loss => GSL_ELOSS,
            Self::Round => GSL_EROUND,
            Self::BadLength => GSL_EBADLEN,
            Self::NotSquare => GSL_ENOTSQR,
            Self::Singularity => GSL_ESING,
            Self::Diverge => GSL_EDIVERGE,
            Self::Unsupported => GSL_EUNSUP,
            Self::Unimplemented => GSL_EUNIMPL,
            Self::Cache => GSL_ECACHE,
            Self::Table => GSL_ETABLE,
            Self::NoProgress => GSL_ENOPROG,
            Self::NoProgressJacobian => GSL_ENOPROGJ,
            Self::ToleranceF => GSL_ETOLF,
            Self::ToleranceX => GSL_ETOLX,
            Self::ToleranceG => GSL_ETOLG,
            Self::EOF => GSL_EOF,
            Self::Unknown(x) => x,
        }
    }
}

impl GSLError {
    fn from_raw(raw: c_int) -> Result<()> {
        match raw {
            GSL_SUCCESS => Ok(()),
            GSL_FAILURE => Err(Self::Failure),
            GSL_CONTINUE => Err(Self::Continue),
            GSL_EDOM => Err(Self::Domain),
            GSL_ERANGE => Err(Self::Range),
            GSL_EFAULT => Err(Self::Fault),
            GSL_EINVAL => Err(Self::Invalid),
            GSL_EFAILED => Err(Self::Failed),
            GSL_EFACTOR => Err(Self::Factorization),
            GSL_ESANITY => Err(Self::Sanity),
            GSL_ENOMEM => Err(Self::NoMemory),
            GSL_EBADFUNC => Err(Self::BadFunction),
            GSL_ERUNAWAY => Err(Self::RunAway),
            GSL_EMAXITER => Err(Self::MaxIteration),
            GSL_EZERODIV => Err(Self::ZeroDiv),
            GSL_EBADTOL => Err(Self::BadTolerance),
            GSL_ETOL => Err(Self::Tolerance),
            GSL_EUNDRFLW => Err(Self::UnderFlow),
            GSL_EOVRFLW => Err(Self::OverFlow),
            GSL_ELOSS => Err(Self::Loss),
            GSL_EROUND => Err(Self::Round),
            GSL_EBADLEN => Err(Self::BadLength),
            GSL_ENOTSQR => Err(Self::NotSquare),
            GSL_ESING => Err(Self::Singularity),
            GSL_EDIVERGE => Err(Self::Diverge),
            GSL_EUNSUP => Err(Self::Unsupported),
            GSL_EUNIMPL => Err(Self::Unimplemented),
            GSL_ECACHE => Err(Self::Cache),
            GSL_ETABLE => Err(Self::Table),
            GSL_ENOPROG => Err(Self::NoProgress),
            GSL_ENOPROGJ => Err(Self::NoProgressJacobian),
            GSL_ETOLF => Err(Self::ToleranceF),
            GSL_ETOLX => Err(Self::ToleranceX),
            GSL_ETOLG => Err(Self::ToleranceG),
            GSL_EOF => Err(Self::EOF),
            x => Err(Self::Unknown(x)),
        }
    }
}
