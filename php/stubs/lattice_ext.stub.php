<?php
declare(strict_types=1);

namespace Lattice;

if (!class_exists(Money::class)) {
    class Money
    {
        public int $amount;

        public string $currency;

        public function __construct(int $amount, string $currency) {}

        public function currency(): string {}
    }
}

if (!class_exists(Product::class)) {
    class Product
    {
        public mixed $reference;

        public string $name;

        public Money $price;

        /** @var string[] */
        public array $tags;

        /**
         * @param string[]|null $tags
         */
        public function __construct(
            mixed $reference,
            string $name,
            Money $price,
            ?array $tags = [],
        ) {}
    }
}

if (!class_exists(Item::class)) {
    class Item
    {
        public mixed $reference;

        public string $name;

        public Money $price;

        public Product $product;

        /** @var string[] */
        public array $tags;

        /**
         * @param string[]|null $tags
         */
        public function __construct(
            mixed $reference,
            string $name,
            Money $price,
            Product $product,
            ?array $tags = [],
        ) {}

        public static function fromProduct(
            mixed $reference,
            Product $product,
        ): self {}
    }
}

if (!class_exists(LayerOutput::class)) {
    class LayerOutput
    {
        private ?Layer $participating;

        private ?Layer $nonParticipating;

        public function __construct() {}

        public static function passThrough(): self {}

        public static function split(
            Layer $participating,
            Layer $nonParticipating,
        ): self {}
    }
}

if (!class_exists(Layer::class)) {
    class Layer
    {
        public mixed $reference;

        public LayerOutput $output;

        /** @var Promotions\Promotion[] */
        public array $promotions;

        /**
         * @param Promotions\Promotion[]|null $promotions
         */
        public function __construct(
            mixed $reference,
            LayerOutput $output,
            ?array $promotions = [],
        ) {}
    }
}

if (!class_exists(Stack::class)) {
    class Stack
    {
        /** @var Layer[] */
        public array $layers;

        /**
         * @param Layer[]|null $layers
         */
        public function __construct(?array $layers = []) {}

        public function validateGraph(): bool {}

        /**
         * @param Item[] $items
         */
        public function process(array $items): Receipt {}
    }
}

if (!class_exists(StackBuilder::class)) {
    class StackBuilder
    {
        /** @var Layer[] */
        public array $layers;

        public ?Layer $rootLayer;

        public function __construct() {}

        public function addLayer(Layer $layer): Layer {}

        public function setRoot(Layer $layer): void {}

        public function build(): Stack {}
    }
}

if (!class_exists(PromotionApplication::class)) {
    class PromotionApplication
    {
        public Promotions\Promotion $promotion;

        public Item $item;

        public int $bundleId;

        public Money $originalPrice;

        public Money $finalPrice;

        public function __construct(
            Promotions\Promotion $promotion,
            Item $item,
            int $bundle_id,
            Money $original_price,
            Money $final_price,
        ) {}
    }
}

if (!class_exists(Receipt::class)) {
    class Receipt
    {
        public Money $subtotal;

        public Money $total;

        /** @var Item[] */
        public array $fullPriceItems;

        /** @var PromotionApplication[] */
        public array $promotionApplications;

        /**
         * @param Item[] $full_price_items
         * @param PromotionApplication[] $promotion_applications
         */
        public function __construct(
            Money $subtotal,
            Money $total,
            array $full_price_items,
            array $promotion_applications,
        ) {}
    }
}

namespace Lattice\Stack;

if (!class_exists(InvalidStackException::class)) {
    class InvalidStackException extends \Exception {}
}

namespace Lattice;

if (!class_exists(Qualification::class)) {
    class Qualification
    {
        public Qualification\BoolOp $op;

        /** @var Qualification\Rule[] */
        public array $rules;

        /**
         * @param Qualification\Rule[]|null $rules
         */
        public function __construct(
            Qualification\BoolOp $op,
            ?array $rules = null,
        ) {}

        public static function matchAll(): self {}

        /**
         * @param string[]|null $tags
         */
        public static function matchAny(?array $tags = []): self {}

        /**
         * @param string[]|null $item_tags
         */
        public function matches(?array $item_tags = []): bool {}
    }
}

namespace Lattice\Qualification;

if (!class_exists(BoolOp::class)) {
    enum BoolOp: string
    {
        case AndOp = "and";
        case OrOp = "or";
    }
}

if (!class_exists(RuleKind::class)) {
    enum RuleKind: string
    {
        case HasAll = "has_all";
        case HasAny = "has_any";
        case HasNone = "has_none";
        case Group = "group";
    }
}

if (!class_exists(Rule::class)) {
    class Rule
    {
        public RuleKind $kind;

        /** @var string[] */
        public array $tags;

        public ?\Lattice\Qualification $group;

        public function __construct() {}

        /**
         * @param string[]|null $tags
         */
        public static function hasAll(?array $tags = []): self {}

        /**
         * @param string[]|null $tags
         */
        public static function hasAny(?array $tags = []): self {}

        /**
         * @param string[]|null $tags
         */
        public static function hasNone(?array $tags = []): self {}

        public static function group(
            \Lattice\Qualification $qualification,
        ): self {}

        /**
         * @param string[]|null $item_tags
         */
        public function matches(?array $item_tags = []): bool {}
    }
}

namespace Lattice\Discount;

use Lattice\Money;

if (!class_exists(InvalidPercentageException::class)) {
    class InvalidPercentageException extends \Exception {}
}

if (!class_exists(PercentageOutOfRangeException::class)) {
    class PercentageOutOfRangeException extends \Exception {}
}

if (!class_exists(InvalidDiscountException::class)) {
    class InvalidDiscountException extends \Exception {}
}

if (!class_exists(Percentage::class)) {
    class Percentage
    {
        public readonly float $value;

        public function __construct(string $value) {}

        public static function fromDecimal(float $value): self {}

        public static function fromNormalized(float $value): self {}

        public static function validateNormalized(float $value): void {}

        public function value(): float {}
    }
}

if (!enum_exists(DiscountKind::class)) {
    enum DiscountKind: string
    {
        case PercentageOff = "percentage_off";
        case AmountOverride = "amount_override";
        case AmountOff = "amount_off";
    }
}

if (!class_exists(SimpleDiscount::class)) {
    class SimpleDiscount
    {
        public DiscountKind $kind;

        public ?Percentage $percentage;

        public ?Money $amount;

        public function __construct() {}

        public static function percentageOff(Percentage $percentage): self {}

        public static function amountOverride(Money $amount): self {}

        public static function amountOff(Money $amount): self {}
    }
}

namespace Lattice\Promotions;

use Lattice\Discount\SimpleDiscount;
use Lattice\Money;
use Lattice\Qualification;
use Lattice\Promotions\MixAndMatch\Discount as MixAndMatchDiscountConfig;
use Lattice\Promotions\MixAndMatch\Slot as MixAndMatchSlot;

if (!class_exists(Budget::class)) {
    class Budget
    {
        public ?int $applicationLimit;

        public ?Money $monetaryLimit;

        public function __construct() {}

        public static function unlimited(): self {}

        public static function withApplicationLimit(int $limit): self {}

        public static function withMonetaryLimit(Money $limit): self {}

        public static function withBothLimits(
            int $application,
            Money $monetary,
        ): self {}
    }
}

if (!interface_exists(Promotion::class)) {
    interface Promotion {}
}

if (!class_exists(DirectDiscount::class)) {
    class DirectDiscount implements Promotion
    {
        public mixed $reference;

        public Qualification $qualification;

        public SimpleDiscount $discount;

        public Budget $budget;

        public function __construct(
            mixed $reference,
            Qualification $qualification,
            SimpleDiscount $discount,
            Budget $budget,
        ) {}
    }
}

if (!class_exists(PositionalDiscount::class)) {
    class PositionalDiscount implements Promotion
    {
        public mixed $reference;

        public Qualification $qualification;

        public int $size;

        /** @var int[] */
        public array $positions;

        public SimpleDiscount $discount;

        public Budget $budget;

        /**
         * @param int[] $positions
         */
        public function __construct(
            mixed $reference,
            int $size,
            array $positions,
            Qualification $qualification,
            SimpleDiscount $discount,
            Budget $budget,
        ) {}
    }
}

if (!class_exists(MixAndMatchDiscount::class)) {
    class MixAndMatchDiscount implements Promotion
    {
        public mixed $reference;

        /** @var MixAndMatchSlot[] */
        public array $slots;

        public MixAndMatchDiscountConfig $discount;

        public Budget $budget;

        /**
         * @param MixAndMatchSlot[] $slots
         */
        public function __construct(
            mixed $reference,
            array $slots,
            MixAndMatchDiscount $discount,
            Budget $budget,
        ) {}
    }
}

namespace Lattice\Promotions\MixAndMatch;

use Lattice\Discount\Percentage;
use Lattice\Money;
use Lattice\Qualification;

if (!enum_exists(DiscountKind::class)) {
    enum DiscountKind: string
    {
        case PercentageOffAllItems = "percentage_off_all_items";
        case AmountOffEachItem = "amount_off_each_item";
        case OverrideEachItem = "override_each_item";
        case AmountOffTotal = "amount_off_total";
        case OverrideTotal = "override_total";
        case PercentageOffCheapest = "percentage_off_cheapest";
        case OverrideCheapest = "override_cheapest";
    }
}

if (!class_exists(Discount::class)) {
    class Discount
    {
        public DiscountKind $kind;

        public ?Percentage $percentage;

        public ?Money $amount;

        public function __construct() {}

        public static function percentageOffAllItems(
            Percentage $percentage,
        ): self {}

        public static function amountOffEachItem(Money $amount): self {}

        public static function overrideEachItem(Money $amount): self {}

        public static function amountOffTotal(Money $amount): self {}

        public static function overrideTotal(Money $amount): self {}

        public static function percentageOffCheapest(
            Percentage $percentage,
        ): self {}

        public static function overrideCheapest(Money $amount): self {}
    }
}

if (!class_exists(Slot::class)) {
    class Slot
    {
        public mixed $reference;

        public Qualification $qualification;

        public int $min;

        public ?int $max;

        public function __construct(
            mixed $reference,
            Qualification $qualification,
            int $min,
            ?int $max = null,
        ) {}
    }
}
