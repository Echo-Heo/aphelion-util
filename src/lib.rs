/*!
Aphelion is a 64-bit RISC instruction set architecture.
It operates on 64-bit data and uses 32-bit wide instructions.
Aphelion aims to be a rich and featureful architecture without
succumbing to paralyzing minimalism or unwieldy complexity.

# Registers

| Mnemonic   | Code      | Description         |
| ---------- | --------- | ------------------- |
| `rz`       | `0`       | always `0`          |
| `ra`--`rk` | `1`--`11` | general purpose     |
| `ip`       | `12`      | instruction pointer |
| `sp`       | `13`      | stack pointer       |
| `fp`       | `14`      | frame pointer       |
| `st`       | `15`      | status register     |

## General Purpose Registers

Registers `ra` through `rk` can be used to store data relevant to the program.
They serve no special function and are not independently significant in any way.

## RZ --- Zero Register

The zero register `rz` always holds the value 0. `rz` ignores all write operations.

## IP --- Instruction Pointer

The instruction pointer `ip` holds the address of the next instruction to be executed.
It is incremented after an instruction is loaded into the processor,
but before that instruction is executed.
This is so that control flow instructions can modify the instruction pointer
to point to the next instruction without worrying about off-by-one errors.

The instruction pointer `ip` can be set to a value that is not aligned to 4 bytes,
but an `Unaligned Access` interupt will trigger when the next instruction is loaded.

## SP, FP --- Stack & Frame Pointer

Registers `sp` and `fp` are the stack pointer and the frame pointer, respectively.
The stack pointer contains the memory address of the top stack entry.
The frame pointer contains the base address of the current stack frame.
See Interrupts for error states.

Like all registers, `fp` and `sp` are initialized to `0` upon startup.
Aphelion's built-in stack instructions grow the stack downwards,
so these registers should be explicitly set before any operations that involve the stack happen.

## ST --- Status Register

The status register contains bit flags and information about the processor state.
Most flags are set by the `cmp` comparison instructions, with the exception of `CB` and `CBU`,
which are set by `add` and `sub`.
Modifying the status register (outside of special instructions) is illegal and will trigger an `Invalid Instruction` interrupt.

`st` is laid out like so:

| 
*/