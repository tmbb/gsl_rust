use crate::rng::RngType;
use crate::bindings;

/// This is the CRAY random number generator RANF. Its sequence is
///
/// x_{n+1} = (a x_n) mod m
/// defined on 48-bit unsigned integers with a = 44485709377909 and m = 2^48. The seed specifies the lower 32 bits of the initial value, x_1, with the lowest bit set to prevent the seed taking an even value. The upper 16 bits of x_1 are set to 0. A consequence of this procedure is that the pairs of seeds 2 and 3, 4 and 5, etc. produce the same sequences.
///
/// The generator compatible with the CRAY MATHLIB routine RANF. It produces double precision floating point numbers which should be identical to those from the original RANF.
///
/// There is a subtlety in the implementation of the seeding. The initial state is reversed through one step, by multiplying by the modular inverse of a mod m. This is done for compatibility with the original CRAY implementation.
///
/// Note that you can only seed the generator with integers up to 2^32, while the original CRAY implementation uses non-portable wide integers which can cover all 2^48 states of the generator.
///
/// The function gsl_rng_get returns the upper 32 bits from each term of the sequence. The function gsl_rng_uniform uses the full 48 bits to return the double precision number x_n/m.
///
/// The period of this generator is 2^46.
#[doc(alias = "gsl_rng_ranf")]
pub fn ranf() -> RngType {
    ffi_wrap!(gsl_rng_ranf)
}

/// This is the RANMAR lagged-fibonacci generator of Marsaglia, Zaman and Tsang. It is a 24-bit generator, originally designed for single-precision IEEE floating point numbers.
/// It was included in the CERNLIB high-energy physics library.
#[doc(alias = "gsl_rng_ranmar")]
pub fn ranmar() -> RngType {
    ffi_wrap!(gsl_rng_ranmar)
}

/// This is the shift-register generator of Kirkpatrick and Stoll. The sequence is based on the recurrence
///
/// x_n = x_{n-103} ^^ x_{n-250}
/// where ^^ denotes “exclusive-or”, defined on 32-bit words. The period of this generator is about 2^250 and it uses 250 words of state per generator.
///
/// For more information see,
///
/// S. Kirkpatrick and E. Stoll, “A very fast shift-register sequence random number generator”, Journal of Computational Physics, 40, 517–526 (1981)
#[doc(alias = "gsl_rng_r250")]
pub fn r250() -> RngType {
    ffi_wrap!(gsl_rng_r250)
}

/// This is an earlier version of the twisted generalized feedback shift-register generator, and has been superseded by the development of MT19937. However, it is
/// still an acceptable generator in its own right. It has a period of 2^800 and uses 33 words of storage per generator.
///
/// For more information see,
///
/// Makoto Matsumoto and Yoshiharu Kurita, “Twisted GFSR Generators II”, ACM Transactions on Modelling and Computer Simulation, Vol. 4, No. 3, 1994, pages 254–266.
#[doc(alias = "gsl_rng_tt800")]
pub fn tt800() -> RngType {
    ffi_wrap!(gsl_rng_tt800)
}

/// This is the VAX generator MTH$RANDOM. Its sequence is,
///
/// x_{n+1} = (a x_n + c) mod m
///
/// with a = 69069, c = 1 and m = 2^32. The seed specifies the initial value, x_1. The period of this generator is 2^32 and it uses 1 word of storage per generator.
#[doc(alias = "gsl_rng_vax")]
pub fn vax() -> RngType {
    ffi_wrap!(gsl_rng_vax)
}

/// This is the random number generator from the INMOS Transputer Development system. Its sequence is,
///
/// x_{n+1} = (a x_n) mod m
///
/// with a = 1664525 and m = 2^32. The seed specifies the initial value, x_1.
#[doc(alias = "gsl_rng_transputer")]
pub fn transputer() -> RngType {
    ffi_wrap!(gsl_rng_transputer)
}

/// This is the IBM RANDU generator. Its sequence is
///
/// x_{n+1} = (a x_n) mod m
///
/// with a = 65539 and m = 2^31. The seed specifies the initial value, x_1. The period of this generator was only 2^29. It has become a textbook example of a poor generator.
#[doc(alias = "gsl_rng_randu")]
pub fn randu() -> RngType {
    ffi_wrap!(gsl_rng_randu)
}

/// This is Park and Miller’s “minimal standard” MINSTD generator, a simple linear congruence which takes care to avoid the major pitfalls of such algorithms. Its sequence is,
///
/// x_{n+1} = (a x_n) mod m
///
/// with a = 16807 and m = 2^31 - 1 = 2147483647. The seed specifies the initial value, x_1. The period of this generator is about 2^31.
///
/// This generator was used in the IMSL Library (subroutine RNUN) and in MATLAB (the RAND function) in the past. It is also sometimes known by the acronym "GGL" (I'm not sure what that stands for).
///
/// For more information see,
///
/// Park and Miller, "Random Number Generators: Good ones are hard to find", Communications of the ACM, October 1988, Volume 31, No 10, pages 1192–1201.
#[doc(alias = "gsl_rng_minstd")]
pub fn minstd() -> RngType {
    ffi_wrap!(gsl_rng_minstd)
}

/// This is a reimplementation of the 16-bit SLATEC random number generator RUNIF. A generalization of the generator to 32 bits is provided by gsl_rng_uni32.
/// The original source code is available from NETLIB.
#[doc(alias = "gsl_rng_uni")]
pub fn uni() -> RngType {
    ffi_wrap!(gsl_rng_uni)
}

/// This is a reimplementation of the 16-bit SLATEC random number generator RUNIF. A generalization of the generator to 32 bits is provided by gsl_rng_uni32.
/// The original source code is available from NETLIB.
#[doc(alias = "gsl_rng_uni32")]
pub fn uni32() -> RngType {
    ffi_wrap!(gsl_rng_uni32)
}

/// This is the SLATEC random number generator RAND. It is ancient. The original source code is available from NETLIB.
#[doc(alias = "gsl_rng_slatec")]
pub fn slatec() -> RngType {
    ffi_wrap!(gsl_rng_slatec)
}

/// This is the ZUFALL lagged Fibonacci series generator of Peterson. Its sequence is,
///
/// t = u_{n-273} + u_{n-607}
/// u_n  = t - floor(t)
///
/// The original source code is available from NETLIB. For more information see,
///
/// W. Petersen, “Lagged Fibonacci Random Number Generators for the NEC SX-3”, International Journal of High Speed Computing (1994).
#[doc(alias = "gsl_rng_zuf")]
pub fn zuf() -> RngType {
    ffi_wrap!(gsl_rng_zuf)
}

/// This is a second-order multiple recursive generator described by Knuth in Seminumerical Algorithms, 3rd Ed., page 108. Its sequence is,
///
/// x_n = (a_1 x_{n-1} + a_2 x_{n-2}) mod m
///
/// with a_1 = 271828183, a_2 = 314159269, and m = 2^31 - 1.
#[doc(alias = "gsl_rng_knuthran2")]
pub fn knuthran2() -> RngType {
    ffi_wrap!(gsl_rng_knuthran2)
}

/// This is a second-order multiple recursive generator described by Knuth in Seminumerical Algorithms, 3rd Ed., Section 3.6. Knuth provides
/// its C code. The updated routine gsl_rng_knuthran2002 is from the revised 9th printing and corrects some weaknesses in the earlier version,
/// which is implemented as gsl_rng_knuthran.
#[doc(alias = "gsl_rng_knuthran2002")]
pub fn knuthran2002() -> RngType {
    ffi_wrap!(gsl_rng_knuthran2002)
}

/// This is a second-order multiple recursive generator described by Knuth in Seminumerical Algorithms, 3rd Ed., Section 3.6. Knuth provides
/// its C code. The updated routine gsl_rng_knuthran2002 is from the revised 9th printing and corrects some weaknesses in the earlier version,
/// which is implemented as gsl_rng_knuthran.
#[doc(alias = "gsl_rng_knuthran")]
pub fn knuthran() -> RngType {
    ffi_wrap!(gsl_rng_knuthran)
}

/// This multiplicative generator is taken from Knuth’s Seminumerical Algorithms, 3rd Ed., pages 106–108. Their sequence is,
///
/// x_{n+1} = (a x_n) mod m
///
/// where the seed specifies the initial value, x_1. The parameters a and m are as follows, Borosh-Niederreiter: a = 1812433253,
/// m = 2^32, Fishman18: a = 62089911, m = 2^31 - 1, Fishman20: a = 48271, m = 2^31 - 1, L’Ecuyer: a = 40692, m = 2^31 - 249,
/// Waterman: a = 1566083941, m = 2^32.
#[doc(alias = "gsl_rng_borosh13")]
pub fn borosh13() -> RngType {
    ffi_wrap!(gsl_rng_borosh13)
}

/// This multiplicative generator is taken from Knuth’s Seminumerical Algorithms, 3rd Ed., pages 106–108. Their sequence is,
///
/// x_{n+1} = (a x_n) mod m
///
/// where the seed specifies the initial value, x_1. The parameters a and m are as follows, Borosh-Niederreiter: a = 1812433253,
/// m = 2^32, Fishman18: a = 62089911, m = 2^31 - 1, Fishman20: a = 48271, m = 2^31 - 1, L’Ecuyer: a = 40692, m = 2^31 - 249,
/// Waterman: a = 1566083941, m = 2^32.
#[doc(alias = "gsl_rng_fishman18")]
pub fn fishman18() -> RngType {
    ffi_wrap!(gsl_rng_fishman18)
}

/// This multiplicative generator is taken from Knuth’s Seminumerical Algorithms, 3rd Ed., pages 106–108. Their sequence is,
///
/// x_{n+1} = (a x_n) mod m
///
/// where the seed specifies the initial value, x_1. The parameters a and m are as follows, Borosh-Niederreiter: a = 1812433253,
/// m = 2^32, Fishman18: a = 62089911, m = 2^31 - 1, Fishman20: a = 48271, m = 2^31 - 1, L’Ecuyer: a = 40692, m = 2^31 - 249,
/// Waterman: a = 1566083941, m = 2^32.
#[doc(alias = "gsl_rng_fishman20")]
pub fn fishman20() -> RngType {
    ffi_wrap!(gsl_rng_fishman20)
}

/// This multiplicative generator is taken from Knuth’s Seminumerical Algorithms, 3rd Ed., pages 106–108. Their sequence is,
///
/// x_{n+1} = (a x_n) mod m
///
/// where the seed specifies the initial value, x_1. The parameters a and m are as follows, Borosh-Niederreiter: a = 1812433253,
/// m = 2^32, Fishman18: a = 62089911, m = 2^31 - 1, Fishman20: a = 48271, m = 2^31 - 1, L’Ecuyer: a = 40692, m = 2^31 - 249,
/// Waterman: a = 1566083941, m = 2^32.
#[doc(alias = "gsl_rng_lecuyer21")]
pub fn lecuyer21() -> RngType {
    ffi_wrap!(gsl_rng_lecuyer21)
}

/// This multiplicative generator is taken from Knuth’s Seminumerical Algorithms, 3rd Ed., pages 106–108. Their sequence is,
///
/// x_{n+1} = (a x_n) mod m
///
/// where the seed specifies the initial value, x_1. The parameters a and m are as follows, Borosh-Niederreiter: a = 1812433253,
/// m = 2^32, Fishman18: a = 62089911, m = 2^31 - 1, Fishman20: a = 48271, m = 2^31 - 1, L’Ecuyer: a = 40692, m = 2^31 - 249,
/// Waterman: a = 1566083941, m = 2^32.
#[doc(alias = "gsl_rng_waterman14")]
pub fn waterman14() -> RngType {
    ffi_wrap!(gsl_rng_waterman14)
}

/// This is the L’Ecuyer–Fishman random number generator. It is taken from Knuth’s Seminumerical Algorithms, 3rd Ed., page 108. Its sequence is,
///
/// z_{n+1} = (x_n - y_n) mod m
///
/// with m = 2^31 - 1. x_n and y_n are given by the fishman20 and lecuyer21 algorithms. The seed specifies the initial value, x_1.
#[doc(alias = "gsl_rng_fishman2x")]
pub fn fishman2x() -> RngType {
    ffi_wrap!(gsl_rng_fishman2x)
}

/// This is the Coveyou random number generator. It is taken from Knuth’s Seminumerical Algorithms, 3rd Ed., Section 3.2.2. Its sequence is,
///
/// x_{n+1} = (x_n (x_n + 1)) mod m
///
/// with m = 2^32. The seed specifies the initial value, x_1.
#[doc(alias = "gsl_rng_coveyou")]
pub fn coveyou() -> RngType {
    ffi_wrap!(gsl_rng_coveyou)
}