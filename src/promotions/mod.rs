//! Promotions

use slotmap::new_key_type;

use crate::{
    items::groups::ItemGroup,
    promotions::{
        direct_discount::DirectDiscountPromotion, positional_discount::PositionalDiscountPromotion,
    },
    solvers::ilp::promotions::ILPPromotion,
};

pub mod applications;
pub mod direct_discount;
pub mod positional_discount;

new_key_type! {
    /// Promotion Key
    pub struct PromotionKey;
}

/// Promotion metadata
#[derive(Debug)]
pub struct PromotionMeta {
    /// Promotion name
    pub name: String,
}

/// Promotion enum
#[derive(Debug, Clone)]
pub enum Promotion<'a> {
    /// Direct Discount Promotion
    DirectDiscount(DirectDiscountPromotion<'a>),

    /// Positional Discount
    PositionalDiscount(PositionalDiscountPromotion<'a>),
}

impl Promotion<'_> {
    /// Return the promotion key.
    pub fn key(&self) -> PromotionKey {
        match self {
            Promotion::DirectDiscount(direct_discount) => direct_discount.key(),
            Promotion::PositionalDiscount(positional_discount) => positional_discount.key(),
        }
    }

    /// Return whether this promotion _might_ apply to the given item group.
    pub fn is_applicable(&self, item_group: &ItemGroup<'_>) -> bool {
        match self {
            Promotion::DirectDiscount(direct_discount) => direct_discount.is_applicable(item_group),
            Promotion::PositionalDiscount(promotion) => promotion.is_applicable(item_group),
        }
    }
}

#[cfg(test)]
mod tests {
    use rusty_money::{Money, iso};
    use slotmap::SlotMap;
    use smallvec::SmallVec;

    use crate::{
        discounts::SimpleDiscount,
        items::groups::ItemGroup,
        promotions::direct_discount::DirectDiscountPromotion,
        promotions::positional_discount::PositionalDiscountPromotion,
        tags::{collection::TagCollection, string::StringTagCollection},
    };

    use super::*;

    #[test]
    fn key_delegates_to_inner_promotion_key() {
        let item_group: ItemGroup<'_> = ItemGroup::new(SmallVec::new(), iso::GBP);

        // Generate a non-default promotion key so returning `Default::default()` is detectable.
        let mut keys = SlotMap::<PromotionKey, ()>::with_key();
        let key = keys.insert(());

        let inner = DirectDiscountPromotion::new(
            key,
            StringTagCollection::empty(),
            SimpleDiscount::AmountOverride(Money::from_minor(50, iso::GBP)),
        );

        let promo = Promotion::DirectDiscount(inner);

        assert_eq!(promo.key(), key);
        assert_ne!(promo.key(), PromotionKey::default());

        // Also smoke that this promo is "well-formed" for other calls.
        let _ = promo.is_applicable(&item_group);
    }

    #[test]
    fn is_applicable_delegates_to_inner_promotion() {
        // An empty item set should not be considered applicable; this ensures
        // `Promotion::is_applicable` doesn't accidentally short-circuit to `true`.
        let item_group: ItemGroup<'_> = ItemGroup::new(SmallVec::new(), iso::GBP);

        let inner = DirectDiscountPromotion::new(
            PromotionKey::default(),
            StringTagCollection::empty(),
            SimpleDiscount::AmountOverride(Money::from_minor(50, iso::GBP)),
        );

        let promo = Promotion::DirectDiscount(inner);

        assert!(!promo.is_applicable(&item_group));
    }

    #[test]
    fn key_delegates_to_positional_promotion() {
        let mut keys = SlotMap::<PromotionKey, ()>::with_key();
        let key = keys.insert(());

        let inner = PositionalDiscountPromotion::new(
            key,
            StringTagCollection::empty(),
            2,
            SmallVec::from_vec(vec![1u16]),
            SimpleDiscount::AmountOff(Money::from_minor(50, iso::GBP)),
        );

        let promo = Promotion::PositionalDiscount(inner);

        assert_eq!(promo.key(), key);
        assert_ne!(promo.key(), PromotionKey::default());
    }

    #[test]
    fn is_applicable_handles_positional_discount_tags() {
        let items: SmallVec<[crate::items::Item<'_>; 10]> =
            SmallVec::from_vec(vec![crate::items::Item::with_tags(
                crate::products::ProductKey::default(),
                Money::from_minor(100, iso::GBP),
                StringTagCollection::from_strs(&["fresh"]),
            )]);
        let item_group: ItemGroup<'_> = ItemGroup::new(items, iso::GBP);

        let inner = PositionalDiscountPromotion::new(
            PromotionKey::default(),
            StringTagCollection::from_strs(&["fresh"]),
            2,
            SmallVec::from_vec(vec![1u16]),
            SimpleDiscount::AmountOff(Money::from_minor(10, iso::GBP)),
        );

        let promo = Promotion::PositionalDiscount(inner);

        assert!(promo.is_applicable(&item_group));
    }
}
