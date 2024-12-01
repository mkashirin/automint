#!usr/bin/bash

# Build to check that everything is OK, and for the "target/deploy" directory
# to be generated
cargo build-sbf

# Dump the Metaplex Metadata program to the fixtures directory, so that it can
# be added to the test environment
if [ ! -f "/target/deploy/mpl_metadata_program.so" ]; then
    solana program dump \
        --url="mainnet-beta" \
        "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s" \
        "./target/deploy/mpl_metadata_program.so"
fi

# Now, the tests can be ran
cargo test-sbf
