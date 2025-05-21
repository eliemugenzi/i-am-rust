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
	rustc src/types/casting.rs -o output/casting
	rustc src/types/literals.rs -o output/literals
	rustc src/types/inference.rs -o output/inference
	rustc src/types/aliasing.rs -o output/aliasing
	rustc src/conversion/conversion.rs -o output/conversion
	rustc src/conversion/try-from.rs -o output/try-from
	rustc src/flow-of-control/if-else.rs -o output/if-else
	rustc src/flow-of-control/loop.rs -o output/loop
	rustc src/flow-of-control/match.rs -o output/match
	rustc src/flow-of-control/if-let.rs -o output/if-let
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
casting:
	output/casting
literals:
	output/literals
inference:
	output/inference
aliasing:	
	output/aliasing
conversion:
	output/conversion
try-from:
	output/try-from
if-else:
	output/if-else
loop:
	output/loop
match:
	output/match
if-let:
	output/if-let
clean:
	rm -f output/formatted-print-debug
	rm -f output/formatted-print
	rm -f output/formatted-print-display
	rm -f output/operators
	rm -f output/tuples
	rm -f output/arrays
	rm -f output/structures
	rm -f output/enums
	rm -f output/constants
	rm -f output/mutability
	rm -f output/scope-and-shadowing
	rm -f output/casting
	rm -f output/literals

run-display:
	output/formatted-print-display
