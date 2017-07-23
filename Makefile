.PHONY: test new input output

test: 
	cargo run < ./input.txt | diff - ./output.txt
	@echo "SUCCESS"

new: $(name) $(name)/tests $(name)/Makefile

$(name):
	cargo new $(name) --bin
	
$(name)/tests: $(name)
	mkdir $(name)/tests
	cd $(name)/tests; curl https://www.hackerrank.com/rest/contests/master/challenges/$(name)/download_testcases > tests.zip
	cd $(name)/tests; unzip tests.zip; rm -f tests.zip

$(name)/Makefile: $(name) $(name)/tests
	cd $(name); ../scripts/create_makefile.sh  
	