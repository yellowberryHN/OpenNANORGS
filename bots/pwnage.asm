info: PWNAGE, Joshua Hu

jmp Start

simple_drone_loop:
travel r4
jns simple_drone_switch
sense r1
cmp r1, 0
je simple_drone_loop
cmp r1, 0xFFFF
je simple_drone_drop
eat
jmp simple_drone_loop
simple_drone_drop:
energy r3
sub r3, 5000
release r3
jmp simple_drone_loop
simple_drone_switch:
charge r4, 1000
rand r4, 4
jmp simple_drone_loop

pokedefense:
data { 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 }
data { 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 }
data { 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 }
data { 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 }
data { 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 }
data { 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 }
data { 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 }
data { 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 }
data { 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 }
data { 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 }

Start:
rand r4, 4
rand r1, 2
cmp r1, 0
je attacker_loop

drone_loop:
travel r4
jns drone_switch
sense r1
cmp r1, 0
je drone_loop
cmp r1, 0xFFFF
jl drone_test
drone_drop:
energy r3
sub r3, 5000
release r3
jmp drone_loop
drone_test:
cmp [badfood+r1], 0xFFFE
jg drone_eat
je drone_loop
mov r13, 0
cksum r13, 3599
eat
jns drone_loop
mov r12, 0
cksum r12, 3599
cmp r12, r13
jne drone_mark
mov [badfood+r1], 0xFFFF
jmp drone_loop
drone_eat:
eat
jmp drone_loop
drone_mark:
mov [badfood+r1], 0xFFFE
jmp drone_loop
drone_switch:
charge r4, 1000
rand r4, 4
jmp drone_loop

badfood:
data { 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 }

attacker_loop:
travel r4
jns attacker_command
eat
travel r4
jns attacker_command
eat
travel r4
jns attacker_command
eat
travel r4
jns attacker_command
eat
travel r4
jns attacker_command
eat
jmp attacker_switch
attacker_command:
mov r2, r4
mov r0, 0
poke r2, 84
peek r2, 87
cmp r2, 32782
jne attacker_switch
attacker_attack:
mov r9,	simple_drone_loop
mov r0,	[simple_drone_loop]
attacker_attack_loop:
poke r2, r9
jns attacker_switch
add r9,	1
mov r0,	[r9]
cmp r9,	pokedefense
jne attacker_attack_loop
attacker_end:
jmp drone_loop
attacker_switch:
charge r4, 1000
rand r4, 4
jmp attacker_loop

count:
data { 0 }

myID: 
data { 500 }
