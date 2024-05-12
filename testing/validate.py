import os
import subprocess

for asm_file in os.listdir("test_files_asm_fixed"):
    subprocess.run(["cargo", "run", "--", "--dump-bytecode-text", "test_files_asm_fixed/" + asm_file], stdout=subprocess.DEVNULL, stderr=subprocess.STDOUT) 

    output_file_name = "test_files_asm_fixed/" + asm_file + ".txt"
    output = ""

    with open(output_file_name) as output_file:
        output = output_file.read()

    os.remove(output_file_name)
    os.makedirs("us", exist_ok=True)

    with open("us/" + asm_file.replace(".asm", ".txt"), "w") as output_file:
        output_file.write(output)

