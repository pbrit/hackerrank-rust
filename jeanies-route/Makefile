.PHONY: test

test:  
	@gecho -e "\e[33mTEST: \e[34m*\e[39m"
	cargo run < tests/input/* | diff - tests/output/output*
	@gecho -e "\e[32mSUCCESS: \e[34m*\e[39m"

