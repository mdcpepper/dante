//! Items

use rusty_money::{Money, iso};

use crate::tags::{collection::TagCollection, string::StringTagCollection};

/// An unprocessed item with a price and tags.
#[derive(Clone, Debug, PartialEq)]
pub struct Item<'a, T: TagCollection = StringTagCollection> {
    price: Money<'a, iso::Currency>,
    tags: T,
}

impl<'a, T: TagCollection> Item<'a, T> {
    /// Creates a new item with the given price and empty tags.
    pub fn new(price: Money<'a, iso::Currency>) -> Self {
        Self::with_tags(price, T::empty())
    }

    /// Creates a new item with the given price and tags.
    pub fn with_tags(price: Money<'a, iso::Currency>, tags: T) -> Self {
        Self { price, tags }
    }

    /// Returns the price of the item
    pub fn price(&self) -> &Money<'a, iso::Currency> {
        &self.price
    }

    /// Returns the tags for the item.
    pub fn tags(&self) -> &T {
        &self.tags
    }

    /// Returns the tags for the item, mutably.
    pub fn tags_mut(&mut self) -> &mut T {
        &mut self.tags
    }
}

/// Returns the cheapest item in a list of items
pub fn cheapest_item<'a, T: TagCollection>(items: &'a [Item<'a, T>]) -> Option<&'a Item<'a, T>> {
    items
        .iter()
        .min_by_key(|item| item.price().to_minor_units())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cheapest_item() {
        let items: [Item<'_, StringTagCollection>; 2] = [
            Item::with_tags(
                Money::from_minor(100, iso::USD),
                StringTagCollection::empty(),
            ),
            Item::new(Money::from_minor(200, iso::USD)),
        ];

        let cheapest = cheapest_item(&items).expect("expected cheapest item");
        assert_eq!(cheapest.price(), &Money::from_minor(100, iso::USD));
    }

    #[test]
    fn item_tag_accessors_work() {
        let tags = StringTagCollection::from_strs(&["fresh"]);
        let mut item = Item::with_tags(Money::from_minor(150, iso::USD), tags);

        assert!(item.tags().contains("fresh"));

        item.tags_mut().add("sale");
        assert!(item.tags().contains("sale"));
    }
}
