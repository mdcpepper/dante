//! Positional Discounts Example
//!
//! This example demonstrates percentage discounts that are applied to the nth
//! item in each bundle, when qualifying items are sorted by descending price.
//!
//! This covers promotions such as "Buy 1 Get 1 Free", "Buy 1 Get 1 Half Price",
//!  "3 for 2", etc.

use std::{io::Write, time::Instant};

use anyhow::Result;
use clap::Parser;
use dante::{
    fixtures::Fixture,
    items::groups::ItemGroup,
    receipt::Receipt,
    solvers::{Solver, ilp::ILPSolver},
    utils::ExampleBasketArgs,
};

/// Positional Discounts Example
pub fn main() -> Result<()> {
    let args = ExampleBasketArgs::parse();

    // Load fixture set
    let fixture = Fixture::from_set("example_positional_discounts")?;

    let basket = fixture.basket(args.n)?;
    let item_group = ItemGroup::from(&basket);
    let promotions = fixture.promotions();

    let start = Instant::now();

    let result = ILPSolver::solve(promotions, &item_group)?;

    let elapsed = start.elapsed().as_secs_f32();

    let receipt = Receipt::from_solver_result(&basket, result)?;

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    receipt.write_to(
        &mut handle,
        &basket,
        fixture.product_meta_map(),
        fixture.promotion_meta_map(),
    )?;

    writeln!(handle, "\nSolution: {elapsed}s")?;

    Ok(())
}
