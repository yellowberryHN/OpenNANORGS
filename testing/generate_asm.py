import os

instructions = [
    #("Instruction", "Operand1 Supported Modes", "Operand2 Supported Modes"),
    ("NOP", False, False),
    ("MOV", True, True),
    ("PUSH", True, False),
    ("POP", True, False),
    ("CALL", True, False),
    ("RET", False, False),
    ("JMP", True, False),
    ("JL", True, False),
    ("JLE", True, False),
    ("JG", True, False),
    ("JGE", True, False),
    ("JE", True, False),
    ("JNE", True, False),
    ("JS", True, False),
    ("JNS", True, False),
    ("ADD", True, True),
    ("SUB", True, True),
    ("MULT", True, True),
    ("DIV", True, True),
    ("MOD", True, True),
    ("AND", True, True),
    ("OR", True, True),
    ("XOR", True, True),
    ("CMP", True, True),
    ("TEST", True, True),
    ("GETXY", True, True),
    ("ENERGY", True, False),
    ("TRAVEL", True, False),
    ("SHL", True, True),
    ("SHR", True, True),
    ("SENSE", True, False),
    ("EAT", False, False),
    ("RAND", True, True),
    ("RELEASE", True, False),
    ("CHARGE", True, True),
    ("POKE", True, True),
    ("PEEK", True, True),
    ("CKSUM", True, True),
]

def generate_operands(instruction, op1, op2):
    instructions = [] 
    operands = [] 

    if op1:
        # registers
        for reg in ["r1", "r2", "r10", "r13", "SP"]:
            operands.append(reg)
            operands.append("[" + reg + "]")
            operands.append("[" + reg + "+1]")
            operands.append("[" + reg + "-1]")

        operands.append("1000")
        operands.append("[1000]")
        operands.append("0xDEAD")
        operands.append("[0xDEAD]")
        operands.append("someLabel")
        operands.append("[someLabel]")

    if op1 and op2:
        for op in operands:
            # registers
            for reg in ["r1", "r2", "r10", "r13", "SP"]:
                instructions.append((instruction, op, reg))
                instructions.append((instruction, op, "[" + reg + "]"))
                instructions.append((instruction, op, "[" + reg + "+1]"))
                instructions.append((instruction, op, "[" + reg + "-1]"))

            instructions.append((instruction, op, "1000"))
            instructions.append((instruction, op, "[1000]"))
            instructions.append((instruction, op, "0xDEAD"))
            instructions.append((instruction, op, "[0xDEAD]"))
            instructions.append((instruction, op, "someLabel"))
            instructions.append((instruction, op, "[someLabel]"))

    if op1 and not op2:
        for op in operands:
            instructions.append((instruction, op, None))

    if not op1 and not op2:
        instructions.append((instruction, None, None))
        

    return instructions


     

all_program_str = ""

os.makedirs("test_files_asm", exist_ok=True)

for instruction in instructions:
    program = [] 
    program.extend(generate_operands(instruction[0], instruction[1], instruction[2]))

    program_str = "info: " + instruction[0] + ", Automated Testing\n"
    program_str += "someLabel: \n"
    for p_instr in program:
        if p_instr[1] and p_instr[2]:
            program_str += p_instr[0] + " " + p_instr[1] + ", " + p_instr[2] + "\n"

        if p_instr[1] and not p_instr[2]:
            program_str += p_instr[0] + " " + p_instr[1] + "\n"
        
        if not p_instr[1] and not p_instr[2]:
            program_str += p_instr[0] + "\n"
    
    with open("test_files_asm/" + instruction[0] + ".asm", "w") as file:
        file.write(program_str)

    all_program_str += program_str


print("Generated: " + str(len(instructions)) + " test files, containing " + str(len(all_program_str.split("\n"))) + " lines")


