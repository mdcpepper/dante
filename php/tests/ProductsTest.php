<?php

declare(strict_types=1);

use FeedCode\Lattice\Product;

it("creates a product with expected properties", function (): void {
    assertLatticeExtensionLoaded();

    $product = new Product(1, "Test Product", 123, ["test-tag"]);

    expect($product)->toBeInstanceOf(Product::class);
    expect($product->reference)->toBe(1);
    expect($product->name)->toBe("Test Product");
    expect($product->tags)->toBe(["test-tag"]);
    expect($product->price)->toBe(123);
});

it("removes duplicate product tags", function (): void {
    assertLatticeExtensionLoaded();

    $product = new Product(1, "Test Product", 123, ["test-tag", "test-tag"]);
    expect($product->tags)->toBe(["test-tag"]);
});
