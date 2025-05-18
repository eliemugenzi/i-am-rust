build:
	rustc src/formatted-print/debug.rs -o output/formatted-print-debug
	rustc src/formatted-print/index.rs -o output/formatted-print
	rustc src/formatted-print/display.rs -o output/formatted-print-display
run:
	output/formatted-print
clean:
	rm -f output/formatted-print-debug
	rm -f output/formatted-print
	rm -f output/formatted-print-display

run-display:
	output/formatted-print-display
.PHONY: format run clean