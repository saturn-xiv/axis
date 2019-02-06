dist=dist

all:
	cargo build --release
	strip -s target/release/axis
	mkdir -pv $(dist)
	-cp target/release/axis assets/* $(dist)

clean:
	-rm -r $(dist)
	cargo clean

schema:
	DATABASE_URL="tmp/db" diesel print-schema > src/orm/schema.rs