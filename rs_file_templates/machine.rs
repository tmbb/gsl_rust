/// Magic constants
{% for cmacro in cmacros %}
pub const {{cmacro.name}}: {{cmacro.type}} = {{cmacro.value}};
{% endfor %}