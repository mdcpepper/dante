//! Complex Discounts Example
//!
//! This example demonstrates a basket with multiple promotion types
//!
//! Run with: `cargo run --example complex`

use std::{io, time::Instant};

use anyhow::Result;

use clap::Parser;
use dante::{
    fixtures::Fixture,
    items::groups::ItemGroup,
    receipt::Receipt,
    solvers::{Solver, ilp::ILPSolver},
    utils::ExampleBasketArgs,
};

/// Complex Discounts Example
#[expect(clippy::print_stdout, reason = "Example code")]
pub fn main() -> Result<()> {
    let args = ExampleBasketArgs::parse();

    // Load fixture set
    let fixture = Fixture::from_set("example_complex")?;

    let basket = fixture.basket(args.n)?;
    let item_group = ItemGroup::from(&basket);
    let promotions = fixture.promotions();

    let start = Instant::now();

    let result = ILPSolver::solve(promotions, &item_group)?;

    let elapsed = start.elapsed().as_secs_f32();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    Receipt::from_solver_result(&basket, result)?.write_to(
        &mut handle,
        &basket,
        fixture.product_meta_map(),
        fixture.promotion_meta_map(),
    )?;

    println!("\nSolution: {elapsed}s");

    Ok(())
}
