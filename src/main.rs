mod cli;
pub use ctbox;
use cli::Cli;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    Cli::process();

    Ok(())
}
