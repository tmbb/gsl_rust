/*
    rng.rs
    Copyright (C) 2021 Pim van den Berg
    Copyright (C) 2024 Tiago Barroso

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

#![allow(non_snake_case)]

use crate::bindings;
use crate::rng::Rng;

/// These functions compute results for the tail of a unit Gaussian
/// distribution.
/// They are equivalent to the functions above with a standard
/// deviation of one, `sigma` = 1.
/// 
/// Binds the function [`gsl_ran_ugaussian_tail`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_ugaussian_tail).
pub fn ugaussian_tail_rvs(mut r: Rng, a: f64) -> f64 {
    unsafe { bindings::gsl_ran_ugaussian_tail(&mut r, a) }
}

/// This function returns  a random variate from the Type-1 Gumbel
/// distribution.
/// The Type-1 Gumbel distribution function is,
/// 
/// $$p(x) dx = a b \exp(-(b \exp(-ax) + ax)) dx$$
/// 
/// for $-\infty < x < \infty$.
/// 
/// Binds the function [`gsl_ran_gumbel1`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_gumbel1).
pub fn gumbel1_rvs(mut r: Rng, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_ran_gumbel1(&mut r, a, b) }
}

/// This function returns a random variate from the Landau distribution.
/// The
/// probability distribution for Landau random variates is defined
/// analytically by the complex integral,
/// 
/// $$p(x) = {1 \over {2 \pi i}} \int_{c-i\infty}^{c+i\infty} ds\, \exp(s \log(s) + x s)$$
/// 
/// For numerical purposes it is more convenient to use the following
/// equivalent form of the integral,
/// 
/// $$p(x) = (1/\pi) \int_0^\infty dt \exp(-t \log(t) - x t) \sin(\pi t).$$
/// 
/// Binds the function [`gsl_ran_landau`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_landau).
pub fn landau_rvs(mut r: Rng) -> f64 {
    unsafe { bindings::gsl_ran_landau(&mut r) }
}

/// This function returns a random variate from the exponential power distribution
/// with scale parameter `a` and exponent `b`.
/// The distribution is,
/// 
/// $$p(x) dx = {1 \over 2 a \Gamma(1+1/b)} \exp(-|x/a|^b) dx$$
/// 
/// for $x \ge 0$.
/// For $b = 1$ this reduces to the Laplace
/// distribution.
/// For $b = 2$ it has the same form as a Gaussian
/// distribution, but with $a = \sqrt{2} \sigma$.
/// 
/// Binds the function [`gsl_ran_exppow`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_exppow).
pub fn exppow_rvs(mut r: Rng, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_ran_exppow(&mut r, a, b) }
}

/// This function returns a random integer from the logarithmic
/// distribution.
/// The probability distribution for logarithmic random variates
/// is,
/// 
/// $$p(k) = {-1 \over \log(1-p)} {\left( p^k \over k \right)}$$
/// 
/// for $k \ge 1$.
/// 
/// Binds the function [`gsl_ran_logarithmic`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_logarithmic).
pub fn logarithmic_rvs(mut r: Rng, p: f64) -> u32 {
    unsafe { bindings::gsl_ran_logarithmic(&mut r, p) }
}

/// These functions compute results for the unit Gaussian distribution.
/// They
/// are equivalent to the functions above with a standard deviation of one,
/// `sigma` = 1.
/// 
/// Binds the function [`gsl_ran_ugaussian_ratio_method`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_ugaussian_ratio_method).
pub fn ugaussian_ratio_method_rvs(mut r: Rng) -> f64 {
    unsafe { bindings::gsl_ran_ugaussian_ratio_method(&mut r) }
}

/// This function returns a random variate from the chi-squared distribution
/// with `nu` degrees of freedom. The distribution function is,
/// 
/// $$p(x) dx = {1 \over 2 \Gamma(\nu/2) } (x/2)^{\nu/2 - 1} \exp(-x/2) dx$$
/// 
/// for $x \ge 0$.
/// 
/// Binds the function [`gsl_ran_chisq`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_chisq).
pub fn chi2(mut r: Rng, nu: f64) -> f64 {
    unsafe { bindings::gsl_ran_chisq(&mut r, nu) }
}

/// This function computes a Gaussian random variate using the alternative
/// Marsaglia-Tsang ziggurat and Kinderman-Monahan-Leva ratio methods.
/// The
/// Ziggurat algorithm is the fastest available algorithm in most cases.
/// 
/// Binds the function [`gsl_ran_gaussian_ziggurat`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_gaussian_ziggurat).
pub fn gaussian_ziggurat_rvs(mut r: Rng, sigma: f64) -> f64 {
    unsafe { bindings::gsl_ran_gaussian_ziggurat(&mut r, sigma) }
}

/// This function returns a random variate from the tail of the Rayleigh
/// distribution with scale parameter `sigma` and a lower limit of
/// `a`.
/// The distribution is,
/// 
/// $$p(x) dx = {x \over \sigma^2} \exp ((a^2 - x^2) /(2 \sigma^2)) dx$$
/// 
/// for $x > a$.
/// 
/// Binds the function [`gsl_ran_rayleigh_tail`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_rayleigh_tail).
pub fn rayleigh_tail_rvs(mut r: Rng, a: f64, sigma: f64) -> f64 {
    unsafe { bindings::gsl_ran_rayleigh_tail(&mut r, a, sigma) }
}

/// This function returns a random variate from the Laplace distribution
/// with width `a`.
/// The distribution is,
/// 
/// $$p(x) dx = {1 \over 2 a}  \exp(-|x/a|) dx$$
/// 
/// for $-\infty < x < \infty$.
/// 
/// Binds the function [`gsl_ran_laplace`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_laplace).
pub fn laplace_rvs(mut r: Rng, a: f64) -> f64 {
    unsafe { bindings::gsl_ran_laplace(&mut r, a) }
}

/// This function returns a random variate from the Levy symmetric stable
/// distribution with scale `c` and exponent `alpha`.
/// The symmetric
/// stable probability distribution is defined by a Fourier transform,
/// 
/// $$p(x) = {1 \over 2 \pi} \int_{-\infty}^{+\infty} dt \exp(-it x - |c t|^\alpha)$$
/// 
/// There is no explicit solution for the form of $p(x)$ and the
/// library does not define a corresponding `pdf` function.
/// For
/// $\alpha = 1$ the distribution reduces to the Cauchy distribution.
/// For
/// $\alpha = 2$ it is a Gaussian distribution with $\sigma = \sqrt{2} c$.
/// For $\alpha < 1$ the tails of the distribution become extremely wide.
/// 
/// The algorithm only works for $0 < \alpha \le 2$.
/// 
/// ![_images/rand-levy.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-levy.png)
/// 
/// Binds the function [`gsl_ran_levy`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_levy).
pub fn levy_rvs(mut r: Rng, c: f64, alpha: f64) -> f64 {
    unsafe { bindings::gsl_ran_levy(&mut r, c, alpha) }
}

/// This function returns a gamma variate using the algorithms from Knuth (vol 2).
/// 
/// Binds the function [`gsl_ran_gamma_knuth`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_gamma_knuth).
pub fn gamma_knuth_rvs(mut r: Rng, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_ran_gamma_knuth(&mut r, a, b) }
}

/// This function returns a random variate from the t-distribution.
/// The
/// distribution function is,
/// 
/// $$p(x) dx = {\Gamma((\nu + 1)/2) \over \sqrt{\pi \nu} \Gamma(\nu/2)}
///    (1 + x^2/\nu)^{-(\nu + 1)/2} dx$$
/// 
/// for $-\infty < x < +\infty$.
/// 
/// Binds the function [`gsl_ran_tdist`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_tdist).
pub fn student_t(mut r: Rng, nu: f64) -> f64 {
    unsafe { bindings::gsl_ran_tdist(&mut r, nu) }
}

/// This function returns a random variate from the Levy skew stable
/// distribution with scale `c`, exponent `alpha` and skewness
/// parameter `beta`.
/// The skewness parameter must lie in the range
/// $[-1,1]$.
/// The Levy skew stable probability distribution is defined
/// by a Fourier transform,
/// 
/// $$p(x) = {1 \over 2 \pi} \int_{-\infty}^{+\infty} dt \exp(-it x - |c t|^\alpha (1-i \beta \sgn(t) \tan(\pi\alpha/2)))$$
/// 
/// When $\alpha = 1$ the term $\tan(\pi \alpha/2)$ is replaced by
/// $-(2/\pi)\log|t|$.
/// There is no explicit solution for the form of
/// $p(x)$ and the library does not define a corresponding `pdf`
/// function.
/// For $\alpha = 2$ the distribution reduces to a Gaussian
/// distribution with $\sigma = \sqrt{2} c$
/// and the skewness parameter has no effect.
/// For $\alpha < 1$ the tails of the distribution become extremely
/// wide.
/// The symmetric distribution corresponds to $\beta = 0$.
/// 
/// The algorithm only works for $0 < \alpha \le 2$.
/// 
/// Binds the function [`gsl_ran_levy_skew`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_levy_skew).
pub fn levy_skew_rvs(mut r: Rng, c: f64, alpha: f64, beta: f64) -> f64 {
    unsafe { bindings::gsl_ran_levy_skew(&mut r, c, alpha, beta) }
}

/// This function provides random variates from the upper tail of a Gaussian
/// distribution with standard deviation `sigma`.
/// The values returned
/// are larger than the lower limit `a`, which must be positive.
/// The
/// method is based on Marsaglia’s famous rectangle-wedge-tail algorithm (Ann.
/// Math. Stat. 32, 894–899 (1961)), with this aspect explained in Knuth, v2,
/// 3rd ed, p139,586 (exercise 11).
/// 
/// The probability distribution for Gaussian tail random variates is,
/// 
/// $$p(x) dx = {1 \over N(a;\sigma) \sqrt{2 \pi \sigma^2}} \exp (- x^2 / 2\sigma^2) dx$$
/// 
/// for $x > a$ where $N(a;\sigma)$ is the normalization constant,
/// 
/// $$N(a;\sigma) = {1 \over 2} \hbox{erfc}\left({a \over \sqrt{2 \sigma^2}}\right).$$
/// 
/// Binds the function [`gsl_ran_gaussian_tail`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_gaussian_tail).
pub fn gaussian_tail_rvs(mut r: Rng, a: f64, sigma: f64) -> f64 {
    unsafe { bindings::gsl_ran_gaussian_tail(&mut r, a, sigma) }
}

/// This function returns a random variate from the Rayleigh distribution with
/// scale parameter `sigma`.
/// The distribution is,
/// 
/// $$p(x) dx = {x \over \sigma^2} \exp(- x^2/(2 \sigma^2)) dx$$
/// 
/// for $x > 0$.
/// 
/// Binds the function [`gsl_ran_rayleigh`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_rayleigh).
pub fn rayleigh_rvs(mut r: Rng, sigma: f64) -> f64 {
    unsafe { bindings::gsl_ran_rayleigh(&mut r, sigma) }
}

/// This function returns a random variate from the beta
/// distribution.
/// The distribution function is,
/// 
/// $$p(x) dx = {\Gamma(a+b) \over \Gamma(a) \Gamma(b)} x^{a-1} (1-x)^{b-1} dx$$
/// 
/// for $0 \le x \le 1$.
/// 
/// Binds the function [`gsl_ran_beta`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_beta).
pub fn beta_rvs(mut r: Rng, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_ran_beta(&mut r, a, b) }
}

/// This function returns a random variate from the Cauchy distribution with
/// scale parameter `a`.
/// The probability distribution for Cauchy
/// random variates is,
/// 
/// $$p(x) dx = {1 \over a\pi (1 + (x/a)^2) } dx$$
/// 
/// for $x$ in the range $-\infty$ to $+\infty$.
/// The Cauchy
/// distribution is also known as the Lorentz distribution.
/// 
/// Binds the function [`gsl_ran_cauchy`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_cauchy).
pub fn cauchy_rvs(mut r: Rng, a: f64) -> f64 {
    unsafe { bindings::gsl_ran_cauchy(&mut r, a) }
}

/// This function returns a random variate from the lognormal
/// distribution.
/// The distribution function is,
/// 
/// $$p(x) dx = {1 \over x \sqrt{2 \pi \sigma^2}} \exp(-(\ln(x) - \zeta)^2/2 \sigma^2) dx$$
/// 
/// for $x > 0$.
/// 
/// Binds the function [`gsl_ran_lognormal`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_lognormal).
pub fn lognormal_rvs(mut r: Rng, zeta: f64, sigma: f64) -> f64 {
    unsafe { bindings::gsl_ran_lognormal(&mut r, zeta, sigma) }
}

/// This function returns a random variate from the Pareto distribution of
/// order `a`.
/// The distribution function is,
/// 
/// $$p(x) dx = (a/b) / (x/b)^{a+1} dx$$
/// 
/// for $x \ge b$.
/// 
/// Binds the function [`gsl_ran_pareto`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_pareto).
pub fn pareto_rvs(mut r: Rng, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_ran_pareto(&mut r, a, b) }
}

/// This function returns a random variate from the flat (uniform)
/// distribution from `a` to `b`. The distribution is,
/// 
/// $$p(x) dx = {1 \over (b-a)} dx$$
/// 
/// if $a \le x < b$ and 0 otherwise.
/// 
/// Binds the function [`gsl_ran_flat`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_flat).
pub fn flat_rvs(mut r: Rng, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_ran_flat(&mut r, a, b) }
}

/// This function returns a random variate from the Type-2 Gumbel
/// distribution.
/// The Type-2 Gumbel distribution function is,
/// 
/// $$p(x) dx = a b x^{-a-1} \exp(-b x^{-a}) dx$$
/// 
/// for $0 < x < \infty$.
/// 
/// Binds the function [`gsl_ran_gumbel2`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_gumbel2).
pub fn gumbel2_rvs(mut r: Rng, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_ran_gumbel2(&mut r, a, b) }
}

/// This function returns either 0 or 1, the result of a Bernoulli trial
/// with probability `p`.
/// The probability distribution for a Bernoulli
/// trial is,
/// 
/// $$p(0) & = 1 - p \\
/// p(1) & = p$$
/// 
/// Binds the function [`gsl_ran_bernoulli`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_bernoulli).
pub fn bernoulli_rvs(mut r: Rng, p: f64) -> u32 {
    unsafe { bindings::gsl_ran_bernoulli(&mut r, p) }
}

/// This function returns a random variate from the logistic
/// distribution.
/// The distribution function is,
/// 
/// $$p(x) dx = { \exp(-x/a) \over a (1 + \exp(-x/a))^2 } dx$$
/// 
/// for $-\infty < x < +\infty$.
/// 
/// Binds the function [`gsl_ran_logistic`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_logistic).
pub fn logistic_rvs(mut r: Rng, a: f64) -> f64 {
    unsafe { bindings::gsl_ran_logistic(&mut r, a) }
}

/// This function returns a random integer from the hypergeometric
/// distribution.
/// The probability distribution for hypergeometric
/// random variates is,
/// 
/// $$p(k) = C(n_1, k) C(n_2, t - k) / C(n_1 + n_2, t)$$
/// 
/// where $C(a,b) = a!/(b!(a-b)!)$ and
/// $t \leq n_1 + n_2$.
/// The domain of $k$ is
/// $\max(0, t - n_2), \ldots, \min(t, n_1)$
/// 
/// If a population contains $n_1$ elements of “type 1” and
/// $n_2$ elements of “type 2” then the hypergeometric
/// distribution gives the probability of obtaining $k$ elements of
/// “type 1” in $t$ samples from the population without
/// replacement.
/// 
/// Binds the function [`gsl_ran_hypergeometric`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_hypergeometric).
pub fn hypergeometric_rvs(mut r: Rng, n1: u32, n2: u32, t: u32) -> u32 {
    unsafe { bindings::gsl_ran_hypergeometric(&mut r, n1, n2, t) }
}

/// This function returns a random integer from the binomial distribution,
/// the number of successes in `n` independent trials with probability
/// `p`.
/// The probability distribution for binomial variates is,
/// 
/// $$p(k) = {n! \over k! (n-k)!} p^k (1-p)^{n-k}$$
/// 
/// for $0 \le k \le n$.
/// 
/// Binds the function [`gsl_ran_binomial`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_binomial).
pub fn binomial_rvs(mut r: Rng, p: f64, n: u32) -> u32 {
    unsafe { bindings::gsl_ran_binomial(&mut r, p, n) }
}

/// This function returns a Gaussian random variate, with mean zero and
/// standard deviation `sigma`.
/// The probability distribution for
/// Gaussian random variates is,
/// 
/// $$p(x) dx = {1 \over \sqrt{2 \pi \sigma^2}} \exp (-x^2 / 2\sigma^2) dx$$
/// 
/// for $x$ in the range $-\infty$ to $+\infty$.
/// Use the
/// transformation $z = \mu + x$ on the numbers returned by
/// `gsl_ran_gaussian()` to obtain a Gaussian distribution with mean
/// $\mu$.
/// This function uses the Box-Muller algorithm which requires two
/// calls to the random number generator `r`.
/// 
/// Binds the function [`gsl_ran_gaussian`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_gaussian).
pub fn gaussian_rvs(mut r: Rng, sigma: f64) -> f64 {
    unsafe { bindings::gsl_ran_gaussian(&mut r, sigma) }
}

/// This function returns a random variate from the Weibull distribution.
/// The
/// distribution function is,
/// 
/// $$p(x) dx = {b \over a^b} x^{b-1}  \exp(-(x/a)^b) dx$$
/// 
/// for $x \ge 0$.
/// 
/// Binds the function [`gsl_ran_weibull`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_weibull).
pub fn weibull_rvs(mut r: Rng, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_ran_weibull(&mut r, a, b) }
}

/// This function returns a random integer from the geometric distribution,
/// the number of independent trials with probability `p` until the
/// first success.
/// The probability distribution for geometric variates
/// is,
/// 
/// $$p(k) = p (1-p)^{k-1}$$
/// 
/// for $k \ge 1$.
/// Note that the distribution begins with $k = 1$ with this
/// definition.
/// There is another convention in which the exponent $k - 1$
/// is replaced by $k$.
/// 
/// Binds the function [`gsl_ran_geometric`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_geometric).
pub fn geometric_rvs(mut r: Rng, p: f64) -> u32 {
    unsafe { bindings::gsl_ran_geometric(&mut r, p) }
}

/// This function returns a random integer from the negative binomial
/// distribution, the number of failures occurring before `n` successes
/// in independent trials with probability `p` of success.
/// The
/// probability distribution for negative binomial variates is,
/// 
/// $$p(k) = {\Gamma(n + k) \over \Gamma(k+1) \Gamma(n) } p^n (1-p)^k$$
/// 
/// Note that $n$ is not required to be an integer.
/// 
/// Binds the function [`gsl_ran_negative_binomial`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_negative_binomial).
pub fn negative_binomial_rvs(mut r: Rng, p: f64, n: f64) -> u32 {
    unsafe { bindings::gsl_ran_negative_binomial(&mut r, p, n) }
}

/// This function returns a random variate from the gamma
/// distribution.
/// The distribution function is,
/// 
/// $$p(x) dx = {1 \over \Gamma(a) b^a} x^{a-1} e^{-x/b} dx$$
/// 
/// for $x > 0$.
/// 
/// The gamma distribution with an integer parameter `a` is known as the Erlang distribution.
/// 
/// The variates are computed using the Marsaglia-Tsang fast gamma method.
/// This function for this method was previously called
/// `gsl_ran_gamma_mt()` and can still be accessed using this name.
/// 
/// Binds the function [`gsl_ran_gamma`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_gamma).
pub fn gamma_rvs(mut r: Rng, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_ran_gamma(&mut r, a, b) }
}

/// This function returns a random integer from the Poisson distribution
/// with mean `mu`.
/// The probability distribution for Poisson variates is,
/// 
/// $$p(k) = {\mu^k \over k!} \exp(-\mu)$$
/// 
/// for $k \ge 0$.
/// 
/// Binds the function [`gsl_ran_poisson`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_poisson).
pub fn poisson_rvs(mut r: Rng, mu: f64) -> u32 {
    unsafe { bindings::gsl_ran_poisson(&mut r, mu) }
}

/// These functions compute results for the unit Gaussian distribution.
/// They
/// are equivalent to the functions above with a standard deviation of one,
/// `sigma` = 1.
/// 
/// Binds the function [`gsl_ran_ugaussian`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_ugaussian).
pub fn ugaussian_rvs(mut r: Rng) -> f64 {
    unsafe { bindings::gsl_ran_ugaussian(&mut r) }
}

/// This function returns a random variate from the exponential distribution
/// with mean `mu`. The distribution is,
/// 
/// $$p(x) dx = {1 \over \mu} \exp(-x/\mu) dx$$
/// 
/// for $x \ge 0$.
/// 
/// Binds the function [`gsl_ran_exponential`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_exponential).
pub fn exponential_rvs(mut r: Rng, mu: f64) -> f64 {
    unsafe { bindings::gsl_ran_exponential(&mut r, mu) }
}

/// This function returns a random integer from the Pascal distribution.
/// The
/// Pascal distribution is simply a negative binomial distribution with an
/// integer value of $n$.
/// 
/// $$p(k) = {(n + k - 1)! \over k! (n - 1)! } p^n (1-p)^k$$
/// 
/// for $k \ge 0$.
/// 
/// Binds the function [`gsl_ran_pascal`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_pascal).
pub fn pascal_rvs(mut r: Rng, p: f64, n: u32) -> u32 {
    unsafe { bindings::gsl_ran_pascal(&mut r, p, n) }
}

/// This function returns a random variate from the F-distribution with degrees of freedom `nu1` and `nu2`.
/// The distribution function is,
/// 
/// $$p(x) dx =
///    { \Gamma((\nu_1 + \nu_2)/2)
///         \over \Gamma(\nu_1/2) \Gamma(\nu_2/2) }
///    \nu_1^{\nu_1/2} \nu_2^{\nu_2/2}
///    x^{\nu_1/2 - 1} (\nu_2 + \nu_1 x)^{-\nu_1/2 -\nu_2/2}$$
/// 
/// for $x \ge 0$.
/// 
/// Binds the function [`gsl_ran_fdist`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_fdist).
pub fn fdist_rvs(mut r: Rng, nu1: f64, nu2: f64) -> f64 {
    unsafe { bindings::gsl_ran_fdist(&mut r, nu1, nu2) }
}

/// This function computes a Gaussian random variate using the alternative
/// Marsaglia-Tsang ziggurat and Kinderman-Monahan-Leva ratio methods.
/// The
/// Ziggurat algorithm is the fastest available algorithm in most cases.
/// 
/// Binds the function [`gsl_ran_gaussian_ratio_method`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_gaussian_ratio_method).
pub fn gaussian_ratio_method_rvs(mut r: Rng, sigma: f64) -> f64 {
    unsafe { bindings::gsl_ran_gaussian_ratio_method(&mut r, sigma) }
}


/// This function computes the probability $p(k)$ of obtaining
/// `k` from a Bernoulli distribution with probability parameter
/// `p`.
/// 
/// ![_images/rand-bernoulli.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-bernoulli.png)
/// 
/// Binds the function [`gsl_ran_bernoulli_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_bernoulli_pdf).
pub fn bernoulli_pdf(k: u32, p: f64) -> f64 {
    unsafe { bindings::gsl_ran_bernoulli_pdf(k, p) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the beta
/// distribution with parameters `a` and `b`.
/// 
/// Binds the function [`gsl_cdf_beta_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_beta_P).
pub fn beta_P_cdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_beta_P(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the beta
/// distribution with parameters `a` and `b`.
/// 
/// Binds the function [`gsl_cdf_beta_Pinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_beta_Pinv).
pub fn beta_Pinv_cdf(p: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_beta_Pinv(p, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the beta
/// distribution with parameters `a` and `b`.
/// 
/// Binds the function [`gsl_cdf_beta_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_beta_Q).
pub fn beta_Q_cdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_beta_Q(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the beta
/// distribution with parameters `a` and `b`.
/// 
/// Binds the function [`gsl_cdf_beta_Qinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_beta_Qinv).
pub fn beta_Qinv_cdf(q: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_beta_Qinv(q, a, b) }
}

/// This function computes the probability density $p(x)$ at `x`
/// for a beta distribution with parameters `a` and `b`, using the
/// formula given above.
/// 
/// ![_images/rand-beta.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-beta.png)
/// 
/// Binds the function [`gsl_ran_beta_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_beta_pdf).
pub fn beta_pdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_ran_beta_pdf(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(k)$, $Q(k)$  for the binomial
/// distribution with parameters `p` and `n`.
/// 
/// Binds the function [`gsl_cdf_binomial_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_binomial_P).
pub fn binomial_P_cdf(k: u32, p: f64, n: u32) -> f64 {
    unsafe { bindings::gsl_cdf_binomial_P(k, p, n) }
}

/// These functions compute the cumulative distribution functions
/// $P(k)$, $Q(k)$  for the binomial
/// distribution with parameters `p` and `n`.
/// 
/// Binds the function [`gsl_cdf_binomial_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_binomial_Q).
pub fn binomial_Q_cdf(k: u32, p: f64, n: u32) -> f64 {
    unsafe { bindings::gsl_cdf_binomial_Q(k, p, n) }
}

/// This function computes the probability $p(k)$ of obtaining `k`
/// from a binomial distribution with parameters `p` and `n`, using
/// the formula given above.
/// 
/// ![_images/rand-binomial.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-binomial.png)
/// 
/// Binds the function [`gsl_ran_binomial_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_binomial_pdf).
pub fn binomial_pdf(k: u32, p: f64, n: u32) -> f64 {
    unsafe { bindings::gsl_ran_binomial_pdf(k, p, n) }
}

/// This function computes the probability density $p(x,y)$ at
/// (`x`, `y`) for a bivariate Gaussian distribution with standard
/// deviations `sigma_x`, `sigma_y` and correlation coefficient
/// `rho`.
/// 
/// ![_images/rand-bivariate-gaussian.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-bivariate-gaussian.png)
/// 
/// Binds the function [`gsl_ran_bivariate_gaussian_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_bivariate_gaussian_pdf).
pub fn bivariate_gaussian_pdf(x: f64, y: f64, sigma_x: f64, sigma_y: f64, rho: f64) -> f64 {
    unsafe { bindings::gsl_ran_bivariate_gaussian_pdf(x, y, sigma_x, sigma_y, rho) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Cauchy
/// distribution with scale parameter `a`.
/// 
/// Binds the function [`gsl_cdf_cauchy_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_cauchy_P).
pub fn cauchy_P_cdf(x: f64, a: f64) -> f64 {
    unsafe { bindings::gsl_cdf_cauchy_P(x, a) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Cauchy
/// distribution with scale parameter `a`.
/// 
/// Binds the function [`gsl_cdf_cauchy_Pinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_cauchy_Pinv).
pub fn cauchy_Pinv_cdf(p: f64, a: f64) -> f64 {
    unsafe { bindings::gsl_cdf_cauchy_Pinv(p, a) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Cauchy
/// distribution with scale parameter `a`.
/// 
/// Binds the function [`gsl_cdf_cauchy_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_cauchy_Q).
pub fn cauchy_Q_cdf(x: f64, a: f64) -> f64 {
    unsafe { bindings::gsl_cdf_cauchy_Q(x, a) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Cauchy
/// distribution with scale parameter `a`.
/// 
/// Binds the function [`gsl_cdf_cauchy_Qinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_cauchy_Qinv).
pub fn cauchy_Qinv_cdf(q: f64, a: f64) -> f64 {
    unsafe { bindings::gsl_cdf_cauchy_Qinv(q, a) }
}

/// This function computes the probability density $p(x)$ at `x`
/// for a Cauchy distribution with scale parameter `a`, using the formula
/// given above.
/// 
/// ![_images/rand-cauchy.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-cauchy.png)
/// 
/// Binds the function [`gsl_ran_cauchy_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_cauchy_pdf).
pub fn cauchy_pdf(x: f64, a: f64) -> f64 {
    unsafe { bindings::gsl_ran_cauchy_pdf(x, a) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the chi-squared
/// distribution with `nu` degrees of freedom.
/// 
/// Binds the function [`gsl_cdf_chisq_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_chisq_P).
pub fn chi2_P_cdf(x: f64, nu: f64) -> f64 {
    unsafe { bindings::gsl_cdf_chisq_P(x, nu) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the chi-squared
/// distribution with `nu` degrees of freedom.
/// 
/// Binds the function [`gsl_cdf_chisq_Pinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_chisq_Pinv).
pub fn chi2_Pinv_cdf(p: f64, nu: f64) -> f64 {
    unsafe { bindings::gsl_cdf_chisq_Pinv(p, nu) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the chi-squared
/// distribution with `nu` degrees of freedom.
/// 
/// Binds the function [`gsl_cdf_chisq_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_chisq_Q).
pub fn chi2_Q_cdf(x: f64, nu: f64) -> f64 {
    unsafe { bindings::gsl_cdf_chisq_Q(x, nu) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the chi-squared
/// distribution with `nu` degrees of freedom.
/// 
/// Binds the function [`gsl_cdf_chisq_Qinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_chisq_Qinv).
pub fn chi2_Qinv_cdf(q: f64, nu: f64) -> f64 {
    unsafe { bindings::gsl_cdf_chisq_Qinv(q, nu) }
}

/// This function computes the probability density $p(x)$ at `x`
/// for a chi-squared distribution with `nu` degrees of freedom, using
/// the formula given above.
/// 
/// ![_images/rand-chisq.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-chisq.png)
/// 
/// Binds the function [`gsl_ran_chisq_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_chisq_pdf).
pub fn chi2_pdf(x: f64, nu: f64) -> f64 {
    unsafe { bindings::gsl_ran_chisq_pdf(x, nu) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the exponential
/// distribution with mean `mu`.
/// 
/// Binds the function [`gsl_cdf_exponential_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_exponential_P).
pub fn exponential_P_cdf(x: f64, mu: f64) -> f64 {
    unsafe { bindings::gsl_cdf_exponential_P(x, mu) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the exponential
/// distribution with mean `mu`.
/// 
/// Binds the function [`gsl_cdf_exponential_Pinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_exponential_Pinv).
pub fn exponential_Pinv_cdf(p: f64, mu: f64) -> f64 {
    unsafe { bindings::gsl_cdf_exponential_Pinv(p, mu) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the exponential
/// distribution with mean `mu`.
/// 
/// Binds the function [`gsl_cdf_exponential_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_exponential_Q).
pub fn exponential_Q_cdf(x: f64, mu: f64) -> f64 {
    unsafe { bindings::gsl_cdf_exponential_Q(x, mu) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the exponential
/// distribution with mean `mu`.
/// 
/// Binds the function [`gsl_cdf_exponential_Qinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_exponential_Qinv).
pub fn exponential_Qinv_cdf(q: f64, mu: f64) -> f64 {
    unsafe { bindings::gsl_cdf_exponential_Qinv(q, mu) }
}

/// This function computes the probability density $p(x)$ at `x`
/// for an exponential distribution with mean `mu`, using the formula
/// given above.
/// 
/// ![_images/rand-exponential.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-exponential.png)
/// 
/// Binds the function [`gsl_ran_exponential_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_exponential_pdf).
pub fn exponential_pdf(x: f64, mu: f64) -> f64 {
    unsafe { bindings::gsl_ran_exponential_pdf(x, mu) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ for the exponential power distribution with
/// parameters `a` and `b`.
/// 
/// Binds the function [`gsl_cdf_exppow_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_exppow_P).
pub fn exppow_P_cdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_exppow_P(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ for the exponential power distribution with
/// parameters `a` and `b`.
/// 
/// Binds the function [`gsl_cdf_exppow_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_exppow_Q).
pub fn exppow_Q_cdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_exppow_Q(x, a, b) }
}

/// This function computes the probability density $p(x)$ at `x`
/// for an exponential power distribution with scale parameter `a`
/// and exponent `b`.
/// 
/// ![_images/rand-exppow.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-exppow.png)
/// 
/// Binds the function [`gsl_ran_exppow_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_exppow_pdf).
pub fn exppow_pdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_ran_exppow_pdf(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the F-distribution
/// with `nu1` and `nu2` degrees of freedom.
/// 
/// Binds the function [`gsl_cdf_fdist_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_fdist_P).
pub fn fdist_P_cdf(x: f64, nu1: f64, nu2: f64) -> f64 {
    unsafe { bindings::gsl_cdf_fdist_P(x, nu1, nu2) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the F-distribution
/// with `nu1` and `nu2` degrees of freedom.
/// 
/// Binds the function [`gsl_cdf_fdist_Pinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_fdist_Pinv).
pub fn fdist_Pinv_cdf(p: f64, nu1: f64, nu2: f64) -> f64 {
    unsafe { bindings::gsl_cdf_fdist_Pinv(p, nu1, nu2) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the F-distribution
/// with `nu1` and `nu2` degrees of freedom.
/// 
/// Binds the function [`gsl_cdf_fdist_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_fdist_Q).
pub fn fdist_Q_cdf(x: f64, nu1: f64, nu2: f64) -> f64 {
    unsafe { bindings::gsl_cdf_fdist_Q(x, nu1, nu2) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the F-distribution
/// with `nu1` and `nu2` degrees of freedom.
/// 
/// Binds the function [`gsl_cdf_fdist_Qinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_fdist_Qinv).
pub fn fdist_Qinv_cdf(q: f64, nu1: f64, nu2: f64) -> f64 {
    unsafe { bindings::gsl_cdf_fdist_Qinv(q, nu1, nu2) }
}

/// This function computes the probability density $p(x)$ at `x`
/// for an F-distribution with `nu1` and `nu2` degrees of freedom,
/// using the formula given above.
/// 
/// ![_images/rand-fdist.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-fdist.png)
/// 
/// Binds the function [`gsl_ran_fdist_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_fdist_pdf).
pub fn fdist_pdf(x: f64, nu1: f64, nu2: f64) -> f64 {
    unsafe { bindings::gsl_ran_fdist_pdf(x, nu1, nu2) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for a uniform distribution
/// from `a` to `b`.
/// 
/// Binds the function [`gsl_cdf_flat_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_flat_P).
pub fn flat_P_cdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_flat_P(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for a uniform distribution
/// from `a` to `b`.
/// 
/// Binds the function [`gsl_cdf_flat_Pinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_flat_Pinv).
pub fn flat_Pinv_cdf(p: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_flat_Pinv(p, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for a uniform distribution
/// from `a` to `b`.
/// 
/// Binds the function [`gsl_cdf_flat_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_flat_Q).
pub fn flat_Q_cdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_flat_Q(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for a uniform distribution
/// from `a` to `b`.
/// 
/// Binds the function [`gsl_cdf_flat_Qinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_flat_Qinv).
pub fn flat_Qinv_cdf(q: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_flat_Qinv(q, a, b) }
}

/// This function computes the probability density $p(x)$ at `x`
/// for a uniform distribution from `a` to `b`, using the formula
/// given above.
/// 
/// ![_images/rand-flat.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-flat.png)
/// 
/// Binds the function [`gsl_ran_flat_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_flat_pdf).
pub fn flat_pdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_ran_flat_pdf(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the gamma
/// distribution with parameters `a` and `b`.
/// 
/// Binds the function [`gsl_cdf_gamma_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_gamma_P).
pub fn gamma_P_cdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_gamma_P(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the gamma
/// distribution with parameters `a` and `b`.
/// 
/// Binds the function [`gsl_cdf_gamma_Pinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_gamma_Pinv).
pub fn gamma_Pinv_cdf(p: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_gamma_Pinv(p, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the gamma
/// distribution with parameters `a` and `b`.
/// 
/// Binds the function [`gsl_cdf_gamma_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_gamma_Q).
pub fn gamma_Q_cdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_gamma_Q(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the gamma
/// distribution with parameters `a` and `b`.
/// 
/// Binds the function [`gsl_cdf_gamma_Qinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_gamma_Qinv).
pub fn gamma_Qinv_cdf(q: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_gamma_Qinv(q, a, b) }
}

/// This function computes the probability density $p(x)$ at `x`
/// for a gamma distribution with parameters `a` and `b`, using the
/// formula given above.
/// 
/// ![_images/rand-gamma.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-gamma.png)
/// 
/// Binds the function [`gsl_ran_gamma_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_gamma_pdf).
pub fn gamma_pdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_ran_gamma_pdf(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Gaussian
/// distribution with standard deviation `sigma`.
/// 
/// Binds the function [`gsl_cdf_gaussian_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_gaussian_P).
pub fn gaussian_P_cdf(x: f64, sigma: f64) -> f64 {
    unsafe { bindings::gsl_cdf_gaussian_P(x, sigma) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Gaussian
/// distribution with standard deviation `sigma`.
/// 
/// Binds the function [`gsl_cdf_gaussian_Pinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_gaussian_Pinv).
pub fn gaussian_Pinv_cdf(p: f64, sigma: f64) -> f64 {
    unsafe { bindings::gsl_cdf_gaussian_Pinv(p, sigma) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Gaussian
/// distribution with standard deviation `sigma`.
/// 
/// Binds the function [`gsl_cdf_gaussian_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_gaussian_Q).
pub fn gaussian_Q_cdf(x: f64, sigma: f64) -> f64 {
    unsafe { bindings::gsl_cdf_gaussian_Q(x, sigma) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Gaussian
/// distribution with standard deviation `sigma`.
/// 
/// Binds the function [`gsl_cdf_gaussian_Qinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_gaussian_Qinv).
pub fn gaussian_Qinv_cdf(q: f64, sigma: f64) -> f64 {
    unsafe { bindings::gsl_cdf_gaussian_Qinv(q, sigma) }
}

/// This function computes the probability density $p(x)$ at `x`
/// for a Gaussian distribution with standard deviation `sigma`, using
/// the formula given above.
/// 
/// ![_images/rand-gaussian.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-gaussian.png)
/// 
/// Binds the function [`gsl_ran_gaussian_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_gaussian_pdf).
pub fn gaussian_pdf(x: f64, sigma: f64) -> f64 {
    unsafe { bindings::gsl_ran_gaussian_pdf(x, sigma) }
}

/// This function computes the probability density $p(x)$ at `x`
/// for a Gaussian tail distribution with standard deviation `sigma` and
/// lower limit `a`.
/// 
/// ![_images/rand-gaussian-tail.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-gaussian-tail.png)
/// 
/// Binds the function [`gsl_ran_gaussian_tail_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_gaussian_tail_pdf).
pub fn gaussian_tail_pdf(x: f64, a: f64, sigma: f64) -> f64 {
    unsafe { bindings::gsl_ran_gaussian_tail_pdf(x, a, sigma) }
}

/// These functions compute the cumulative distribution functions
/// $P(k)$, $Q(k)$ for the geometric distribution with parameter
/// `p`.
/// 
/// Binds the function [`gsl_cdf_geometric_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_geometric_P).
pub fn geometric_P_cdf(k: u32, p: f64) -> f64 {
    unsafe { bindings::gsl_cdf_geometric_P(k, p) }
}

/// These functions compute the cumulative distribution functions
/// $P(k)$, $Q(k)$ for the geometric distribution with parameter
/// `p`.
/// 
/// Binds the function [`gsl_cdf_geometric_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_geometric_Q).
pub fn geometric_Q_cdf(k: u32, p: f64) -> f64 {
    unsafe { bindings::gsl_cdf_geometric_Q(k, p) }
}

/// This function computes the probability $p(k)$ of obtaining `k`
/// from a geometric distribution with probability parameter `p`, using
/// the formula given above.
/// 
/// ![_images/rand-geometric.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-geometric.png)
/// 
/// Binds the function [`gsl_ran_geometric_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_geometric_pdf).
pub fn geometric_pdf(k: u32, p: f64) -> f64 {
    unsafe { bindings::gsl_ran_geometric_pdf(k, p) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Type-1 Gumbel
/// distribution with parameters `a` and `b`.
/// 
/// Binds the function [`gsl_cdf_gumbel1_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_gumbel1_P).
pub fn gumbel1_P_cdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_gumbel1_P(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Type-1 Gumbel
/// distribution with parameters `a` and `b`.
/// 
/// Binds the function [`gsl_cdf_gumbel1_Pinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_gumbel1_Pinv).
pub fn gumbel1_Pinv_cdf(p: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_gumbel1_Pinv(p, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Type-1 Gumbel
/// distribution with parameters `a` and `b`.
/// 
/// Binds the function [`gsl_cdf_gumbel1_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_gumbel1_Q).
pub fn gumbel1_Q_cdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_gumbel1_Q(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Type-1 Gumbel
/// distribution with parameters `a` and `b`.
/// 
/// Binds the function [`gsl_cdf_gumbel1_Qinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_gumbel1_Qinv).
pub fn gumbel1_Qinv_cdf(q: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_gumbel1_Qinv(q, a, b) }
}

/// This function computes the probability density $p(x)$ at `x`
/// for a Type-1 Gumbel distribution with parameters `a` and `b`,
/// using the formula given above.
/// 
/// ![_images/rand-gumbel1.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-gumbel1.png)
/// 
/// Binds the function [`gsl_ran_gumbel1_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_gumbel1_pdf).
pub fn gumbel1_pdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_ran_gumbel1_pdf(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Type-2 Gumbel
/// distribution with parameters `a` and `b`.
/// 
/// Binds the function [`gsl_cdf_gumbel2_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_gumbel2_P).
pub fn gumbel2_P_cdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_gumbel2_P(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Type-2 Gumbel
/// distribution with parameters `a` and `b`.
/// 
/// Binds the function [`gsl_cdf_gumbel2_Pinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_gumbel2_Pinv).
pub fn gumbel2_Pinv_cdf(p: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_gumbel2_Pinv(p, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Type-2 Gumbel
/// distribution with parameters `a` and `b`.
/// 
/// Binds the function [`gsl_cdf_gumbel2_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_gumbel2_Q).
pub fn gumbel2_Q_cdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_gumbel2_Q(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Type-2 Gumbel
/// distribution with parameters `a` and `b`.
/// 
/// Binds the function [`gsl_cdf_gumbel2_Qinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_gumbel2_Qinv).
pub fn gumbel2_Qinv_cdf(q: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_gumbel2_Qinv(q, a, b) }
}

/// This function computes the probability density $p(x)$ at `x`
/// for a Type-2 Gumbel distribution with parameters `a` and `b`,
/// using the formula given above.
/// 
/// ![_images/rand-gumbel2.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-gumbel2.png)
/// 
/// Binds the function [`gsl_ran_gumbel2_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_gumbel2_pdf).
pub fn gumbel2_pdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_ran_gumbel2_pdf(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(k)$, $Q(k)$ for the hypergeometric distribution with
/// parameters `n1`, `n2` and `t`.
/// 
/// Binds the function [`gsl_cdf_hypergeometric_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_hypergeometric_P).
pub fn hypergeometric_P_cdf(k: u32, n1: u32, n2: u32, t: u32) -> f64 {
    unsafe { bindings::gsl_cdf_hypergeometric_P(k, n1, n2, t) }
}

/// These functions compute the cumulative distribution functions
/// $P(k)$, $Q(k)$ for the hypergeometric distribution with
/// parameters `n1`, `n2` and `t`.
/// 
/// Binds the function [`gsl_cdf_hypergeometric_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_hypergeometric_Q).
pub fn hypergeometric_Q_cdf(k: u32, n1: u32, n2: u32, t: u32) -> f64 {
    unsafe { bindings::gsl_cdf_hypergeometric_Q(k, n1, n2, t) }
}

/// This function computes the probability $p(k)$ of obtaining `k`
/// from a hypergeometric distribution with parameters `n1`, `n2`,
/// `t`.
/// 
/// ![_images/rand-hypergeometric.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-hypergeometric.png)
/// 
/// Binds the function [`gsl_ran_hypergeometric_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_hypergeometric_pdf).
pub fn hypergeometric_pdf(k: u32, n1: u32, n2: u32, t: u32) -> f64 {
    unsafe { bindings::gsl_ran_hypergeometric_pdf(k, n1, n2, t) }
}

/// This function computes the probability density $p(x)$ at `x`
/// for the Landau distribution using an approximation to the formula given
/// above.
/// 
/// ![_images/rand-landau.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-landau.png)
/// 
/// Binds the function [`gsl_ran_landau_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_landau_pdf).
pub fn landau_pdf(x: f64) -> f64 {
    unsafe { bindings::gsl_ran_landau_pdf(x) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Laplace
/// distribution with width `a`.
/// 
/// Binds the function [`gsl_cdf_laplace_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_laplace_P).
pub fn laplace_P_cdf(x: f64, a: f64) -> f64 {
    unsafe { bindings::gsl_cdf_laplace_P(x, a) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Laplace
/// distribution with width `a`.
/// 
/// Binds the function [`gsl_cdf_laplace_Pinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_laplace_Pinv).
pub fn laplace_Pinv_cdf(p: f64, a: f64) -> f64 {
    unsafe { bindings::gsl_cdf_laplace_Pinv(p, a) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Laplace
/// distribution with width `a`.
/// 
/// Binds the function [`gsl_cdf_laplace_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_laplace_Q).
pub fn laplace_Q_cdf(x: f64, a: f64) -> f64 {
    unsafe { bindings::gsl_cdf_laplace_Q(x, a) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Laplace
/// distribution with width `a`.
/// 
/// Binds the function [`gsl_cdf_laplace_Qinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_laplace_Qinv).
pub fn laplace_Qinv_cdf(q: f64, a: f64) -> f64 {
    unsafe { bindings::gsl_cdf_laplace_Qinv(q, a) }
}

/// This function computes the probability density $p(x)$ at `x`
/// for a Laplace distribution with width `a`, using the formula
/// given above.
/// 
/// ![_images/rand-laplace.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-laplace.png)
/// 
/// Binds the function [`gsl_ran_laplace_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_laplace_pdf).
pub fn laplace_pdf(x: f64, a: f64) -> f64 {
    unsafe { bindings::gsl_ran_laplace_pdf(x, a) }
}

/// This function computes the probability $p(k)$ of obtaining `k`
/// from a logarithmic distribution with probability parameter `p`,
/// using the formula given above.
/// 
/// ![_images/rand-logarithmic.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-logarithmic.png)
/// 
/// Binds the function [`gsl_ran_logarithmic_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_logarithmic_pdf).
pub fn logarithmic_pdf(k: u32, p: f64) -> f64 {
    unsafe { bindings::gsl_ran_logarithmic_pdf(k, p) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the logistic
/// distribution with scale parameter `a`.
/// 
/// Binds the function [`gsl_cdf_logistic_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_logistic_P).
pub fn logistic_P_cdf(x: f64, a: f64) -> f64 {
    unsafe { bindings::gsl_cdf_logistic_P(x, a) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the logistic
/// distribution with scale parameter `a`.
/// 
/// Binds the function [`gsl_cdf_logistic_Pinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_logistic_Pinv).
pub fn logistic_Pinv_cdf(p: f64, a: f64) -> f64 {
    unsafe { bindings::gsl_cdf_logistic_Pinv(p, a) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the logistic
/// distribution with scale parameter `a`.
/// 
/// Binds the function [`gsl_cdf_logistic_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_logistic_Q).
pub fn logistic_Q_cdf(x: f64, a: f64) -> f64 {
    unsafe { bindings::gsl_cdf_logistic_Q(x, a) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the logistic
/// distribution with scale parameter `a`.
/// 
/// Binds the function [`gsl_cdf_logistic_Qinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_logistic_Qinv).
pub fn logistic_Qinv_cdf(q: f64, a: f64) -> f64 {
    unsafe { bindings::gsl_cdf_logistic_Qinv(q, a) }
}

/// This function computes the probability density $p(x)$ at `x`
/// for a logistic distribution with scale parameter `a`, using the
/// formula given above.
/// 
/// ![_images/rand-logistic.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-logistic.png)
/// 
/// Binds the function [`gsl_ran_logistic_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_logistic_pdf).
pub fn logistic_pdf(x: f64, a: f64) -> f64 {
    unsafe { bindings::gsl_ran_logistic_pdf(x, a) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the lognormal
/// distribution with parameters `zeta` and `sigma`.
/// 
/// Binds the function [`gsl_cdf_lognormal_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_lognormal_P).
pub fn lognormal_P_cdf(x: f64, zeta: f64, sigma: f64) -> f64 {
    unsafe { bindings::gsl_cdf_lognormal_P(x, zeta, sigma) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the lognormal
/// distribution with parameters `zeta` and `sigma`.
/// 
/// Binds the function [`gsl_cdf_lognormal_Pinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_lognormal_Pinv).
pub fn lognormal_Pinv_cdf(p: f64, zeta: f64, sigma: f64) -> f64 {
    unsafe { bindings::gsl_cdf_lognormal_Pinv(p, zeta, sigma) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the lognormal
/// distribution with parameters `zeta` and `sigma`.
/// 
/// Binds the function [`gsl_cdf_lognormal_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_lognormal_Q).
pub fn lognormal_Q_cdf(x: f64, zeta: f64, sigma: f64) -> f64 {
    unsafe { bindings::gsl_cdf_lognormal_Q(x, zeta, sigma) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the lognormal
/// distribution with parameters `zeta` and `sigma`.
/// 
/// Binds the function [`gsl_cdf_lognormal_Qinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_lognormal_Qinv).
pub fn lognormal_Qinv_cdf(q: f64, zeta: f64, sigma: f64) -> f64 {
    unsafe { bindings::gsl_cdf_lognormal_Qinv(q, zeta, sigma) }
}

/// This function computes the probability density $p(x)$ at `x`
/// for a lognormal distribution with parameters `zeta` and `sigma`,
/// using the formula given above.
/// 
/// ![_images/rand-lognormal.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-lognormal.png)
/// 
/// Binds the function [`gsl_ran_lognormal_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_lognormal_pdf).
pub fn lognormal_pdf(x: f64, zeta: f64, sigma: f64) -> f64 {
    unsafe { bindings::gsl_ran_lognormal_pdf(x, zeta, sigma) }
}

/// These functions compute the cumulative distribution functions
/// $P(k)$, $Q(k)$ for the negative binomial distribution with
/// parameters `p` and `n`.
/// 
/// Binds the function [`gsl_cdf_negative_binomial_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_negative_binomial_P).
pub fn negative_binomial_P_cdf(k: u32, p: f64, n: f64) -> f64 {
    unsafe { bindings::gsl_cdf_negative_binomial_P(k, p, n) }
}

/// These functions compute the cumulative distribution functions
/// $P(k)$, $Q(k)$ for the negative binomial distribution with
/// parameters `p` and `n`.
/// 
/// Binds the function [`gsl_cdf_negative_binomial_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_negative_binomial_Q).
pub fn negative_binomial_Q_cdf(k: u32, p: f64, n: f64) -> f64 {
    unsafe { bindings::gsl_cdf_negative_binomial_Q(k, p, n) }
}

/// This function computes the probability $p(k)$ of obtaining `k`
/// from a negative binomial distribution with parameters `p` and
/// `n`.
/// 
/// ![_images/rand-nbinomial.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-nbinomial.png)
/// 
/// Binds the function [`gsl_ran_negative_binomial_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_negative_binomial_pdf).
pub fn negative_binomial_pdf(k: u32, p: f64, n: f64) -> f64 {
    unsafe { bindings::gsl_ran_negative_binomial_pdf(k, p, n) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Pareto
/// distribution with exponent `a` and scale `b`.
/// 
/// Binds the function [`gsl_cdf_pareto_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_pareto_P).
pub fn pareto_P_cdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_pareto_P(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Pareto
/// distribution with exponent `a` and scale `b`.
/// 
/// Binds the function [`gsl_cdf_pareto_Pinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_pareto_Pinv).
pub fn pareto_Pinv_cdf(p: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_pareto_Pinv(p, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Pareto
/// distribution with exponent `a` and scale `b`.
/// 
/// Binds the function [`gsl_cdf_pareto_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_pareto_Q).
pub fn pareto_Q_cdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_pareto_Q(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Pareto
/// distribution with exponent `a` and scale `b`.
/// 
/// Binds the function [`gsl_cdf_pareto_Qinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_pareto_Qinv).
pub fn pareto_Qinv_cdf(q: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_pareto_Qinv(q, a, b) }
}

/// This function computes the probability density $p(x)$ at `x`
/// for a Pareto distribution with exponent `a` and scale `b`, using
/// the formula given above.
/// 
/// ![_images/rand-pareto.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-pareto.png)
/// 
/// Binds the function [`gsl_ran_pareto_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_pareto_pdf).
pub fn pareto_pdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_ran_pareto_pdf(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(k)$, $Q(k)$ for the Pascal distribution with
/// parameters `p` and `n`.
/// 
/// Binds the function [`gsl_cdf_pascal_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_pascal_P).
pub fn pascal_P_cdf(k: u32, p: f64, n: u32) -> f64 {
    unsafe { bindings::gsl_cdf_pascal_P(k, p, n) }
}

/// These functions compute the cumulative distribution functions
/// $P(k)$, $Q(k)$ for the Pascal distribution with
/// parameters `p` and `n`.
/// 
/// Binds the function [`gsl_cdf_pascal_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_pascal_Q).
pub fn pascal_Q_cdf(k: u32, p: f64, n: u32) -> f64 {
    unsafe { bindings::gsl_cdf_pascal_Q(k, p, n) }
}

/// This function computes the probability $p(k)$ of obtaining `k`
/// from a Pascal distribution with parameters `p` and
/// `n`.
/// 
/// ![_images/rand-pascal.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-pascal.png)
/// 
/// Binds the function [`gsl_ran_pascal_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_pascal_pdf).
pub fn pascal_pdf(k: u32, p: f64, n: u32) -> f64 {
    unsafe { bindings::gsl_ran_pascal_pdf(k, p, n) }
}

/// These functions compute the cumulative distribution functions
/// $P(k)$, $Q(k)$ for the Poisson distribution with parameter
/// `mu`.
/// 
/// Binds the function [`gsl_cdf_poisson_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_poisson_P).
pub fn poisson_P_cdf(k: u32, mu: f64) -> f64 {
    unsafe { bindings::gsl_cdf_poisson_P(k, mu) }
}

/// These functions compute the cumulative distribution functions
/// $P(k)$, $Q(k)$ for the Poisson distribution with parameter
/// `mu`.
/// 
/// Binds the function [`gsl_cdf_poisson_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_poisson_Q).
pub fn poisson_Q_cdf(k: u32, mu: f64) -> f64 {
    unsafe { bindings::gsl_cdf_poisson_Q(k, mu) }
}

/// This function computes the probability $p(k)$ of obtaining  `k`
/// from a Poisson distribution with mean `mu`, using the formula
/// given above.
/// 
/// ![_images/rand-poisson.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-poisson.png)
/// 
/// Binds the function [`gsl_ran_poisson_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_poisson_pdf).
pub fn poisson_pdf(k: u32, mu: f64) -> f64 {
    unsafe { bindings::gsl_ran_poisson_pdf(k, mu) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Rayleigh
/// distribution with scale parameter `sigma`.
/// 
/// Binds the function [`gsl_cdf_rayleigh_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_rayleigh_P).
pub fn rayleigh_P_cdf(x: f64, sigma: f64) -> f64 {
    unsafe { bindings::gsl_cdf_rayleigh_P(x, sigma) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Rayleigh
/// distribution with scale parameter `sigma`.
/// 
/// Binds the function [`gsl_cdf_rayleigh_Pinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_rayleigh_Pinv).
pub fn rayleigh_Pinv_cdf(p: f64, sigma: f64) -> f64 {
    unsafe { bindings::gsl_cdf_rayleigh_Pinv(p, sigma) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Rayleigh
/// distribution with scale parameter `sigma`.
/// 
/// Binds the function [`gsl_cdf_rayleigh_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_rayleigh_Q).
pub fn rayleigh_Q_cdf(x: f64, sigma: f64) -> f64 {
    unsafe { bindings::gsl_cdf_rayleigh_Q(x, sigma) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Rayleigh
/// distribution with scale parameter `sigma`.
/// 
/// Binds the function [`gsl_cdf_rayleigh_Qinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_rayleigh_Qinv).
pub fn rayleigh_Qinv_cdf(q: f64, sigma: f64) -> f64 {
    unsafe { bindings::gsl_cdf_rayleigh_Qinv(q, sigma) }
}

/// This function computes the probability density $p(x)$ at `x`
/// for a Rayleigh distribution with scale parameter `sigma`, using the
/// formula given above.
/// 
/// ![_images/rand-rayleigh.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-rayleigh.png)
/// 
/// Binds the function [`gsl_ran_rayleigh_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_rayleigh_pdf).
pub fn rayleigh_pdf(x: f64, sigma: f64) -> f64 {
    unsafe { bindings::gsl_ran_rayleigh_pdf(x, sigma) }
}

/// This function computes the probability density $p(x)$ at `x`
/// for a Rayleigh tail distribution with scale parameter `sigma` and
/// lower limit `a`.
/// 
/// ![_images/rand-rayleigh-tail.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-rayleigh-tail.png)
/// 
/// Binds the function [`gsl_ran_rayleigh_tail_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_rayleigh_tail_pdf).
pub fn rayleigh_tail_pdf(x: f64, a: f64, sigma: f64) -> f64 {
    unsafe { bindings::gsl_ran_rayleigh_tail_pdf(x, a, sigma) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the t-distribution
/// with `nu` degrees of freedom.
/// 
/// Binds the function [`gsl_cdf_tdist_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_tdist_P).
pub fn student_t_P_cdf(x: f64, nu: f64) -> f64 {
    unsafe { bindings::gsl_cdf_tdist_P(x, nu) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the t-distribution
/// with `nu` degrees of freedom.
/// 
/// Binds the function [`gsl_cdf_tdist_Pinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_tdist_Pinv).
pub fn student_t_Pinv_cdf(p: f64, nu: f64) -> f64 {
    unsafe { bindings::gsl_cdf_tdist_Pinv(p, nu) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the t-distribution
/// with `nu` degrees of freedom.
/// 
/// Binds the function [`gsl_cdf_tdist_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_tdist_Q).
pub fn student_t_Q_cdf(x: f64, nu: f64) -> f64 {
    unsafe { bindings::gsl_cdf_tdist_Q(x, nu) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the t-distribution
/// with `nu` degrees of freedom.
/// 
/// Binds the function [`gsl_cdf_tdist_Qinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_tdist_Qinv).
pub fn student_t_Qinv_cdf(q: f64, nu: f64) -> f64 {
    unsafe { bindings::gsl_cdf_tdist_Qinv(q, nu) }
}

/// This function computes the probability density $p(x)$ at `x`
/// for a t-distribution with `nu` degrees of freedom, using the formula
/// given above.
/// 
/// ![_images/rand-tdist.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-tdist.png)
/// 
/// Binds the function [`gsl_ran_tdist_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_tdist_pdf).
pub fn student_t_pdf(x: f64, nu: f64) -> f64 {
    unsafe { bindings::gsl_ran_tdist_pdf(x, nu) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the unit Gaussian
/// distribution.
/// 
/// Binds the function [`gsl_cdf_ugaussian_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_ugaussian_P).
pub fn ugaussian_P_cdf(x: f64) -> f64 {
    unsafe { bindings::gsl_cdf_ugaussian_P(x) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the unit Gaussian
/// distribution.
/// 
/// Binds the function [`gsl_cdf_ugaussian_Pinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_ugaussian_Pinv).
pub fn ugaussian_Pinv_cdf(p: f64) -> f64 {
    unsafe { bindings::gsl_cdf_ugaussian_Pinv(p) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the unit Gaussian
/// distribution.
/// 
/// Binds the function [`gsl_cdf_ugaussian_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_ugaussian_Q).
pub fn ugaussian_Q_cdf(x: f64) -> f64 {
    unsafe { bindings::gsl_cdf_ugaussian_Q(x) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the unit Gaussian
/// distribution.
/// 
/// Binds the function [`gsl_cdf_ugaussian_Qinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_ugaussian_Qinv).
pub fn ugaussian_Qinv_cdf(q: f64) -> f64 {
    unsafe { bindings::gsl_cdf_ugaussian_Qinv(q) }
}

/// These functions compute results for the unit Gaussian distribution.
/// They
/// are equivalent to the functions above with a standard deviation of one,
/// `sigma` = 1.
/// 
/// Binds the function [`gsl_ran_ugaussian_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_ugaussian_pdf).
pub fn ugaussian_pdf(x: f64) -> f64 {
    unsafe { bindings::gsl_ran_ugaussian_pdf(x) }
}

/// These functions compute results for the tail of a unit Gaussian
/// distribution.
/// They are equivalent to the functions above with a standard
/// deviation of one, `sigma` = 1.
/// 
/// Binds the function [`gsl_ran_ugaussian_tail_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_ugaussian_tail_pdf).
pub fn ugaussian_tail_pdf(x: f64, a: f64) -> f64 {
    unsafe { bindings::gsl_ran_ugaussian_tail_pdf(x, a) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Weibull
/// distribution with scale `a` and exponent `b`.
/// 
/// Binds the function [`gsl_cdf_weibull_P`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_weibull_P).
pub fn weibull_P_cdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_weibull_P(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Weibull
/// distribution with scale `a` and exponent `b`.
/// 
/// Binds the function [`gsl_cdf_weibull_Pinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_weibull_Pinv).
pub fn weibull_Pinv_cdf(p: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_weibull_Pinv(p, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Weibull
/// distribution with scale `a` and exponent `b`.
/// 
/// Binds the function [`gsl_cdf_weibull_Q`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_weibull_Q).
pub fn weibull_Q_cdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_weibull_Q(x, a, b) }
}

/// These functions compute the cumulative distribution functions
/// $P(x)$, $Q(x)$ and their inverses for the Weibull
/// distribution with scale `a` and exponent `b`.
/// 
/// Binds the function [`gsl_cdf_weibull_Qinv`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_cdf_weibull_Qinv).
pub fn weibull_Qinv_cdf(q: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_cdf_weibull_Qinv(q, a, b) }
}

/// This function computes the probability density $p(x)$ at `x`
/// for a Weibull distribution with scale `a` and exponent `b`,
/// using the formula given above.
/// 
/// ![_images/rand-weibull.png](https://www.gnu.org/software/gsl/doc/html/_images/rand-weibull.png)
/// 
/// Binds the function [`gsl_ran_weibull_pdf`](https://www.gnu.org/software/gsl/doc/html//randist.html#c.gsl_ran_weibull_pdf).
pub fn weibull_pdf(x: f64, a: f64, b: f64) -> f64 {
    unsafe { bindings::gsl_ran_weibull_pdf(x, a, b) }
}


#[cfg(test)]
mod test {
    use crate::rng;
    use crate::distribution;
    use crate::test_helpers;

    #[test]
    fn test_ugaussian() {
        test_helpers::assert_moments!(
            |rng: rng::Rng| { distribution::ugaussian_rvs(rng) },
            0.0,
            100.0,
            p = 0.5
        );

        test_helpers::assert_moments!(
            |rng: rng::Rng| { distribution::ugaussian_rvs(rng) },
            -1.0,
            1.0,
            p = 0.6826895
        );
    }

    #[test]
    fn test_exponential() {
        test_helpers::assert_moments!(
            |rng: rng::Rng| { distribution::exponential_rvs(rng, 2.0) },
            0.0,
            1.0,
            p = 1.0 - (-0.5 as f64).exp()
        );
    }

    #[test]
    fn test_cauchy() {
        test_helpers::assert_moments!(
            |rng: rng::Rng| { distribution::cauchy_rvs(rng, 1.0) },
            0.0, 10000.0,
            p = 0.5
        );
        
        test_helpers::assert_moments!(
            |rng: rng::Rng| { distribution::cauchy_rvs(rng, 2.5) },
            0.0, 10000.0,
            p = 0.5
        );

        test_helpers::assert_moments!(
            |rng: rng::Rng| { distribution::cauchy_rvs(rng, 4.0) },
            0.0, 10000.0,
            p = 0.5
        );

    }
}