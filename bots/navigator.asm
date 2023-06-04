info: The Navigator, Chris Watkins

main:
    jmp init

check:
    sense r0
    jns inform // nothing of interest?
    cmp r0,0xFFFF
    je collect // food collection
    jmp consume

inform:
    cmp r10,1 // see if inform 'flag' is set
    jne investigate // if not, investigate
    mov r0,r7
    poke [d],[r7] // tell organism of danger
    mov r0,r12
    poke [d],[r12] // tell organism of x coord
    mov r0,r13
    poke [d],[r13] // tell organism of y coord
    jmp move

investigate:
    mov r9,[d]
    peek r9,[r7] // look for any dangers
    mov r7,r9 // record dangers, if any
    mov r9,[d]
    peek r9,[r12] // get x coord
    mov r12,r9 // record x coord, if given
    mov r9,[d]
    peek r9,[r13] // get y coord
    mov r13,r9 // record y coord, if given
    jmp move

collect:
    getxy [x],[y]
    mov r12,[x]
    mov r13,[y]
    mov r10,1 // set off inform 'flag'
    energy r1
    cmp r1,5000
    jl move
    sub r1,5000
    release r1
    jmp move

consume:
    cmp r7,r0
    je move
    eat
    jmp infect // was the food infected?

infect:
    mov r5,0
    cksum r5,999
    cmp r5,[s] // compare this and previous checksum
    jne remedy

remedy:
    mov r7,r0 // identity of food to stay away from
    mov r10,1 // set off inform 'flag'
    jmp move

move:
    energy r1
    mov [t],r12
    or [t],r13
    cmp [t],0
    jne ch
    cmp [c],0
    je dirloc
    travel [d]
    jns dirloc
    jmp check

ch: // check for horizontal
    getxy [x],[y]
    cmp [x],r12
    je cv
    jl te
    mov [d],3 // turn west
    travel [d]
    jns dirloc
    jmp check

te: // turn east
    mov [d],2
    travel [d]
    jns dirloc
    jmp check

cv: // check for vertical
    cmp [y],r13
    je collect
    jl ts
    mov [d],0
    travel [d]
    jns dirloc
    jmp check

ts: // turn south
    mov [d],1
    travel [d]
    jns dirloc
    jmp check

dirloc: // change the variables and try to move again
    rand [c],15
    add [c],1
    rand [d],4
    travel [d]
    jns dirloc
    jmp check

// INITIALIZATION
init:
    rand [c],15
    add [c],1
    mov r7,0
    mov r12,0
    mov r13,0
    cksum [s],999
    getxy [x],[y]
    cmp [x],34
    jle qDT
    jg qUQ
qDT:
    cmp [y],19
    jle qD
    jg qT
qUQ:
    cmp [y],19
    jle qU
    jg qQ
qU:
    mov [d],2
    jmp check
qD:
    mov [d],0
    jmp check
qT:
    mov [d],3
    jmp check
qQ:
    mov [d],1
    jmp check

// VARIABLES
x: data {0}
y: data {0}
d: data {0}
c: data {0}
s: data {0}
t: data {0}