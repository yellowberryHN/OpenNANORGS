# Testing
`test.py` generates all valid variants of operand for each
instruction, and then compares the bytecode output of OpenNANORGS
with the orginal compiler.

Due to the limit of 1200 instructions per (or, more accurately, 3600 words) bot, `test.py` automatically
splits test cases into multiple files and checks their validity sequentially.
