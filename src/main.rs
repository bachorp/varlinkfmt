use clap::Parser;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

use varlinkfmt::{Indent, format, mk_language};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(
        long,
        short('i'),
        help = "Format given FILE... in place. If omitted, will read from stdin"
    )]
    inplace: bool,

    #[arg(
        long,
        short('d'),
        value_name = "N",
        value_parser(parse_indent),
        help = "Number of spaces to indent. If omitted, tabs will be used"
    )]
    indent: Option<Indent>,

    #[arg(
        value_name = "FILE",
        help = "To be formatted if --inplace is specified"
    )]
    files: Vec<PathBuf>,
}

fn parse_indent(raw: &str) -> Result<Indent, String> {
    match raw.parse::<usize>() {
        Ok(n) => Ok(Indent::Spaces(n)),
        Err(e) => Err(e.to_string()),
    }
}

macro_rules! errexit {
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
        std::process::exit(1);
    }};
}

fn main() {
    let args = Args::parse();
    let language = mk_language(args.indent.unwrap_or(Indent::Tab));

    if args.inplace {
        for path in args.files {
            match fs::read_to_string(&path) {
                Err(e) => errexit!("Failure reading `{}`:\n{}", path.display(), e),
                Ok(contents) => match format(&language, &mut contents.as_bytes()) {
                    Err(e) => errexit!("Failure formatting `{}`:\n{}", path.display(), e),
                    Ok(out) => {
                        if let Err(e) = fs::write(&path, out) {
                            errexit!("Failure writing `{}`:\n{}", path.display(), e);
                        }
                    }
                },
            }
        }
    } else {
        let mut stdin = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut stdin) {
            errexit!("Failure reading from stdin:\n{}", e);
        }

        match format(&language, &mut stdin.as_bytes()) {
            Err(e) => errexit!("Failure formatting:\n{}", e),
            Ok(out) => print!("{}", out),
        }
    }
}
