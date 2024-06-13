/*
    lib.rs : gsl_rust. A small, safe Rust wrapper around the GSL.
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


#[allow(clippy::upper_case_acronyms)]
pub trait FFI<T> {
    fn wrap(r: *mut T) -> Self;
    fn soft_wrap(r: *mut T) -> Self;
    fn unwrap_shared(&self) -> *const T;
    fn unwrap_unique(&mut self) -> *mut T;
}