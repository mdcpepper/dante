set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

ext_so := "target/debug/liblattice_php_ext.so"

default: test-extension

build-extension:
    cd crates/php-ext && cargo build

test-extension: build-extension
    cd php && php -d extension=../{{ ext_so }} vendor/bin/pest --configuration=phpunit.xml
