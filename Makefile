.PHONY: build run tuples operators arrays

build:
	rustc src/formatted-print/debug.rs -o output/formatted-print-debug
	rustc src/formatted-print/index.rs -o output/formatted-print
	rustc src/formatted-print/display.rs -o output/formatted-print-display
	rustc src/primitives/operators.rs -o output/operators
	rustc src/primitives/tuples.rs -o output/tuples
	rustc src/primitives/arrays.rs -o output/arrays
run:
	output/formatted-print
tuples:
	output/tuples
operators:
	output/operators
arrays:
	output/arrays
clean:
	rm -f output/formatted-print-debug
	rm -f output/formatted-print
	rm -f output/formatted-print-display
	rm -f output/operators
	rm -f output/tuples
	rm -f output/arrays

run-display:
	output/formatted-print-display
