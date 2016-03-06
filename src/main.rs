extern crate curl;
extern crate docopt;
extern crate rustc_serialize;

#[macro_use]
extern crate log;

// use std::env;
// use std::fmt;
use std::process;
use docopt::Docopt;

mod cmd;


macro_rules! check_repopath {
    ($path:expr) => (
        if $path.len() != 2 {
            println!("<repopath> must have the form <owner>/<repo>.  e.g. ustwo/github-issues");
            process::exit(1)
        }
    );
}

const USAGE: &'static str = "
Github issue consumer.

Usage:
    github-issues <command> <repopath> [--label=<label>...]

Options:
    -h, --help          Display this message
    -V, --version       Print version info and exit
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_command: Option<Command>,
    arg_repopath: String,
    flag_label: Vec<String>,
}

#[derive(Debug, RustcDecodable)]
enum Command {
    Fetch
}

// impl Command {
//     fn run(self) -> CliResult<()> {
//         println!("foo");
//     }
// }

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    match args.arg_command {
        None => {
            println!("Use --help for more info");
            process::exit(0);
        }
        Some(cmd) => {
            match cmd {
                Command::Fetch => {
                    let repopath: Vec<&str> = args.arg_repopath.split("/").collect();
                    let flags = args.flag_label;

                    check_repopath!(repopath);

                    cmd::fetch::run(repopath[0],
                                    repopath[1],
                                    flags);
                }
            }
        }
    }


}
