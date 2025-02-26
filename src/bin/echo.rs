// needs to be adjusted to use clap

struct Args {
    program_name: String,
    args: Vec<String>,
    options: Option<Options>,      // Option<enum> because the enum could be none 
}

enum Options {
    Help,
    Version,
    NoNewline,
}

impl Args {
    fn new() -> Self {
        let mut args = std::env::args();

        let program_name = args.next().unwrap();
        let args: Vec<String> = args.collect();

        let options = args.first().and_then(|arg| match arg.as_str() {
            "-h" | "--help" => Some(Options::Help),
            "-v" | "--version" => Some(Options::Version),
            "-n" | "--no-newline" => Some(Options::NoNewline),
            _ => None,
        });

        Args {
            program_name,
            args,
            options,
        }
    }

    fn echo(&self) {
        if let Some(ref options) = self.options {       // using "ref" options here allows to pattern match a reference, so that we can use the options variable later
            match *options {
                Options::Help => self.print_help(),
                Options::Version => self.print_version(),
                Options::NoNewline => {
                    print!("{}", &self.args[1..].join(" "));      // print no newline, so we use print! instead of println!
                },                                                // we use &self.args[1..] to skip the first argument which is the option
            }
        } else {
            println!("{}", self.args.join(" "));
        }
    }

    fn print_help(&self) {
        println!(r#"Usage: {} [OPTION]... [ARGS]...
    -h, --help     display this help and exit
    -v, --version  output version information and exit
    -n	--newline  do not append a newline
    "#, &self.program_name);
    }

    fn print_version(&self) {
        println!("{} version 1.0", &self.program_name);
    }
}

fn main() {
    let args = Args::new();
    args.echo();
}