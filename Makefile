.PHONY: test new input output

test: 
	cargo run < ./input.txt | diff - ./output.txt
	@echo "SUCCESS"

new: $(name)

input: input.txt

output: output.txt

$(name):
	cargo new $(name) --bin
	@cd $(name); ln -s ../Makefile 

input.txt: 
	pbpaste > input.txt
	echo >> input.txt

output.txt:
	pbpaste > output.txt
	echo >> output.txt
	