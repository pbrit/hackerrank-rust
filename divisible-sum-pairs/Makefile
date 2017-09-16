.PHONY: test

test:  
	@gecho -e "\e[33mTEST: \e[34minput00.txt\e[39m"
	cargo run < tests/input/input00.txt | diff - tests/output/output00.txt
	@gecho -e "\e[32mSUCCESS: \e[34minput00.txt\e[39m"

	@gecho -e "\e[33mTEST: \e[34minput12.txt\e[39m"
	cargo run < tests/input/input12.txt | diff - tests/output/output12.txt
	@gecho -e "\e[32mSUCCESS: \e[34minput12.txt\e[39m"

