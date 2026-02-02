//! Simple Discount
//!
//! A simple fixed amount or percentage discount on all qualifying items

use crate::{
    discounts::Discount,
    tags::{collection::TagCollection, string::StringTagCollection},
};

/// A Simple Fixed or Percentage Discount
#[derive(Debug, Copy, Clone)]
pub struct SimpleDisount<'a, T: TagCollection = StringTagCollection> {
    tags: T,
    discount: Discount<'a>,
}

impl<'a, T: TagCollection> SimpleDisount<'a, T> {
    /// Create a new simple discount promotion.
    pub fn new(tags: T, discount: Discount<'a>) -> Self {
        Self { tags, discount }
    }

    /// Return the tags
    pub fn tags(&self) -> &T {
        &self.tags
    }

    /// Returns the discount
    pub fn discount(&self) -> &Discount<'a> {
        &self.discount
    }
}
