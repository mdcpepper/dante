<?php

declare(strict_types=1);

use FeedCode\Lattice\Item;
use FeedCode\Lattice\Product;

it("creates a product with expected properties", function (): void {
    assertLatticeExtensionLoaded();

    $product = new Product(1, "Test Product", 123);

    expect($product)->toBeInstanceOf(Product::class);
    expect($product->reference)->toBe(1);
    expect($product->name)->toBe("Test Product");
    expect($product->price)->toBe(123);
});

it(
    "creates an item that points to the same product instance",
    function (): void {
        assertLatticeExtensionLoaded();

        $product = new Product(1, "Test Product", 123);
        $item = new Item(2, "Test Item", 123, $product);

        expect($item)->toBeInstanceOf(Item::class);
        expect($item->id)->toBe(2);
        expect($item->name)->toBe("Test Item");
        expect($item->price)->toBe(123);
        expect($item->product)->toBeInstanceOf(Product::class);
        expect(spl_object_id($item->product))->toBe(spl_object_id($product));
    },
);

it(
    "builds an item from product with mixed reference values",
    function (): void {
        assertLatticeExtensionLoaded();

        $productReference = (object) ["sku" => "ABC-123"];
        $itemReference = ["external_item_id" => 99];

        $product = new Product($productReference, "Test Product", 123);
        $item = Item::from_product($itemReference, $product);

        expect($item->id)->toBe($itemReference);
        expect($item->name)->toBe("Test Product");
        expect($item->price)->toBe(123);
        expect($item->product)->toBeInstanceOf(Product::class);
        expect($item->product->reference)->toBe($productReference);
    },
);
