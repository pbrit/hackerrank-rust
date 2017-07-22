.PHONY: test  

test: 
	cargo run < ./input.txt | diff - ./output.txt
	@echo "SUCCESS"

	