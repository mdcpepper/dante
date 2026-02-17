//! Promotion Redemptions

use ext_php_rs::{
    class::RegisteredClass,
    convert::{FromZval, IntoZval},
    flags::DataType,
    prelude::*,
    types::Zval,
};

use crate::{items::ItemRef, money::MoneyRef, promotions::interface::PromotionRef};

#[derive(Debug, Clone)]
#[php_class]
#[php(name = "Lattice\\PromotionRedemption")]
pub struct PromotionRedemption {
    #[php(prop)]
    promotion: PromotionRef,

    #[php(prop)]
    item: ItemRef,

    #[php(prop)]
    redemption_idx: usize,

    #[php(prop)]
    original_price: MoneyRef,

    #[php(prop)]
    final_price: MoneyRef,
}

#[php_impl]
impl PromotionRedemption {
    pub fn __construct(
        promotion: PromotionRef,
        item: ItemRef,
        redemption_idx: usize,
        original_price: MoneyRef,
        final_price: MoneyRef,
    ) -> Self {
        Self {
            promotion,
            item,
            redemption_idx,
            original_price,
            final_price,
        }
    }
}

#[derive(Debug)]
pub struct PromotionRedemptionRef(Zval);

impl PromotionRedemptionRef {
    pub fn from_redemption(redemption: PromotionRedemption) -> Self {
        let mut zv = Zval::new();

        redemption
            .set_zval(&mut zv, false)
            .expect("promotion redemption should always convert to object zval");

        Self(zv)
    }
}

impl<'a> FromZval<'a> for PromotionRedemptionRef {
    const TYPE: DataType =
        DataType::Object(Some(<PromotionRedemption as RegisteredClass>::CLASS_NAME));

    fn from_zval(zval: &'a Zval) -> Option<Self> {
        let obj = zval.object()?;

        if obj.is_instance::<PromotionRedemption>() {
            Some(Self(zval.shallow_clone()))
        } else {
            None
        }
    }
}

impl Clone for PromotionRedemptionRef {
    fn clone(&self) -> Self {
        Self(self.0.shallow_clone())
    }
}

impl IntoZval for PromotionRedemptionRef {
    const TYPE: DataType =
        DataType::Object(Some(<PromotionRedemption as RegisteredClass>::CLASS_NAME));
    const NULLABLE: bool = false;

    fn set_zval(self, zv: &mut Zval, persistent: bool) -> ext_php_rs::error::Result<()> {
        self.0.set_zval(zv, persistent)
    }
}

impl TryFrom<&PromotionRedemptionRef> for PromotionRedemption {
    type Error = PhpException;

    fn try_from(value: &PromotionRedemptionRef) -> Result<Self, Self::Error> {
        let Some(obj) = value.0.object() else {
            return Err(PhpException::default(
                "PromotionRedemption object is invalid.".to_string(),
            ));
        };

        let item = obj.get_property::<ItemRef>("item").map_err(|_| {
            PhpException::default("PromotionRedemption item is invalid.".to_string())
        })?;

        let promotion = obj.get_property::<PromotionRef>("promotion").map_err(|_| {
            PhpException::default("PromotionRedemption promotion is invalid.".to_string())
        })?;

        let redemption_idx = obj.get_property::<usize>("redemption_idx").map_err(|_| {
            PhpException::default("PromotionRedemption redemption_idx is invalid.".to_string())
        })?;

        let original_price = obj
            .get_property::<MoneyRef>("original_price")
            .map_err(|_| {
                PhpException::default("PromotionRedemption original_price is invalid.".to_string())
            })?;

        let final_price = obj.get_property::<MoneyRef>("final_price").map_err(|_| {
            PhpException::default("PromotionRedemption final_price is invalid.".to_string())
        })?;

        Ok(Self {
            promotion,
            item,
            redemption_idx,
            original_price,
            final_price,
        })
    }
}

impl TryFrom<PromotionRedemptionRef> for PromotionRedemption {
    type Error = PhpException;

    fn try_from(value: PromotionRedemptionRef) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}
