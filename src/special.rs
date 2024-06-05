/*
    special.rs
    Copyright (C) 2021 Pim van den Berg

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

pub mod special_function_test;

mod gamma;
mod coulomb;
mod airy;
mod bessel;
mod clausen;
mod coupling;
mod dawson;
mod debye;
mod dilog;
mod elementary;
mod ellint;
mod elljac;
mod erf;
mod exp;
mod expint;
mod fermi_dirac;
mod gegenbauer;
mod hermite;
mod hyperg;
mod laguerre;
mod lambert;
mod legendre;
mod log;
mod mathieu;
mod pow_int;
mod psi;
mod sincos_pi;
mod synchrotron;
mod transport;
mod trig;
mod zeta;


pub use gamma::*;
pub use coulomb::*;
pub use airy::*;
pub use bessel::*;
pub use clausen::*;
pub use coupling::*;
pub use dawson::*;
pub use debye::*;
pub use dilog::*;
pub use elementary::*;
pub use ellint::*;
pub use elljac::*;
pub use erf::*;
pub use exp::*;
pub use expint::*;
pub use fermi_dirac::*;
pub use gegenbauer::*;
pub use hermite::*;
pub use hyperg::*;
pub use laguerre::*;
pub use lambert::*;
pub use legendre::*;
pub use log::*;
pub use mathieu::*;
pub use pow_int::*;
pub use psi::*;
pub use sincos_pi::*;
pub use synchrotron::*;
pub use transport::*;
pub use trig::*;
pub use zeta::*;
