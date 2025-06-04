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
	rustc src/scoping-rules/lifetime_structs.rs -o output/lifetime_structs
	rustc src/scoping-rules/lifetime_traits.rs -o output/lifetime_traits
	rustc src/scoping-rules/lifetime_bounds.rs -o output/lifetime_bounds
	rustc src/traits/derive.rs -o output/derive
	rustc src/traits/return_with_dyn.rs -o output/return_with_dyn
	rustc src/traits/operator_overloading.rs -o output/operator_overloading
	rustc src/traits/iterators.rs -o output/iterators
	rustc src/traits/impl_trait.rs -o output/impl_trait
	rustc src/traits/clone.rs -o output/clone
	rustc src/macro-rules/syntax.rs -o output/macro_rules_syntax
	rustc src/macro-rules/dsl.rs -o output/dsl
	rustc src/macro-rules/variadic_interfaces.rs -o output/variadic_interfaces
	rustc src/error-handling/panic.rs -o output/panic
	rustc src/std/box_stack_heap.rs -o output/box_stack_heap
	rustc src/std/vectors.rs -o output/vectors
	rustc src/std/strings.rs -o output/strings
	rustc src/std/options.rs -o output/options
	rustc src/std/result.rs -o output/result
	rustc src/std/result_alt.rs -o output/result_alt
	rustc src/std/hashmap.rs -o output/hashmap
	rustc src/std/rc.rs -o output/rc
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
lifetime_structs:
	output/lifetime_structs
lifetime_traits:
	output/lifetime_traits
lifetime_bounds:
	output/lifetime_bounds
derive:
	output/derive
return_with_dyn:
	output/return_with_dyn
operator_overloading:
	output/operator_overloading
iterators:
	output/iterators
impl_trait:
	output/impl_trait
clone:
	output/clone
macro_rules_syntax:
	output/macro_rules_syntax
dsl:
	output/dsl
variadic_interfaces:
	output/variadic_interfaces
panic:
	output/panic
box_stack_heap:
	output/box_stack_heap
vectors:
	output/vectors
strings:
	output/strings
options:
	output/options
result:
	output/result
result_alt:
	output/result_alt
hashmap:
	output/hashmap
rc:
	output/rc
