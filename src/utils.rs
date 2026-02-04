//! Utils

use clap::Parser;

/// Arguments for the basket examples
#[derive(Debug, Parser)]
pub struct ExampleBasketArgs {
    /// Number of items to add to the basket
    #[clap(short, long)]
    pub n: Option<usize>,
}
