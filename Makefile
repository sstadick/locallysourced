
build: ./bin transpose

bin:
	mkdir ./bin

transpose: ./src/transpose/src/main.rs
	cd ./src/transpose && cargo build --release
	cp ./src/transpose/target/release/transpose ./bin/
