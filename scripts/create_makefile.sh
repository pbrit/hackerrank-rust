#!/bin/bash 

cat <<EOF > Makefile
.PHONY: test

test:  
EOF

for test_file in tests/input/*; do 
test_name=$(basename $test_file)

cat <<EOF >> Makefile
	@gecho -e "\e[33mTEST: \e[34m${test_name}\e[39m"
	cargo run < tests/input/${test_name} | diff - tests/output/output${test_name##input}
	@gecho -e "\e[32mSUCCESS: \e[34m${test_name}\e[39m"

EOF
done