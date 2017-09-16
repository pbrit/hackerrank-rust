.PHONY: test

test:  
	@gecho -e "\e[33mTEST: \e[34minput00.txt\e[39m"
	cargo run < tests/input/input00.txt | diff - tests/output/output00.txt
	@gecho -e "\e[32mSUCCESS: \e[34minput00.txt\e[39m"

	@gecho -e "\e[33mTEST: \e[34minput01.txt\e[39m"
	cargo run < tests/input/input01.txt | diff - tests/output/output01.txt
	@gecho -e "\e[32mSUCCESS: \e[34minput01.txt\e[39m"

	@gecho -e "\e[33mTEST: \e[34minput02.txt\e[39m"
	cargo run < tests/input/input02.txt | diff - tests/output/output02.txt
	@gecho -e "\e[32mSUCCESS: \e[34minput02.txt\e[39m"

