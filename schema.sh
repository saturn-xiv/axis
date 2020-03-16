#!/bin/sh

DATABASE_URL="var/db" diesel print-schema > src/orm/schema.rs