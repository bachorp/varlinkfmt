pub use topiary_core;
pub use topiary_tree_sitter_facade;

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
fn postprocess(
    formatted: Vec<u8>,
    output: &mut impl std::io::Write,
) -> topiary_core::FormatterResult<()> {
    // SAFETY: Topiary will error on non-UTF8 input
    let formatted = String::from_utf8(formatted).unwrap();

    let mut lines_rev = Vec::<&str>::new();
    {
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

            // Remove trailing whitespace in comments
            lines_rev.push(line.trim_end());
        }
    }

    for line in lines_rev.iter().rev() {
        output.write(line.as_bytes())?;
        output.write("\n".as_bytes())?;
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub skip_idempotence: bool,
    /// Note that tolerating parsing errors can induce messy output and idempotence errors
    pub tolerate_parsing_errors: bool,
}

pub fn formatter(
    input: &mut impl std::io::Read,
    output: &mut impl std::io::Write,
    language: &topiary_core::Language,
    options: Options,
) -> topiary_core::FormatterResult<()> {
    let mut formatted = Vec::new();
    topiary_core::formatter(
        input,
        &mut formatted,
        language,
        topiary_core::Operation::Format {
            skip_idempotence: options.skip_idempotence,
            tolerate_parsing_errors: options.tolerate_parsing_errors,
        },
    )?;

    postprocess(formatted, output)
}

pub fn formatter_str(
    input: &str,
    output: &mut impl std::io::Write,
    language: &topiary_core::Language,
    options: Options,
) -> topiary_core::FormatterResult<()> {
    let mut formatted = Vec::new();
    topiary_core::formatter_str(
        input,
        &mut formatted,
        language,
        topiary_core::Operation::Format {
            skip_idempotence: options.skip_idempotence,
            tolerate_parsing_errors: options.tolerate_parsing_errors,
        },
    )?;

    postprocess(formatted, output)
}

pub fn formatter_tree(
    tree: topiary_tree_sitter_facade::Tree,
    input_content: &str,
    output: &mut impl std::io::Write,
    language: &topiary_core::Language,
    options: Options,
) -> topiary_core::FormatterResult<()> {
    let mut formatted = Vec::new();
    topiary_core::formatter_tree(
        tree,
        input_content,
        &mut formatted,
        language,
        topiary_core::Operation::Format {
            skip_idempotence: options.skip_idempotence,
            tolerate_parsing_errors: options.tolerate_parsing_errors,
        },
    )?;

    postprocess(formatted, output)
}
