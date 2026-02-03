//! Promotion Fixtures

use decimal_percentage::Percentage;
use rustc_hash::FxHashMap;
use serde::Deserialize;

use crate::{
    discounts::Discount,
    fixtures::{FixtureError, products::parse_price},
    promotions::{Promotion, PromotionKey, PromotionMeta, simple_discount::SimpleDiscount},
    tags::string::StringTagCollection,
};

/// Wrapper for promotions in YAML
#[derive(Debug, Deserialize)]
pub struct PromotionsFixture {
    /// Map of promotion key -> promotion fixture
    pub promotions: FxHashMap<String, PromotionFixture>,
}

/// Promotion Fixture
#[derive(Debug, Deserialize)]
pub struct PromotionFixture {
    /// Promotion type (e.g., "`simple_discount`")
    #[serde(rename = "type")]
    pub promotion_type: String,

    /// Promotion name
    pub name: String,

    /// Promotion tags
    pub tags: Vec<String>,

    /// Discount configuration
    pub discount: DiscountFixture,
}

/// Discount Fixture
#[derive(Debug, Deserialize)]
pub struct DiscountFixture {
    /// Discount type
    #[serde(rename = "type")]
    pub discount_type: String,

    /// Percentage value (e.g., "20%")
    pub percentage: Option<String>,

    /// Fixed price (e.g., "100 GBP")
    pub price: Option<String>,
}

impl PromotionFixture {
    /// Convert to `PromotionMeta` and `Promotion`
    ///
    /// # Errors
    ///
    /// Returns an error if the promotion type is unsupported or if the discount
    /// configuration is invalid.
    pub fn try_into_promotion(
        self,
        key: PromotionKey,
    ) -> Result<(PromotionMeta, Promotion<'static>), FixtureError> {
        let meta = PromotionMeta {
            name: self.name.clone(),
        };

        match self.promotion_type.as_str() {
            "simple_discount" => {
                let discount = self.discount.try_into_discount()?;
                let tag_refs: Vec<&str> = self.tags.iter().map(String::as_str).collect();
                let tags = StringTagCollection::from_strs(&tag_refs);

                let simple_discount = SimpleDiscount::new(key, tags, discount);
                let promotion = Promotion::SimpleDiscount(simple_discount);

                Ok((meta, promotion))
            }
            other => Err(FixtureError::UnsupportedPromotionType(other.to_string())),
        }
    }
}

impl DiscountFixture {
    /// Convert to `Discount`
    ///
    /// # Errors
    ///
    /// Returns an error if the discount type is unsupported or if required fields
    /// (percentage or price) are missing.
    fn try_into_discount(self) -> Result<Discount<'static>, FixtureError> {
        match self.discount_type.as_str() {
            "percentage_off_bundle_total" => {
                let percentage_str = self.percentage.ok_or_else(|| {
                    FixtureError::InvalidPercentage(
                        "percentage_off_bundle_total requires percentage field".to_string(),
                    )
                })?;

                let percentage = parse_percentage(&percentage_str)?;

                Ok(Discount::PercentageOffBundleTotal(percentage))
            }
            "percentage_off_cheapest_item" => {
                let percentage_str = self.percentage.ok_or_else(|| {
                    FixtureError::InvalidPercentage(
                        "percentage_off_cheapest_item requires percentage field".to_string(),
                    )
                })?;

                let percentage = parse_percentage(&percentage_str)?;

                Ok(Discount::PercentageOffCheapestItem(percentage))
            }
            "set_bundle_total_price" => {
                let price_str = self.price.ok_or_else(|| {
                    FixtureError::InvalidPrice(
                        "set_bundle_total_price requires price field".to_string(),
                    )
                })?;

                let (minor_units, currency) = parse_price(&price_str)?;
                let money = rusty_money::Money::from_minor(minor_units, currency);

                Ok(Discount::SetBundleTotalPrice(money))
            }
            "set_cheapest_item_price" => {
                let price_str = self.price.ok_or_else(|| {
                    FixtureError::InvalidPrice(
                        "set_cheapest_item_price requires price field".to_string(),
                    )
                })?;

                let (minor_units, currency) = parse_price(&price_str)?;
                let money = rusty_money::Money::from_minor(minor_units, currency);

                Ok(Discount::SetCheapestItemPrice(money))
            }
            other => Err(FixtureError::UnsupportedPromotionType(format!(
                "discount type: {other}"
            ))),
        }
    }
}

/// Parse percentage string (e.g., "20%") into `Percentage`
///
/// # Errors
///
/// Returns an error if the string does not end with '%' or if the number
/// cannot be parsed as a valid float.
pub fn parse_percentage(s: &str) -> Result<Percentage, FixtureError> {
    if !s.ends_with('%') {
        return Err(FixtureError::InvalidPercentage(format!(
            "Expected format 'NUMBER%', got: {s}"
        )));
    }

    let number_str = s.strip_suffix('%').ok_or_else(|| {
        FixtureError::InvalidPercentage(format!("Expected format 'NUMBER%', got: {s}"))
    })?;

    let number = number_str
        .parse::<f64>()
        .map_err(|_err| FixtureError::InvalidPercentage(s.to_string()))?;

    let decimal = number / 100.0;

    Ok(Percentage::from(decimal))
}

#[cfg(test)]
mod tests {
    use decimal_percentage::Percentage;
    use rusty_money::iso::GBP;

    use crate::discounts::Discount;

    use super::*;

    #[test]
    fn promotion_fixture_rejects_unknown_type() {
        let fixture = PromotionFixture {
            promotion_type: "test".to_string(),
            name: "Test Promotion".to_string(),
            tags: vec![],
            discount: DiscountFixture {
                discount_type: "percentage_off_bundle_total".to_string(),
                percentage: Some("10%".to_string()),
                price: None,
            },
        };

        let result = fixture.try_into_promotion(PromotionKey::default());

        assert!(matches!(
            result,
            Err(FixtureError::UnsupportedPromotionType(ref msg)) if msg == "test"
        ));
    }

    #[test]
    fn discount_fixture_requires_percentage_for_bundle_total() {
        let fixture = DiscountFixture {
            discount_type: "percentage_off_bundle_total".to_string(),
            percentage: None,
            price: None,
        };

        let result = fixture.try_into_discount();

        assert!(matches!(result, Err(FixtureError::InvalidPercentage(_))));
    }

    #[test]
    fn discount_fixture_requires_percentage_for_cheapest_item() {
        let fixture = DiscountFixture {
            discount_type: "percentage_off_cheapest_item".to_string(),
            percentage: None,
            price: None,
        };

        let result = fixture.try_into_discount();

        assert!(matches!(result, Err(FixtureError::InvalidPercentage(_))));
    }

    #[test]
    fn discount_fixture_parses_cheapest_item_percentage() -> Result<(), FixtureError> {
        let fixture = DiscountFixture {
            discount_type: "percentage_off_cheapest_item".to_string(),
            percentage: Some("15%".to_string()),
            price: None,
        };

        let result = fixture.try_into_discount()?;

        assert!(matches!(
            result,
            Discount::PercentageOffCheapestItem(percent) if percent == Percentage::from(0.15)
        ));

        Ok(())
    }

    #[test]
    fn discount_fixture_requires_price_for_bundle_total() {
        let fixture = DiscountFixture {
            discount_type: "set_bundle_total_price".to_string(),
            percentage: None,
            price: None,
        };

        let result = fixture.try_into_discount();

        assert!(matches!(result, Err(FixtureError::InvalidPrice(_))));
    }

    #[test]
    fn discount_fixture_parses_bundle_total_price() -> Result<(), FixtureError> {
        let fixture = DiscountFixture {
            discount_type: "set_bundle_total_price".to_string(),
            percentage: None,
            price: Some("2.50 GBP".to_string()),
        };

        let result = fixture.try_into_discount()?;

        assert!(matches!(
            result,
            Discount::SetBundleTotalPrice(money) if money.to_minor_units() == 250
                && money.currency() == GBP
        ));

        Ok(())
    }

    #[test]
    fn discount_fixture_requires_price_for_cheapest_item() {
        let fixture = DiscountFixture {
            discount_type: "set_cheapest_item_price".to_string(),
            percentage: None,
            price: None,
        };

        let result = fixture.try_into_discount();

        assert!(matches!(result, Err(FixtureError::InvalidPrice(_))));
    }

    #[test]
    fn discount_fixture_parses_cheapest_item_price() -> Result<(), FixtureError> {
        let fixture = DiscountFixture {
            discount_type: "set_cheapest_item_price".to_string(),
            percentage: None,
            price: Some("0.75 GBP".to_string()),
        };

        let result = fixture.try_into_discount()?;

        assert!(matches!(
            result,
            Discount::SetCheapestItemPrice(money) if money.to_minor_units() == 75
                && money.currency() == GBP
        ));

        Ok(())
    }

    #[test]
    fn discount_fixture_rejects_unknown_discount_type() {
        let fixture = DiscountFixture {
            discount_type: "mystery_discount".to_string(),
            percentage: None,
            price: None,
        };

        let result = fixture.try_into_discount();

        assert!(matches!(
            result,
            Err(FixtureError::UnsupportedPromotionType(ref msg))
                if msg == "discount type: mystery_discount"
        ));
    }

    #[test]
    fn parse_percentage_rejects_missing_percent_sign() {
        let result = parse_percentage("25");

        assert!(matches!(result, Err(FixtureError::InvalidPercentage(_))));
    }

    #[test]
    fn parse_percentage_rejects_invalid_number() {
        let result = parse_percentage("oops%");

        assert!(matches!(result, Err(FixtureError::InvalidPercentage(_))));
    }
}
