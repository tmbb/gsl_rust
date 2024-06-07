{% if sf_test_groups == [] -%}
#[cfg(test)]
mod test {}
{% else -%}
#[cfg(test)]
mod test {
    use super::*;
    use crate::special::special_function_test::*;
    {%- for (key, sf_tests) in sf_test_groups %}
    
    #[test]
    {% if key.lower() != key %}#[allow(non_snake_case)]
    {% endif -%}
    fn test_{{key}}() {
        disable_error_handler();
        {%- for sf_test in sf_tests %}

        check_result(
            {{ sf_test.func_rust }}({{ sf_test.args | join(', ') }}),
            {{ sf_test.expected }},
            {{ sf_test.tolerance }}
        );
        {%- endfor %}
    }
    {%- endfor %}
}
{% endif %}