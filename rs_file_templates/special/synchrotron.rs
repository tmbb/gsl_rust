/*
    special/synchrotron.rs
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

{% if sf_heads == [] %}{% else %}
use crate::bindings::*;
use crate::*;

{% for head in sf_heads %}
{% if head.rust_func.lower() != head.rust_func %}#[allow(non_snake_case)]
{% endif -%}
pub fn {{ head.rust_func }}({% for arg in head.args if arg.type != "gsl_sf_result*"  %}{{ arg.name }}: {{ arg.type }}{% if not loop.last %}, {% endif %}{% endfor %}) -> Result<ValWithError<f64>> {
    unsafe {
        let mut result = gsl_sf_result { val: 0.0, err: 0.0 };
        GSLError::from_raw({{ head.c_func }}({% for arg in head.args if arg.type != "gsl_sf_result*" %}{{ arg.name }}, {% endfor %}&mut result))?;
        Ok(result.into())
    }
}
{% endfor %}{% endif %}

#[cfg(test)]
mod test {}