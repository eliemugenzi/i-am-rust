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
	rustc src/flow-of-control/functions.rs -o output/functions
	rustc src/flow-of-control/closures.rs -o output/closures
	rustc src/functions/higher_order_functions.rs -o output/higher_order_functions
	rustc src/functions/diverging_functions.rs -o output/diverging_functions
	rustc src/modules/visibility.rs -o output/visibility
	rustc src/generics/functions.rs -o output/generic-functions
	rustc src/generics/traits.rs -o output/traits
	rustc src/generics/bounds.rs -o output/bounds
	rustc src/generics/new_type_idiom.rs -o output/new-type-idiom
	rustc src/generics/associated_items.rs -o output/associated_items
	rustc src/scoping-rules/ownership_and_moves.rs -o output/ownership_and_moves
	rustc src/scoping-rules/borrowing.rs -o output/borrowing
	rustc src/scoping-rules/the_ref_pattern.rs -o output/the_ref_pattern
	rustc src/scoping-rules/lifetimes.rs -o output/lifetimes
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
functions:
	output/functions
closures:
	output/closures
higher_order_functions:
	output/higher_order_functions
diverging_functions:
	outout/diverging_functions
visibility:
	output/visibility
generic-functions:
	outout/generic-functions
clean:
	rm -f output/*

run-display:
	output/formatted-print-display
ownership_and_moves:
	output/ownership_and_moves
borrowing:
	output/borrowing
the_ref_pattern:
	output/the_ref_pattern
lifetimes:
	output/lifetimes