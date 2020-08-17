use spongemock::mock;
use std::io::{self, BufRead};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Outputs a MoCkInG version of its input
///
/// Takes each INPUT_TO_MOCK argument, converts it to a MoCkEd version and outputs it on its own
/// line.
struct Opt {
    /// Each argument will be MoCkEd on a new line in the output.
    ///
    /// If INPUT_TO_MOCK is not provided, input will instead be read from stdin.
    #[structopt(name = "INPUT_TO_MOCK")]
    mock_vals: Vec<String>,
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(error) => {
            eprintln!("{:?}", error);
            1
        }
    });
}

fn run_app() -> Result<(), io::Error> {
    let opt: Opt = Opt::from_args();

    if !opt.mock_vals.is_empty() {
        for mock_val in opt.mock_vals {
            println!("{}", mock(&mock_val));
        }
    } else {
        for line in io::stdin().lock().lines() {
            println!("{}", mock(&line?));
        }
    }
    Ok(())
}
