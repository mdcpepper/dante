//! Receipt and promotion redemption results

use ext_php_rs::prelude::*;

use crate::{items::ItemRef, money::MoneyRef, receipt::redemptions::PromotionRedemptionRef};

pub mod redemptions;

#[derive(Debug, Clone)]
#[php_class]
#[php(name = "Lattice\\Receipt")]
pub struct Receipt {
    #[php(prop)]
    subtotal: MoneyRef,

    #[php(prop)]
    total: MoneyRef,

    #[php(prop)]
    full_price_items: Vec<ItemRef>,

    #[php(prop)]
    promotion_redemptions: Vec<PromotionRedemptionRef>,
}

#[php_impl]
impl Receipt {
    pub fn __construct(
        subtotal: MoneyRef,
        total: MoneyRef,
        full_price_items: Vec<ItemRef>,
        promotion_redemptions: Vec<PromotionRedemptionRef>,
    ) -> Self {
        Self {
            subtotal,
            total,
            full_price_items,
            promotion_redemptions,
        }
    }
}
