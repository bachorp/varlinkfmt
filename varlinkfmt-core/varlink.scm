(interface
    _ @allow_blank_line_before @prepend_hardline
)

(interface
    (_ (name) @append_space)
)

[
    (keyword_interface)
    (keyword_type)
    (keyword_error)
    (keyword_method)
    ":"
] @append_space

[
    (arrow)
] @prepend_space @append_space

[
    (comment)
] @prepend_input_softline @append_hardline @allow_blank_line_before

(
    "(" @append_begin_scope @append_empty_softline @append_indent_start
    ")" @append_end_scope @prepend_empty_softline @prepend_indent_end

    (#scope_id! "parenthesized")
)

(
    "," @append_spaced_scoped_softline
    .
    (comment)? @do_nothing

    (#scope_id! "parenthesized")
)
