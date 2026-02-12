<?php

declare(strict_types=1);

use FeedCode\Lattice\Product;
use FeedCode\Lattice\Item;

it(
    "creates an item that points to the same product instance",
    function (): void {
        assertLatticeExtensionLoaded();

        $product = new Product(1, "Test Product", 123, ["test-tag"]);
        $item = new Item(2, "Test Item", 123, $product);

        expect($item)->toBeInstanceOf(Item::class);
        expect($item->id)->toBe(2);
        expect($item->name)->toBe("Test Item");
        expect($item->price)->toBe(123);
        expect($item->tags)->toBe([]); // doesn't inherit tags automatically
        expect($item->product)->toBeInstanceOf(Product::class);
        expect(spl_object_id($item->product))->toBe(spl_object_id($product));
    },
);

it("builds an item from product", function (): void {
    assertLatticeExtensionLoaded();

    $productReference = (object) ["sku" => "ABC-123"];
    $itemReference = ["external_item_id" => 99];

    $product = new Product($productReference, "Test Product", 123, [
        "test-tag",
    ]);

    $item = Item::from_product($itemReference, $product);

    expect($item->id)->toBe($itemReference);
    expect($item->name)->toBe("Test Product");
    expect($item->price)->toBe(123);
    expect($item->tags)->toBe(["test-tag"]);
    expect($item->product)->toBeInstanceOf(Product::class);
    expect($item->product->reference)->toBe($productReference);
});

it("removes duplicate item tags", function (): void {
    assertLatticeExtensionLoaded();

    $product = new Product(1, "Test Product", 123);
    $item = new Item(1, "Test Item", 123, $product, ["test-tag", "test-tag"]);

    expect($item->tags)->toBe(["test-tag"]);
});
