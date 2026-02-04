//! Simple Discount
//!
//! A simple fixed amount or percentage discount on all qualifying items

use crate::{
    discounts::{DiscountError, percent_of_minor},
    items::Item,
    promotions::PromotionKey,
    tags::{collection::TagCollection, string::StringTagCollection},
};
use decimal_percentage::Percentage;
use rusty_money::{Money, iso::Currency};

/// Discount configuration for `SimpleDiscount` promotions.
#[derive(Debug, Copy, Clone)]
pub enum SimpleDiscountConfig<'a> {
    /// Apply a percentage discount (e.g., "25% off")
    Percentage(Percentage),

    /// Replace item price with a fixed amount (e.g., "£5 each")
    AmountOverride(Money<'a, Currency>),

    /// Subtract a fixed amount from item price (e.g., "£2 off")
    AmountDiscountOff(Money<'a, Currency>),
}

/// A Simple Fixed or Percentage Discount
#[derive(Debug, Copy, Clone)]
pub struct SimpleDiscount<'a, T: TagCollection = StringTagCollection> {
    key: PromotionKey,
    tags: T,
    config: SimpleDiscountConfig<'a>,
}

impl<'a, T: TagCollection> SimpleDiscount<'a, T> {
    /// Create a new simple discount promotion.
    pub fn new(key: PromotionKey, tags: T, config: SimpleDiscountConfig<'a>) -> Self {
        Self { key, tags, config }
    }

    /// Return the promotion key
    pub fn key(&self) -> PromotionKey {
        self.key
    }

    /// Return the tags
    pub fn tags(&self) -> &T {
        &self.tags
    }

    /// Returns the discount configuration
    pub fn config(&self) -> &SimpleDiscountConfig<'a> {
        &self.config
    }

    /// Calculate the discounted price for a single item.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Percentage calculation overflows or cannot be safely represented.
    /// - Money arithmetic fails (e.g., currency mismatch, negative result).
    pub fn calculate_discounted_price(
        &self,
        item: &Item<'a, T>,
    ) -> Result<Money<'a, Currency>, DiscountError> {
        match &self.config {
            SimpleDiscountConfig::Percentage(pct) => {
                // Calculate the discount amount in minor units
                let original_minor = item.price().to_minor_units();

                let discounted_minor = original_minor
                    .checked_sub(percent_of_minor(pct, original_minor)?)
                    .ok_or(DiscountError::PercentConversion)?;

                Ok(Money::from_minor(discounted_minor, item.price().currency()))
            }
            SimpleDiscountConfig::AmountOverride(amount) => {
                // Replace price with fixed amount
                Ok(*amount)
            }
            SimpleDiscountConfig::AmountDiscountOff(amount) => {
                // Subtract amount from price
                Ok(item.price().sub(*amount)?)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use decimal_percentage::Percentage;
    use rusty_money::{Money, iso};
    use slotmap::SlotMap;
    use testresult::TestResult;

    use crate::{items::Item, products::ProductKey};

    use super::*;

    #[test]
    fn key_returns_constructor_key() {
        let mut keys = SlotMap::<PromotionKey, ()>::with_key();
        let key = keys.insert(());

        let promo = SimpleDiscount::new(
            key,
            StringTagCollection::empty(),
            SimpleDiscountConfig::AmountOverride(Money::from_minor(0, iso::GBP)),
        );

        assert_eq!(promo.key(), key);
        assert_ne!(promo.key(), PromotionKey::default());
    }

    #[test]
    fn calculate_discounted_price_percentage() -> TestResult {
        let promo = SimpleDiscount::new(
            PromotionKey::default(),
            StringTagCollection::empty(),
            SimpleDiscountConfig::Percentage(Percentage::from(0.25)),
        );

        let item = Item::new(ProductKey::default(), Money::from_minor(100, iso::GBP));
        let discounted = promo.calculate_discounted_price(&item)?;

        assert_eq!(discounted, Money::from_minor(75, iso::GBP));

        Ok(())
    }

    #[test]
    fn calculate_discounted_price_amount_override() -> TestResult {
        let promo = SimpleDiscount::new(
            PromotionKey::default(),
            StringTagCollection::empty(),
            SimpleDiscountConfig::AmountOverride(Money::from_minor(50, iso::GBP)),
        );

        let item = Item::new(ProductKey::default(), Money::from_minor(100, iso::GBP));
        let discounted = promo.calculate_discounted_price(&item)?;

        assert_eq!(discounted, Money::from_minor(50, iso::GBP));

        Ok(())
    }

    #[test]
    fn calculate_discounted_price_amount_discount_off() -> TestResult {
        let promo = SimpleDiscount::new(
            PromotionKey::default(),
            StringTagCollection::empty(),
            SimpleDiscountConfig::AmountDiscountOff(Money::from_minor(25, iso::GBP)),
        );

        let item = Item::new(ProductKey::default(), Money::from_minor(100, iso::GBP));
        let discounted = promo.calculate_discounted_price(&item)?;

        assert_eq!(discounted, Money::from_minor(75, iso::GBP));

        Ok(())
    }
}
