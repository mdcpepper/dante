//! Promotions

use crate::{
    basket::Basket, promotions::simple_discount::SimpleDisount,
    solvers::ilp::promotions::ILPPromotion,
};

pub mod simple_discount;

/// Promotion enum
#[derive(Debug, Clone)]
pub enum Promotion<'a> {
    /// Simple discount promotion
    SimpleDiscount(SimpleDisount<'a>),
}

impl<'a> Promotion<'a> {
    /// Return whether this promotion *might* apply to the given basket and candidate items.
    pub fn is_applicable(&self, basket: &'a Basket<'a>, items: &[usize]) -> bool {
        match self {
            Promotion::SimpleDiscount(simple_disount) => {
                simple_disount.is_applicable(basket, items)
            }
        }
    }
}
