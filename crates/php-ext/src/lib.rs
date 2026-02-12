#![cfg_attr(windows, feature(abi_vectorcall))]

use ext_php_rs::prelude::*;

use crate::{items::Item, products::Product};

pub mod items;
pub mod products;
pub mod reference_value;

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.class::<Product>().class::<Item>()
}
