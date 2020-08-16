use spongemock::mock;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Outputs a MoCkInG version of its input
///
/// Takes each INPUT_TO_MOCK argument, converts it to a MoCkEd version and outputs it on its own
/// line.
struct Opt {
    /// Each argument will be MoCkEd on a new line in the output.
    #[structopt(name = "INPUT_TO_MOCK", required = true)]
    mock_vals: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();

    for mock_val in opt.mock_vals {
        println!("{}", mock(mock_val));
    }
}
