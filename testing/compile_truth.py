import sys
import os 
import subprocess
import re

if len(sys.argv) != 2:
    print("Please specify a path to your platforms NANORGs binary")
    print("compile_truth.py only accepts a single argument")
    exit()

nanorgs_path = sys.argv[1]

for asm_file in os.listdir("test_files_asm"):
    asm_file_path = "test_files_asm/" + asm_file

    error_count = 0

    while True:
        out = subprocess.run([nanorgs_path, "-z:" + asm_file_path], capture_output=True, text=True)
        
        line_num_str = re.findall(r'\d+', out.stdout)
        line_nums = list(map(int, line_num_str))
        
        if len(line_nums) != 1:
            break

        line_num = line_nums[0]
        
        lines = []

        with open(asm_file_path, "r") as file:
            lines = file.readlines()
            lines[line_num - 1] = ";" + lines[line_num - 1]


        with open(asm_file_path, "w") as file:
            file.writelines(lines) 

        error_count += 1

    header = "[" + asm_file + "]"
    print(header + " " * (20 - len(header)) + "Fixed: " + str(error_count) + " errors")


for asm_file in os.listdir("test_files_asm"):
    asm_file_path = "test_files_asm/" + asm_file
    out = subprocess.run([nanorgs_path, "-z:" + asm_file_path], stdout=subprocess.PIPE)
   
    lines = []

    for line in out.stdout.decode().splitlines()[2:-1]:
        lines.append(line[38:-1])

    with open("truth/" + asm_file.replace(".asm", ".txt"), "w") as file:
        lines = "\n".join(lines)
        lines += "\n"
        file.write(lines)
