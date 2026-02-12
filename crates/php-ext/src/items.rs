//! Items

use ext_php_rs::prelude::*;

use crate::{products::ProductRef, reference_value::ReferenceValue};

#[derive(Debug)]
#[php_class]
#[php(name = "FeedCode\\Lattice\\Item")]
pub struct Item {
    #[php(prop)]
    id: ReferenceValue,

    #[php(prop)]
    name: String,

    #[php(prop)]
    price: i64,

    #[php(prop)]
    product: ProductRef,
}

#[php_impl]
impl Item {
    pub fn __construct(id: ReferenceValue, name: String, price: i64, product: ProductRef) -> Self {
        Self {
            id,
            name,
            price,
            product,
        }
    }

    #[php(name = "from_product")]
    pub fn from_product(reference: ReferenceValue, product: ProductRef) -> Self {
        let name = product.name();
        let price = product.price();

        Self {
            id: reference,
            name,
            price,
            product,
        }
    }
}
