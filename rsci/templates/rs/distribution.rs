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
<%= for ran_fun <- @ran_functions do %>
<%= RSci.doc_to_rust(ran_fun.doc) %>
pub fn <%= ran_fun.rust_name %>(mut r: Rng<%= for arg <- ran_fun.rust_arguments do
        %>, <%= arg.name %>: <%= arg.type %><% end%>) -> <%= ran_fun.rust_return_type %> {
    unsafe { bindings::<%= ran_fun.c_name %>(&mut r<%= for arg <- ran_fun.rust_arguments do
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