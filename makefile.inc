main:
	rustc main.rs -o run

prog:
	./run

clean:
	rm -f run
	rm -rf run.dSYM

.PHONY: clean
