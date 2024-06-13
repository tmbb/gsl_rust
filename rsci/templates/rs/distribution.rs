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
// Import the FFI trait so that we can call `r.unwrap_unique()`
use crate::ffi::FFI;
// Import the ndarray functionality to have somewhere to store the results
// of multiple samplings and the results of a single sample of distributions
// that generate more than one `f64` or `i32`
use ndarray::{Array, Ix1};
<%= for ran_fun <- @ran_functions do %>
<%= RSci.doc_to_rust(ran_fun.doc) %>
pub fn <%= ran_fun.rust_name %>(r: &mut Rng<%= for arg <- ran_fun.rust_arguments do
        %>, <%= arg.name %>: <%= arg.type %><% end%>) -> <%= ran_fun.rust_return_type %> {
    unsafe { bindings::<%= ran_fun.c_name %>(r.unwrap_unique()<%= for arg <- ran_fun.rust_arguments do
        %>, <%= arg.name %><% end%>) }
}
<% end %>
<%= for normal_fun <- @normal_functions do %>
<%= RSci.doc_to_rust(normal_fun.doc) %>
pub fn <%= normal_fun.rust_name %>(<%= for {arg, last?} <- RSci.with_last(normal_fun.rust_arguments) do
        %><%= arg.name %>: <%= arg.type %><%= if not last? do %>, <% end %><% end%>) -> <%= normal_fun.rust_return_type %> {
    unsafe { bindings::<%= normal_fun.c_name %>(<%= for {arg, last?} <- RSci.with_last(normal_fun.rust_arguments) do
        %><%= arg.name %><%= if not last? do %>, <% end %><% end%>) }
}
<% end %>

// Functions to draw an array of random samples
<%= for ran_many_fun <- @ran_many_functions do %>
<%= RSci.doc_to_rust(ran_many_fun.doc) %>
pub fn draw_samples_from_<%= ran_many_fun.rust_name %>(r: &mut Rng<%=
        for arg <- ran_many_fun.rust_arguments do
            %>, <%= arg.name %>: <%= arg.type %><% end
            %>, <%= RSci.RandDist.nr_of_samples_var_name %>: usize) -> Array<<%= ran_many_fun.rust_return_type %>, Ix1> {
    Array::from_iter(
        (0..<%= RSci.RandDist.nr_of_samples_var_name %>).map(|_i| draw_sample_from_<%= ran_many_fun.rust_name %>(r<%=
            for arg <- ran_many_fun.rust_arguments do
                %>, <%= arg.name %><% end%>))
    )
}
<% end %>

// #[cfg(test)]
// mod test {
//     use crate::rng;
//     use crate::distribution;
//     use crate::test_helpers;

//     #[test]
//     fn test_ugaussian() {
//         test_helpers::assert_moments!(
//             |rng: rng::Rng| { distribution::draw_sample_from_ugaussian(rng) },
//             0.0,
//             100.0,
//             p = 0.5
//         );

//         test_helpers::assert_moments!(
//             |rng: rng::Rng| { distribution::draw_sample_from_ugaussian(rng) },
//             -1.0,
//             1.0,
//             p = 0.6826895
//         );
//     }

//     #[test]
//     fn test_exponential() {
//         test_helpers::assert_moments!(
//             |rng: rng::Rng| { distribution::draw_sample_from_exponential(rng, 2.0) },
//             0.0,
//             1.0,
//             p = 1.0 - (-0.5 as f64).exp()
//         );
//     }

//     #[test]
//     fn test_cauchy() {
//         test_helpers::assert_moments!(
//             |rng: rng::Rng| { distribution::draw_sample_from_cauchy(rng, 1.0) },
//             0.0, 10000.0,
//             p = 0.5
//         );
        
//         test_helpers::assert_moments!(
//             |rng: rng::Rng| { distribution::draw_sample_from_cauchy(rng, 2.5) },
//             0.0, 10000.0,
//             p = 0.5
//         );

//         test_helpers::assert_moments!(
//             |rng: rng::Rng| { distribution::draw_sample_from_cauchy(rng, 4.0) },
//             0.0, 10000.0,
//             p = 0.5
//         );

//     }
// }