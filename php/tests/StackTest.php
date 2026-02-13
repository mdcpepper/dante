<?php

declare(strict_types=1);

use FeedCode\Lattice\Discount\SimpleDiscount;
use FeedCode\Lattice\Layer;
use FeedCode\Lattice\LayerOutput;
use FeedCode\Lattice\Money;
use FeedCode\Lattice\Promotions\Budget;
use FeedCode\Lattice\Promotions\DirectDiscount;
use FeedCode\Lattice\Qualification;
use FeedCode\Lattice\Stack;
use FeedCode\Lattice\Stack\InvalidStackException;

it("validates a linear stack as a promotion graph", function (): void {
    assertLatticeExtensionLoaded();

    $promotion = new DirectDiscount(
        key: "promo-1",
        qualification: Qualification::matchAny(["food"]),
        discount: SimpleDiscount::amountOff(new Money(25, "GBP")),
        budget: Budget::unlimited(),
    );

    $stack = new Stack([
        new Layer(
            key: "layer-1",
            output: LayerOutput::PassThrough,
            promotions: [$promotion],
        ),
    ]);

    expect($stack->validateGraph())->toBeTrue();
});

it("throws when validating an empty stack", function (): void {
    assertLatticeExtensionLoaded();

    $stack = new Stack();

    $thrown = null;

    try {
        $stack->validateGraph();
    } catch (Throwable $error) {
        $thrown = $error;
    }

    expect($thrown)->toBeInstanceOf(InvalidStackException::class);
    expect($thrown?->getMessage())->toContain("at least one layer");
});

it(
    "throws when a layer uses split output in linear stack mode",
    function (): void {
        assertLatticeExtensionLoaded();

        $stack = new Stack([
            new Layer(key: "split-layer", output: LayerOutput::Split),
        ]);

        $thrown = null;

        try {
            $stack->validateGraph();
        } catch (Throwable $error) {
            $thrown = $error;
        }

        expect($thrown)->toBeInstanceOf(InvalidStackException::class);
        expect($thrown?->getMessage())->toContain("LayerOutput::Split");
    },
);
