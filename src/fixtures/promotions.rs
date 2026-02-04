//! Promotion Fixtures

use rustc_hash::FxHashMap;
use rusty_money::Money;
use serde::Deserialize;

use crate::{
    discounts::SimpleDiscount,
    fixtures::{
        FixtureError,
        products::{parse_percentage, parse_price},
    },
    promotions::{
        Promotion, PromotionKey, PromotionMeta, direct_discount::DirectDiscountPromotion,
        positional_discount::PositionalDiscountPromotion,
    },
    tags::string::StringTagCollection,
};

/// Wrapper for promotions in YAML
#[derive(Debug, Deserialize)]
pub struct PromotionsFixture {
    /// Map of promotion key -> promotion fixture
    pub promotions: FxHashMap<String, PromotionFixture>,
}

/// Promotion fixture from YAML
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PromotionFixture {
    /// Direct Discount Promotion
    DirectDiscount {
        /// Promotion name
        name: String,

        /// Promotion tags for targeting
        tags: Vec<String>,

        /// Discount configuration
        discount: SimpleDiscountFixture,
    },

    /// Positional Discount Promotion
    PositionalDiscount {
        /// Promotion name
        name: String,

        /// Promotion tags
        tags: Vec<String>,

        /// Size of the bundle
        size: u16,

        /// The nth item in the bundle to apply the discount to
        positions: Vec<u16>,

        /// Discount configuration
        discount: SimpleDiscountFixture,
    },
}

impl PromotionFixture {
    /// Convert to `PromotionMeta` and `Promotion`
    ///
    /// # Errors
    ///
    /// Returns an error if the discount configuration is invalid.
    pub fn try_into_promotion(
        self,
        key: PromotionKey,
    ) -> Result<(PromotionMeta, Promotion<'static>), FixtureError> {
        match self {
            PromotionFixture::DirectDiscount {
                name,
                tags,
                discount,
            } => {
                let meta = PromotionMeta { name: name.clone() };
                let tag_refs: Vec<&str> = tags.iter().map(String::as_str).collect();
                let promotion = Promotion::DirectDiscount(DirectDiscountPromotion::new(
                    key,
                    StringTagCollection::from_strs(&tag_refs),
                    SimpleDiscount::try_from(discount)?,
                ));

                Ok((meta, promotion))
            }
            Self::PositionalDiscount {
                name,
                tags,
                size,
                positions,
                discount,
            } => {
                let meta = PromotionMeta { name: name.clone() };

                let tag_refs: Vec<&str> = tags.iter().map(String::as_str).collect();

                let promotion = Promotion::PositionalDiscount(PositionalDiscountPromotion::new(
                    key,
                    StringTagCollection::from_strs(&tag_refs),
                    size,
                    positions.into(),
                    SimpleDiscount::try_from(discount)?,
                ));

                Ok((meta, promotion))
            }
        }
    }
}

/// Simple Discount configuration from YAML fixtures
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SimpleDiscountFixture {
    /// Percentage discount (supports "15%" or "0.15" formats)
    PercentageOff {
        /// Discount percentage (e.g., "15%" or "0.15" for 15%)
        amount: String,
    },

    /// Fixed price override (e.g., "2.50 GBP")
    AmountOverride {
        /// Price string (e.g., "2.50 GBP")
        amount: String,
    },

    /// Fixed amount discount off (e.g., "0.75 GBP")
    AmountOff {
        /// Discount amount string (e.g., "0.75 GBP")
        amount: String,
    },
}

impl TryFrom<SimpleDiscountFixture> for SimpleDiscount<'_> {
    type Error = FixtureError;

    fn try_from(config: SimpleDiscountFixture) -> Result<Self, Self::Error> {
        match config {
            SimpleDiscountFixture::PercentageOff { amount: percentage } => Ok(
                SimpleDiscount::PercentageOff(parse_percentage(&percentage)?),
            ),
            SimpleDiscountFixture::AmountOverride { amount } => {
                let (minor_units, currency) = parse_price(&amount)?;

                Ok(SimpleDiscount::AmountOverride(Money::from_minor(
                    minor_units,
                    currency,
                )))
            }
            SimpleDiscountFixture::AmountOff { amount } => {
                let (minor_units, currency) = parse_price(&amount)?;

                Ok(SimpleDiscount::AmountOff(Money::from_minor(
                    minor_units,
                    currency,
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use decimal_percentage::Percentage;
    use rusty_money::iso::GBP;
    use testresult::TestResult;

    use crate::{
        discounts::SimpleDiscount,
        promotions::{Promotion, PromotionKey},
        tags::collection::TagCollection,
    };

    use super::*;

    #[test]
    fn promotion_fixture_rejects_unknown_type() {
        let yaml = r"
type: unknown_promotion
name: Test
tags: []
discount:
  type: percentage
  value: 0.10
";
        let result: Result<PromotionFixture, _> = serde_norway::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn discount_fixture_parses_percentage() -> Result<(), FixtureError> {
        let fixture = SimpleDiscountFixture::PercentageOff {
            amount: "15%".to_string(),
        };

        let config = SimpleDiscount::try_from(fixture)?;

        assert!(matches!(
            config,
            SimpleDiscount::PercentageOff(percent) if percent == Percentage::from(0.15)
        ));

        Ok(())
    }

    #[test]
    fn discount_fixture_parses_percentage_decimal_format() -> Result<(), FixtureError> {
        let fixture = SimpleDiscountFixture::PercentageOff {
            amount: "0.15".to_string(),
        };

        let config = SimpleDiscount::try_from(fixture)?;

        assert!(matches!(
            config,
            SimpleDiscount::PercentageOff(percent) if percent == Percentage::from(0.15)
        ));

        Ok(())
    }

    #[test]
    fn discount_fixture_parses_amount_override() -> Result<(), FixtureError> {
        let fixture = SimpleDiscountFixture::AmountOverride {
            amount: "2.50 GBP".to_string(),
        };

        let config = SimpleDiscount::try_from(fixture)?;

        assert!(matches!(
            config,
            SimpleDiscount::AmountOverride(money) if money.to_minor_units() == 250
                && money.currency() == GBP
        ));

        Ok(())
    }

    #[test]
    fn discount_fixture_parses_amount_discount_off() -> Result<(), FixtureError> {
        let fixture = SimpleDiscountFixture::AmountOff {
            amount: "0.75 GBP".to_string(),
        };

        let config = SimpleDiscount::try_from(fixture)?;

        assert!(matches!(
            config,
            SimpleDiscount::AmountOff(money) if money.to_minor_units() == 75
                && money.currency() == GBP
        ));

        Ok(())
    }

    #[test]
    fn discount_fixture_rejects_unknown_discount_type() {
        let yaml = r"
type: mystery_discount
value: 0.10
";
        let result: Result<SimpleDiscountFixture, _> = serde_norway::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn discount_fixture_rejects_invalid_percentage_string() {
        let fixture = SimpleDiscountFixture::PercentageOff {
            amount: "not a number".to_string(),
        };

        let result = SimpleDiscount::try_from(fixture);
        assert!(matches!(result, Err(FixtureError::InvalidPercentage(_))));
    }

    #[test]
    fn promotion_fixture_converts_direct_discount() -> TestResult {
        let fixture = PromotionFixture::DirectDiscount {
            name: "Member Sale".to_string(),
            tags: vec!["member".to_string(), "sale".to_string()],
            discount: SimpleDiscountFixture::AmountOff {
                amount: "0.50 GBP".to_string(),
            },
        };

        let (meta, promotion) = fixture.try_into_promotion(PromotionKey::default())?;

        assert_eq!(meta.name, "Member Sale");

        match promotion {
            Promotion::DirectDiscount(promo) => {
                assert!(promo.tags().contains("member"));
                assert!(matches!(
                    promo.discount(),
                    SimpleDiscount::AmountOff(amount) if amount.to_minor_units() == 50
                ));
            }
            Promotion::PositionalDiscount(_) => {
                panic!("Expected direct discount promotion")
            }
        }

        Ok(())
    }

    #[test]
    fn promotion_fixture_converts_positional_discount() -> TestResult {
        let fixture = PromotionFixture::PositionalDiscount {
            name: "3-for-2".to_string(),
            tags: vec!["snack".to_string()],
            size: 3,
            positions: vec![2],
            discount: SimpleDiscountFixture::PercentageOff {
                amount: "50%".to_string(),
            },
        };

        let (meta, promotion) = fixture.try_into_promotion(PromotionKey::default())?;

        assert_eq!(meta.name, "3-for-2");

        match promotion {
            Promotion::PositionalDiscount(promo) => {
                assert!(promo.tags().contains("snack"));
                assert_eq!(promo.size(), 3);
                assert_eq!(promo.positions(), &[2]);
                assert!(matches!(promo.discount(), SimpleDiscount::PercentageOff(_)));
            }
            Promotion::DirectDiscount(_) => {
                panic!("Expected positional discount promotion")
            }
        }

        Ok(())
    }
}
