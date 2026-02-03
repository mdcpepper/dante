//! Simple Discounts Example
//!
//! This example demonstrates simple percentage discounts applied to items.
//! Two promotions are configured, one that applies a 20% discount to items
//! tagged "20-off", and one that applies a 40% discount to items tagged
//! "40-off".
//!
//! Run with: `cargo run --example simple_discounts`

use std::time::Instant;

use anyhow::Result;

use dante::{
    fixtures::Fixture,
    items::groups::ItemGroup,
    receipt::Receipt,
    solvers::{Solver, ilp::ILPSolver},
};

/// Simple Discounts Example
#[expect(clippy::print_stdout, reason = "Example code")]
pub fn main() -> Result<()> {
    // Load fixture set
    let fixture = Fixture::from_set("example_simple_discounts")?;

    let basket = fixture.basket()?;
    let item_group: ItemGroup<'_> = ItemGroup::from(&basket);
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

    println!("\nSolution: {elapsed}s");

    Ok(())
}
