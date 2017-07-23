.PHONY: test new input output commit

test: 
	cargo run < ./input.txt | diff - ./output.txt
	@echo "SUCCESS"

new: $(name) $(name)/tests $(name)/Makefile commit

commit: $(name) $(name)/tests $(name)/Makefile
	git reset
	git add $(name)
	git commit -m "Init $(name)"

$(name):
	cargo new $(name) --bin
	@wget https://www.hackerrank.com/rest/contests/master/challenges/$(name)/download_pdf?language=English -O $(name)/statement.pdf 2>/dev/null
	
$(name)/tests: $(name)
	mkdir $(name)/tests
	cd $(name)/tests; curl https://www.hackerrank.com/rest/contests/master/challenges/$(name)/download_testcases > tests.zip 2>/dev/null
	cd $(name)/tests; unzip tests.zip; rm -f tests.zip

$(name)/Makefile: $(name) $(name)/tests
	cd $(name); ../scripts/create_makefile.sh  
	