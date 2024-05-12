import os
import subprocess

filenames = []

for asm_file in os.listdir("test_files_asm_fixed"):
    subprocess.run(["cargo", "run", "--", "--dump-bytecode-text", "test_files_asm_fixed/" + asm_file], stdout=subprocess.DEVNULL, stderr=subprocess.STDOUT) 

    output_file_name = "test_files_asm_fixed/" + asm_file + ".txt"
    output = ""

    with open(output_file_name) as output_file:
        output = output_file.read()

    os.remove(output_file_name)
    os.makedirs("us", exist_ok=True)

    filename = asm_file.replace(".asm", ".txt")
    filenames.append(filename)

    with open("us/" + filename, "w") as output_file:
        output_file.write(output.upper())

failing_tests = 0

for filename in filenames:
    truth = ""
    us = ""

    with open("truth/" + filename) as f:
        truth = f.readlines()

    
    with open("us/" + filename) as f:
        us = f.readlines()

    for i in range(0, len(truth)):
        if truth[i] != us[i]:
            failing_tests += 1
            prefix = "[" + filename + ", line: " + str(i+1) + "]"
            print(prefix + " " * (30 - len(prefix)) + truth[i].strip() + " ----- " + us[i].strip())

if failing_tests > 0:
    print(str(failing_tests) + " tests failed")
else:
    print("all tests passed! :)")