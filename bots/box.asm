info: BoxBot, Yellow

main:
    add r0, 1 // pick next dir
    mod r0, 4 // wrap dir around
    add r1, 5 // how big the box is
move:
    travel [dirs+r0]
    eat
    sub r1, 1
    cmp r1, 0
    jne move
    jmp main
dirs:
    data { 3 1 2 0 }