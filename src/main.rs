use clap::Parser;
use std::fs::OpenOptions;
use std::io::{self, Seek, SeekFrom, Write};
use std::path::PathBuf;

use varlinkfmt_core::{Indent, formatter, mk_language};

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
            match OpenOptions::new().read(true).write(true).open(&path) {
                Err(err) => errexit!("Failure opening `{}`: {}", path.display(), err),
                Ok(mut file) => {
                    let mut output = Vec::new();
                    if let Err(err) =
                        formatter(&mut file, &mut output, &language, Default::default())
                    {
                        errexit!("Failure formatting `{}`: {}", path.display(), err);
                    }

                    if let Err(err) = file
                        .seek(SeekFrom::Start(0))
                        .and_then(|_| file.set_len(0))
                        .and_then(|_| file.write_all(&output))
                    {
                        errexit!("Failure writing `{}`: {}", path.display(), err);
                    }
                }
            };
        }
    } else {
        if let Err(err) = formatter(
            &mut io::stdin(),
            &mut io::stdout(),
            &language,
            Default::default(),
        ) {
            errexit!("Failure formatting: {}", err);
        }
    }
}
