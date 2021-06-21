.PHONY: test build

build:
	-rm -rf emoji_images/twemoji_graphics.satyh
	rustc make_satyh.rs
	./make_satyh

test:
	rustc make_test.rs
	./make_test
	satysfi test/test.saty -o test/test.pdf
	satysfi test/all.saty -o test/all.pdf