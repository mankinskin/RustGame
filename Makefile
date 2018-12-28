
.PHONY: run, clean

all:
	rustc src/main.rs

run: ./main
	./main

clean:
	rm -f ./main
