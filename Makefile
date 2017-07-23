.PHONY: test new

test: 
	cargo run < ./input.txt | diff - ./output.txt
	@echo "SUCCESS"

new: $(name)

$(name):
	cargo new $(name) --bin
	@cd $(name); ln -s ../Makefile 




	