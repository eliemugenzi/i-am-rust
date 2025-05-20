.PHONY: build run tuples operators arrays

build:
	rustc src/formatted-print/debug.rs -o output/formatted-print-debug
	rustc src/formatted-print/index.rs -o output/formatted-print
	rustc src/formatted-print/display.rs -o output/formatted-print-display
	rustc src/primitives/operators.rs -o output/operators
	rustc src/primitives/tuples.rs -o output/tuples
	rustc src/primitives/arrays.rs -o output/arrays
	rustc src/custom-types/structures.rs -o output/structures
	rustc src/custom-types/enums.rs -o output/enums
	rustc src/custom-types/constants.rs -o output/constants
	rustc src/variable-bindings/mutability.rs -o output/mutability
	rustc src/variable-bindings/scope-and-shadowing.rs -o output/scope-and-shadowing
run:
	output/formatted-print
tuples:
	output/tuples
operators:
	output/operators
arrays:
	output/arrays
structures:
	output/structures
enums:
	output/enums
constants:
	output/constants
mutability:
	output/mutability
scope-and-shadowing:
	output/scope-and-shadowing
clean:
	rm -f output/formatted-print-debug
	rm -f output/formatted-print
	rm -f output/formatted-print-display
	rm -f output/operators
	rm -f output/tuples
	rm -f output/arrays

run-display:
	output/formatted-print-display
