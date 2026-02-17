<?php

declare(strict_types=1);

use Lattice\Discount\Percentage;
use Lattice\Layer;
use Lattice\LayerOutput;
use Lattice\Discount\SimpleDiscount;
use Lattice\Item;
use Lattice\Money;
use Lattice\Product;
use Lattice\Promotions\Budget;
use Lattice\Promotions\DirectDiscountPromotion;
use Lattice\Promotions\Promotion;
use Lattice\Qualification;
use Lattice\StackBuilder;

it("implements Promotion interface", function () {
    $promotion = new DirectDiscountPromotion(
        reference: 123,
        qualification: Qualification::matchAll(),
        discount: SimpleDiscount::amountOff(new Money(123, "GBP")),
        budget: Budget::unlimited(),
    );

    expect($promotion)->toBeInstanceOf(Promotion::class);
});

it("can be instantiated", function () {
    $promotion = new DirectDiscountPromotion(
        reference: 123,
        qualification: Qualification::matchAll(),
        discount: SimpleDiscount::amountOff(new Money(123, "GBP")),
        budget: Budget::unlimited(),
    );

    expect($promotion->reference)->toBe(123);
    expect($promotion->discount)->toBeInstanceOf(SimpleDiscount::class);
    expect($promotion->discount->amount)->toEqual(new Money(1_23, "GBP"));
    expect($promotion->budget->redemptionLimit)->toBeNull();
    expect($promotion->budget->monetaryLimit)->toBeNull();
});

it("applies discount correctly", function () {
    $item = Item::fromProduct(
        reference: "item",
        product: new Product(
            reference: "product",
            name: "Product",
            price: new Money(3_00, "GBP"),
            tags: [],
        ),
    );

    $promotion = new DirectDiscountPromotion(
        reference: "promotion",
        qualification: Qualification::matchAll(),
        discount: SimpleDiscount::percentageOff(Percentage::fromDecimal(0.1)),
        budget: Budget::unlimited(),
    );

    $stack = new StackBuilder();

    $stack->addLayer(
        new Layer(
            reference: "layer",
            output: LayerOutput::passThrough(),
            promotions: [$promotion],
        ),
    );

    $receipt = $stack->build()->process([$item]);

    expect($receipt->subtotal)->toEqual(new Money(3_00, "GBP"));
    expect($receipt->total)->toEqual(new Money(2_70, "GBP"));
});
