/*
    special/lambert.rs
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
<%= if @sf_e_functions == [] and  @sf_normal_functions == [] do %><% else %>
use crate::bindings;
use crate::{GSLError, ValWithError, Result};
<%= for sf_e <- @sf_e_functions do %>
<%= RSci.doc_to_rust(sf_e.doc) %>
pub fn <%= sf_e.rust_name %>(<%= for {arg, last?} <- RSci.with_last(sf_e.rust_arguments) do
            %><%= arg.name %>: <%= arg.type %><%= if not last? do %>, <% end %><% end
            %>) -> <%= sf_e.rust_return_type %> {
    unsafe {
        let mut result = bindings::gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw(bindings::<%= sf_e.c_name %>(<%= for arg <- sf_e.rust_arguments do
            %><%= arg.name %>, <% end %>&mut result))?;
        Ok(result.into())
    }
}
<% end %>
<%= for sf <- @sf_normal_functions do %>
<%= RSci.doc_to_rust(sf.doc) %>
pub fn <%= sf.rust_name %>(<%= for {arg, last?} <- RSci.with_last(sf.rust_arguments) do
            %><%= arg.name %>: <%= arg.type %><%= if not last? do %>, <% end %><% end
            %>) -> <%= sf.rust_return_type %> {
    unsafe { bindings::<%= sf.c_name %>(<%= for {arg, last?} <- RSci.with_last(sf.rust_arguments) do
             %><%= arg.name %><%= if not last? do %>, <% end %><% end %>) }
}
<% end %><% end %>
#[cfg(test)]<%= if @sf_e_tests != %{} do %>
mod test {
    // Import the special functions so that we can refer to the directly
    use crate::special::*;
    // Import some macro utilities to make our tests easier to write
    use crate::special::special_function_test::*;
<%= for {rust_function_name, sf_tests} <- @sf_e_tests do %>
    #[test]
    fn test_<%= rust_function_name %>() {<%= for sf_test <- sf_tests do %>
        check_result!(<%= rust_function_name %>(<%=
              RSci.SfTest.commas(sf_test.args) %>), <%=
              RSci.SfTest.serialize(sf_test.expected) %>, <%=
              RSci.SfTest.serialize(sf_test.tolerance) %>);<% end %>
    }
<% end %>}<% else %>
mod test {}<% end %>