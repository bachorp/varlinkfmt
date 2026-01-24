#[derive(Clone, Copy, Debug)]
pub enum Indent {
    Tab,
    Spaces(usize),
}

pub fn mk_language(indent: Indent) -> topiary_core::Language {
    let grammar = tree_sitter_varlink::LANGUAGE.into();
    let indent = match indent {
        Indent::Tab => "\t".to_owned(),
        Indent::Spaces(n) => " ".repeat(n),
    };
    topiary_core::Language {
        name: "varlink".to_owned(),
        query: topiary_core::TopiaryQuery::new(&grammar, include_str!("../varlink.scm")).unwrap(),
        grammar,
        indent: Some(indent),
    }
}

// Ensure blank line between interface declaration/typedef/error/method. Cannot be done using Topiary.
fn space_members(formatted: &String) -> String {
    let mut lines_rev = Vec::<&str>::new();
    let mut need_space = false;
    for line in formatted.lines().rev() {
        if need_space && !line.starts_with("#") {
            if !line.is_empty() {
                lines_rev.push("");
            }

            need_space = false;
        }

        need_space = need_space
            // The formatting ensures that such prefix implies the appropriate node type
            || line.starts_with("type ")
            || line.starts_with("error ")
            || line.starts_with("method ");

        lines_rev.push(line);
    }

    lines_rev
        .iter()
        .rev()
        .fold(String::new(), |acc, line| acc + line + "\n")
}

pub fn format(
    language: &topiary_core::Language,
    input: &mut impl std::io::Read,
) -> Result<String, topiary_core::FormatterError> {
    let mut output = Vec::new();
    topiary_core::formatter(
        input,
        &mut output,
        language,
        topiary_core::Operation::Format {
            skip_idempotence: false,
            tolerate_parsing_errors: false,
        },
    )?;

    Ok(space_members(&String::from_utf8(output).unwrap()))
}
