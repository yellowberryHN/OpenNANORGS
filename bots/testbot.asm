info: TestBot, YellowberryHN

main:
    RAND r0, 4
    TRAVEL r0
    EAT
    SENSE r0
    CMP r0, 0xFFFF
    JNE 0
    RELEASE 10000
    JMP 0

