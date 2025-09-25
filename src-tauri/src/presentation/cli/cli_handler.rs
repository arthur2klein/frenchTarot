use crate::presentation::presentation_error::PresentationError;
use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(name = "tarot")]
pub struct CliArgs {}

fn cli_handler(_args: CliArgs) -> Result<(), PresentationError> {
    Ok(())
}
