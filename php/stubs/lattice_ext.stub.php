<?php
declare(strict_types=1);

namespace FeedCode\Lattice;

if (!class_exists(Product::class)) {
    class Product
    {
        public mixed $reference;
        public string $name;
        public int $price;
        public function __construct(
            mixed $reference,
            string $name,
            int $price,
        ) {}
    }
}

if (!class_exists(Item::class)) {
    class Item
    {
        public mixed $id;
        public string $name;
        public int $price;
        public Product $product;
        public function __construct(
            mixed $id,
            string $name,
            int $price,
            Product $product,
        ) {}
        public static function from_product(
            mixed $reference,
            Product $product,
        ): self {}
    }
}
