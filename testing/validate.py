import os
import subprocess

filenames = []
os.makedirs("us", exist_ok=True)

for asm_file in os.listdir("test_files_asm"):
    # this should be run in release mode due to the absurd amount of times we invoke the compiler
    out = subprocess.run(["cargo", "run", "--release", "--", "-z", "test_files_asm/" + asm_file], capture_output=True, text=True) 

    output_file_name = "test_files_asm/" + asm_file

    filename = asm_file.replace(".asm", ".txt")
    filenames.append(filename)

    with open("us/" + filename, "w") as output_file:
        output_file.write(out.stdout)

failing_tests = 0

for filename in filenames:
    truth = ""
    us = ""

    with open("truth/" + filename) as f:
        truth = f.readlines()

    
    with open("us/" + filename) as f:
        us = f.readlines()

    for i in range(2, len(truth)):
        if truth[i] != us[i]:
            failing_tests += 1
            prefix = "[" + filename + ", line: " + str(i+1) + "]"
            print(prefix + " " * (30 - len(prefix)) + truth[i].strip() + " ----- " + us[i].strip())

if failing_tests > 0:
    print(str(failing_tests) + " tests failed")
else:
    print("all tests passed! :)")
