
Here is a list of every instruction recognized by the bot's CPU.

NOTE: Instructions do not modify flags unless specified.

## Classic Instruction Set

### NOP
**Opcode:** 0 (`00`)<br/>
**Energy Used:** 1

Do nothing.

### MOV `dest`, `src`
**Opcode:** 1 (`01`)<br/>
**Energy Used:** 1

Moves the value of `src` into `dest`.

### PUSH `src`
**Opcode:** 2 (`02`)<br/>
**Energy Used:** 1

Pushes the value of `src` onto the stack. The stack pointer is pre-decremented first.

### POP `dest`
**Opcode:** 3 (`03`)<br/>
**Energy Used:** 1

Pops the top value of the stack into `dest`. The stack pointer is post-incremented afterwards.

### CALL `pos`
**Opcode:** 4 (`04`)<br/>
**Energy Used:** 1

Pushes the address of the next instruction to the top of the stack, and then sets the instruction pointer to **pos**.

### RET
**Opcode:** 5 (`05`)<br/>
**Energy Used:** 1

Sets the instruction pointer to the top value on the stack, and pops the top value from the stack.

### JMP `pos`
**Opcode:** 6 (`06`)<br/>
**Energy Used:** 1

Unconditionally sets the instruction pointer to the value of `pos`.

### JL `pos`
**Opcode:** 7 (`07`)<br/>
**Energy Used:** 1

Sets the instruction pointer to the value of `pos` if the `L` flag is set. If the `L` flag is unset, continues execution normally.

### JLE `pos`
**Opcode:** 8 (`08`)<br/>
**Energy Used:** 1

Sets the instruction pointer to the value of `pos` if the `L` or `E` flag is set. If the neither flag is set, continues execution normally.

### JG `pos`
**Opcode:** 9 (`09`)<br/>
**Energy Used:** 1

Sets the instruction pointer to the value of `pos` if the `G` flag is set. If the `G` flag is unset, continues execution normally.

### JGE `pos`
**Opcode:** 10 (`0A`)<br/>
**Energy Used:** 1

Sets the instruction pointer to the value of `pos` if the `G` or `E` flag is set. If the neither flag is set, continues execution normally.

### JE `pos`
**Opcode:** 11 (`0B`)<br/>
**Energy Used:** 1

Sets the instruction pointer to the value of `pos` if the `E` flag is set. If the `E` flag is unset, continues execution normally.

### JNE `pos`
**Opcode:** 12 (`0C`)<br/>
**Energy Used:** 1

Sets the instruction pointer to the value of `pos` if the `E` flag is **NOT** set. If the `E` flag is set, continues execution normally.

### JS `pos`
**Opcode:** 13 (`0D`)<br/>
**Energy Used:** 1

Sets the instruction pointer to the value of `pos` if the `S` flag is set. If the `S` flag is unset, continues execution normally.

### JNS `pos`
**Opcode:** 14 (`0E`)<br/>
**Energy Used:** 1

Sets the instruction pointer to the value of `pos` if the `S` flag is **NOT** set. If the `S` flag is set, continues execution normally.

### ADD `dest`, `src`
**Opcode:** 15 (`0F`)<br/>
**Energy Used:** 1

Adds the value of `src` to `dest`, and sets `dest` to the result.

### SUB `dest`, `src`
**Opcode:** 16 (`10`)<br/>
**Energy Used:** 1

Subtracts the value of `src` from `dest`, and sets `dest` to the result.

### MULT `dest`, `src`
**Opcode:** 17 (`11`)<br/>
**Energy Used:** 1

Multiplies the value of `dest` by `src`, and sets `dest` to the result.

### DIV `dest`, `src`
**Opcode:** 18 (`12`)<br/>
**Energy Used:** 1

Divides the value of `dest` by `src`, and sets `dest` to the result. Division by zero will act as a [NOP](#NOP) instruction.

### MOD `dest`, `src`
**Opcode:** 19 (`13`)<br/>
**Energy Used:** 1

Performs a modulo division on the value of `dest` by `src`, and sets `dest` to the result. Modulo by zero will act as a [NOP](#NOP) instruction.

### AND `dest`, `src`
**Opcode:** 20 (`14`)<br/>
**Energy Used:** 1

Performs a bitwise AND on the value of `dest` based on the value of `src`.

### OR `dest`, `src`
**Opcode:** 21 (`15`)<br/>
**Energy Used:** 1

Performs a bitwise OR on the value of `dest` based on the value of `src`.

### XOR `dest`, `src`
**Opcode:** 22 (`16`)<br/>
**Energy Used:** 1

Performs a bitwise XOR on the value of `dest` based on the value of `src`.

### CMP `lhs`, `rhs`
**Opcode:** 23 (`17`)<br/>
**Energy Used:** 1

Performs a comparison between the values of `lhs` and `rhs`.

**Flags:** Clears all current flags. Performs the following:

- `L` flag is set if `lhs` < `rhs`
- `G` flag is set if `lhs` > `rhs`
- `E` flag is set if `lhs` == `rhs`

### TEST `lhs`, `rhs`
**Opcode:** 24 (`18`)<br/>
**Energy Used:** 1

Tests equality between `lhs` and `rhs` by performing a bitwise AND. The value of the bitwise AND is discarded.

**Flags:** Clears all current flags. `E` flag is set if the bitwise AND returns 0.

### GETXY `destx`, `desty`
**Opcode:** 25 (`19`)<br/>
**Energy Used:** 1

Places the bot's current X and Y coordinates into the specified `destx` and `desty` locations.

### ENERGY `dest`
**Opcode:** 26 (`1A`)<br/>
**Energy Used:** 1

Places the bot's current energy level into the location specified by `dest`.

### TRAVEL `dir`
**Opcode:** 27 (`1B`)<br/>
**Energy Used:** ***1-10***

Moves the bot one tile in the direction specified by `dir`, as long as the tile is not occupied by another bot or out of bounds. This instruction costs **10 energy** if successful, otherwise it costs 1 energy.

The directions are as follows:

- `0` (North) - Bot's Y coordinate decreases by 1
- `1` (South) - Bot's Y coordinate increases by 1
- `2` (East) - Bot's X coordinate increases by 1
- `3` (West) - Bot's X coordinate decreases by 1

If a value greater than 3 is specified by `dir`, the direction will be `dir % 4`. In other words, it will wrap around.

**Flags:** `S` flag is set if the movement was successful, if not, the `S` flag is unset.

### SHL `dest`, `count`
**Opcode:** 28 (`1C`)<br/>
**Energy Used:** 1

Performs a bitwise left shift on the value of `dest`, shifting left `count` times. If `count` is greater than 16, the result is undefined.

### SHR `dest`, `count`
**Opcode:** 29 (`1D`)<br/>
**Energy Used:** 1

Performs a bitwise right shift on the value of `dest`, shifting right `count` times. If `count` is greater than 16, the result is undefined.

### SENSE `dest`
**Opcode:** 30 (`1E`)<br/>
**Energy Used:** 1

Checks the contents of the current tile. If the tile is not empty, the ID of the object on the tile will be set into the location set by `dest`. If the tile contains sludge, `dest` will be set to the non-zero sludge type. If the tile contains a collection point, `dest` will be set to 65535 (`FFFF`). If the tile is empty, `dest` will be set to 0.

**Flags:** `S` flag is set if the tile is not empty, if it is empty, the `S` flag is unset.

### EAT
**Opcode:** 31 (`1F`)<br/>
**Energy Used:** 1

Attempts to eat the contents of the current tile. If the tile contains sludge, the bot will eat the sludge and receive 2000 energy. If receiving 2000 energy would put the bot's energy level over 65535, the bot fail to eat the sludge.

**Flags:** `S` flag is set if the eating is successful, if not, the `S` flag is unset.

### RAND `dest`, `max`
**Opcode:** 32 (`20`)<br/>
**Energy Used:** 1

Calculates a random number between 0 and the value of `max`, exclusive, and stores it into `dest`.

### RELEASE `amount`
**Opcode:** 33 (`21`)<br/>
**Energy Used:** 1

Attempts to release `amount` of energy onto the current tile. If the tile contains a collection point, the bot's energy will be reduced by `amount`, and the tank score will be increased by `amount`. If the tile does not contain a collection point, the bot will lose the energy released with no increase in score. If the bot attempts to release more energy than it has, the release will fail.

**Flags:** `S` flag is set if the release is successful, if not, the `S` flag is unset.

### CHARGE `dir`, `amount`
**Opcode:** 34 (`22`)<br/>
**Energy Used:** 1

Attempts to give `amount` of energy to a bot in the direction specified by `dir`. If the tile in the specified direction contains a bot, the current bot's energy will be reduced by `amount` and the target bot's energy will be increased by `amount`. If the tile in the specified direction does not contain a bot, charging will fail. If the bot attempts to charge more energy than it has, charging will fail. If charging the target bot would put its energy level over 65535, charging will fail.

**Flags:** `S` flag is set if the charging is successful, if not, the `S` flag is unset.

### POKE `dir`, `offset`
**Opcode:** 35 (`23`)<br/>
**Energy Used:** 1

Attempts to set the memory at `offset` of a bot in the direction specified by `dir` to the current value of `R0`. If the tile in the specified direction does not contain a bot, poking will fail.

**Flags:** `S` flag is set if the poking is successful, if not, the `S` flag is unset.

### PEEK `dest`, `offset`
**Opcode:** 36 (`24`)<br/>
**Energy Used:** 1

Attempts to set the value of `dest` to the memory at `offset` of a bot in the direction specified by `dest`. If the tile in the specified direction does not contain a bot, peeking will fail.

**Flags:** `S` flag is set if the poking is successful, if not, the `S` flag is unset.

### CKSUM `start`, `end`
**Opcode:** 37 (`25`)<br/>
**Energy Used:** 1

Calculates a checksum of the specified range of memory from `start` to `end`, exclusive, by adding their values together. The result is stored into the location specified by `start`.

## Extended Instruction Set (WIP)

These instructions are recognized when the extended features are enabled. These are subject to change until finalized.

### GETID `dest`
**Opcode:** 38 (`26`)<br/>
**Energy Used:** 1

Places the bot's ID into `dest`. Bot ID is determined by the character they are represented by.

### LEVEL `dest`
**Opcode:** 39 (`27`)<br/>
**Energy Used:** 1

Places the bot's current level (Z coordinate) into `dest`.

### CLIMB
**Opcode:** 40 (`28`)<br/>
**Energy Used:** ***1-10***

Moves the bot to the other side of a ramp, as long as the other side of the ramp is not occupied by another bot. This instruction costs **10 energy** if successful, otherwise it costs 1 energy.

**Flags:** `S` flag is set if the movement was successful, if not, the `S` flag is unset.