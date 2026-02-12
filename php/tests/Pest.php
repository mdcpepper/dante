<?php

declare(strict_types=1);

function assertLatticeExtensionLoaded(): void
{
    if (extension_loaded("lattice-php-ext")) {
        return;
    }

    throw new RuntimeException(
        "The lattice-php-ext extension is not loaded. Run tests with: " .
            "php -d extension=../target/debug/liblattice_php_ext.so vendor/bin/pest",
    );
}
