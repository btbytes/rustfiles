main:
	rustc main.rs -o program

run:
	./program

clean:
	rm -f program
	rm -rf program.dSYM

.PHONY: clean
