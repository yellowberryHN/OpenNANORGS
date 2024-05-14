# Testing

## Generate  
`generate_asm.py` generates all valid variants of operand for each
instruction, and then compares the bytecode output of OpenNANORGS
with the orginal compiler.

Due to the limit of 1200 instructions per (or, more accurately, 3600 words) bot, `generate_asm.py` automatically
splits test cases into multiple files based on instruction name.

**GENERATE.PY IS NOT COMPLETE AND PRODUCED ASM FILES REQUIRE MODIFICATION**

## Compile Truth
`compile_truth.py` accepts a path to your platforms NANORGs binary as the first argument,
and compiles each assembly file produced from `generate_asm.py`. `compile_truth.py` also iteratively
removes problematic instructions incorrectly generated from `validate.py`.

## Validate
`validate.py` compiles all generated assembly, and compares the bytecode output with the source
of truth located in `truth/`.

# Usage
It is expected that the provided scripts are ran in the following order:
1. `generate_asm.py`
2. `compile_truth.py`
3. `validate.py`
