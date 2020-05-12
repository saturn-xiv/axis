#!/bin/sh

set -e

export DATABASE_URL="tmp/db"

diesel print-schema > src/orm/schema.rs
cargo fmt

exit 0
