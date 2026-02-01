//! Items

use crate::prices::Price;

/// An unprocessed item
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Item {
    price: Price,
}

impl Item {
    /// Creates a new item with the given price
    pub fn new(price: Price) -> Self {
        Self { price }
    }

    /// Returns the price of the item
    pub fn price(&self) -> Price {
        self.price
    }
}

/// Calculates the total price of a list of items
pub fn total_price(items: &[Item]) -> Price {
    Price::new(items.iter().map(|item| *item.price()).sum())
}

/// Returns the cheapest item in a list of items
pub fn cheapest_item(items: &[Item]) -> Option<&Item> {
    items.iter().min_by(|a, b| a.price().cmp(&b.price()))
}

/// Returns an iterator over a list of items sorted by price in descending order
pub fn iter_by_price_desc<'a>(
    items: &'a [Item],
    idx: &'a mut [usize],
) -> impl Iterator<Item = &'a Item> {
    assert!(idx.len() >= items.len());

    // Initialize the index array with the indices of the items
    let idx = &mut idx[..items.len()];

    // Populate the scratch index buffer so we can sort by price without moving/cloning items.
    for (i, slot) in idx.iter_mut().enumerate() {
        *slot = i;
    }

    // Sort the indices by descending item price (unstable is fine since ties don't need a fixed order).
    idx.sort_unstable_by(|&i, &j| items[j].price().cmp(&items[i].price()));

    // Yield items in that sorted index order, borrowing from the original slice.
    idx.iter().map(move |&i| &items[i])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_price() {
        let items = [Item::new(Price::new(100)), Item::new(Price::new(200))];

        assert_eq!(total_price(&items), Price::new(300));
    }

    #[test]
    fn test_cheapest_item() {
        let item_1 = Item::new(Price::new(100));
        let item_2 = Item::new(Price::new(200));
        let items = [item_1, item_2];

        assert_eq!(cheapest_item(&items), Some(&item_1));
    }

    #[test]
    fn test_iter_by_price_desc() {
        let item_1 = Item::new(Price::new(100));
        let item_2 = Item::new(Price::new(300));
        let item_3 = Item::new(Price::new(200));
        let items = [item_1, item_2, item_3];

        let mut idx = [0usize; 3];
        let got: Vec<Price> = iter_by_price_desc(&items, &mut idx)
            .map(|item| item.price())
            .collect();

        assert_eq!(got, vec![Price::new(300), Price::new(200), Price::new(100)]);
    }
}
