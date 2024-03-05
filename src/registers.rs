#![warn(clippy::pedantic)]
/*!
# Registers

Aphelion defines sixteen 64-bit registers.

| Mnemonic                                   | Code      | Description         |
| :----------------------------------------- | :-------- | :------------------ |
| [`rz`](Register::Rz)                       | `0`       | always `0`          |
| [`ra`](Register::Ra)--[`rk`](Register::Rk) | `1`--`11` | general purpose     |
| [`ip`](Register::Ip)                       | `12`      | instruction pointer |
| [`sp`](Register::Sp)                       | `13`      | stack pointer       |
| [`fp`](Register::Fp)                       | `14`      | frame pointer       |
| [`st`](Register::St)                       | `15`      | status register     |

## General Purpose Registers

Registers [`ra`](Register::Ra) through [`rk`](Register::Rk)
can be used to store data relevant to the program.
They serve no special function and are not independently
significant in any way.

## [`rz`](Register::Rz) --- Zero Register

The zero register [`rz`](Register::Rz) always holds the value `0`.
[`rz`](Register::Rz) ignores all write operations.

## [`ip`](Register::Ip) --- Instruction Pointer

The instruction pointer [`ip`](Register::Ip) holds the address
of the next instruction to be executed.
It is incremented after an instruction is loaded into the processor,
but before that instruction is executed.
This is so that control flow instructions can modify the instruction
pointer to point to the next instruction without
worrying about off-by-one errors.

The instruction pointer [`ip`](Register::Ip) can be set to a value
that is not aligned to 4 bytes,but an [`Unaligned Access`] interupt
will trigger when the next instruction is loaded.

## [`sp`](Register::Sp), [`fp`](Register::Fp) --- Stack & Frame Pointer

Registers [`sp`](Register::Sp) and [`fp`](Register::Fp) are
the stack pointer and the frame pointer, respectively.
The stack pointer contains the memory address of the top stack entry.
The frame pointer contains the base address of the current stack frame.
See [interrupts](crate::TODO) for error states.

Like all registers, [`sp`](Register::Sp) and [`fp`](Register::Fp) are
initialized to `0` upon startup.
Aphelion's built-in stack instructions grow the stack downwards,
so these registers should be explicitly set before any operations
that involve the stack happen.

## [`st`](Register::St) --- Status Register

The status register contains bit flags and
information about the processor state.
Most flags are set by the [`cmp`](crate::TODO) comparison instructions,
with the exception of [`CB`](crate::TODO) and [`CBU`](crate::TODO),
which are set by [`add`](crate::TODO) and [`sub`](crate::TODO).
Modifying the status register (outside of special instructions) is
illegal and will trigger an [`Invalid Instruction`](crate::TODO) interrupt.

The status register is laid out like so:

| `63..32` | `31` | `30..8`    | `7` | `6`  | `5` | `4` | `3`   | `2`  | `1` | `0` |
| -------- | ---- | ---------- | --- | ---- | --- | --- | ----- | ---- | --- | --- |
| `CI`     | `EF` | `[unused]` | `M` | `LU` | `L` | `E` | `CBU` | `CB` | `Z` | `S` |

where:

| Key | Name | Description (with `a` and `b`) |
| :-- | :--- | :---------- |
| `S`   | `SIGN`                  | `(a as i64) < 0` |
| `Z`   | `ZERO`                  | `a == 0` |
| `CB`  | `CARRY_BORROW`          | `a + b + (C as i64) > i64::MAX` \|\| `a - b - (B as i64) < i64::MIN` |
| `CBU` | `CARRY_BORROW_UNSIGNED` | `a + b + (C as u64) > u64::MAX` \|\| `a - b - (B as u64) < u64::MIN` |
| `E`   | `EQUAL`                 | `a == b` |
| `L`   | `LESS`                  | `(a as i64) < (b as i64)` |
| `LU`  | `LESS_UNSIGNED`         | `(a as u64) < (b as u64)` |
| `M`   | `MODE`                  | processor mode |
| `EF`  | `EXT_F`                 | floating point operations enabled |
| `CI`  | `CURRENT_INST`          | copy of the current instruction's machine code |

*/

use crate::nibble::Nibble;

/**
Registers kinds. 
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Register {
    /// [Zero Register](crate::registers#rz--zero-register)
    Rz = 0x0,
    /// [General Purpose Register](crate::registers#general-purpose-registers)
    Ra = 0x1,
    /// [General Purpose Register](crate::registers#general-purpose-registers)
    Rb = 0x2,
    /// [General Purpose Register](crate::registers#general-purpose-registers)
    Rc = 0x3,
    /// [General Purpose Register](crate::registers#general-purpose-registers)
    Rd = 0x4,
    /// [General Purpose Register](crate::registers#general-purpose-registers)
    Re = 0x5,
    /// [General Purpose Register](crate::registers#general-purpose-registers)
    Rf = 0x6,
    /// [General Purpose Register](crate::registers#general-purpose-registers)
    Rg = 0x7,
    /// [General Purpose Register](crate::registers#general-purpose-registers)
    Rh = 0x8,
    /// [General Purpose Register](crate::registers#general-purpose-registers)
    Ri = 0x9,
    /// [General Purpose Register](crate::registers#general-purpose-registers)
    Rj = 0xA,
    /// [General Purpose Register](crate::registers#general-purpose-registers)
    Rk = 0xB,
    /// [Instruction Pointer](crate::registers#ip--instruction-pointer)
    Ip = 0xC,
    /// [Stack Pointer](crate::registers#sp-fp--stack--frame-pointer)
    Sp = 0xD,
    /// [Frame Pointer](crate::registers#sp-fp--stack--frame-pointer)
    Fp = 0xE,
    /// [Status Register](crate::registers#st--status-register)
    St = 0xF,
}
impl Register {
    /// Convert `self` to [`u8`]
    ///
    /// # Examples
    ///
    /// ```
    /// use asteroid_rs::registers::Register;
    ///
    /// assert_eq!(Register::Sp.to_u8(), 0xDu8);
    /// ```
    #[must_use]
    pub const fn to_u8(self) -> u8 { self as u8 }

    /// Attempts to convert a [`u8`] to `Self`
    ///
    /// # Examples
    ///
    /// ```
    /// use asteroid_rs::registers::Register;
    ///
    /// assert_eq!(Register::try_from_u8(0xDu8), Some(Register::Sp));
    /// assert_eq!(Register::try_from_u8(0x10u8), None);
    /// ```
    #[must_use]
    pub const fn try_from_u8(v: u8) -> Option<Self> {
        match v {
            0x0 => Some(Self::Rz),
            0x1 => Some(Self::Ra),
            0x2 => Some(Self::Rb),
            0x3 => Some(Self::Rc),
            0x4 => Some(Self::Rd),
            0x5 => Some(Self::Re),
            0x6 => Some(Self::Rf),
            0x7 => Some(Self::Rg),
            0x8 => Some(Self::Rh),
            0x9 => Some(Self::Ri),
            0xA => Some(Self::Rj),
            0xB => Some(Self::Rk),
            0xC => Some(Self::Ip),
            0xD => Some(Self::Sp),
            0xE => Some(Self::Fp),
            0xF => Some(Self::St),
            _ => None,
        }
    }
    #[must_use]
    pub const fn from_nibble(v: Nibble) -> Self {
        match v {
            Nibble::X0 => Self::Rz,
            Nibble::X1 => Self::Ra,
            Nibble::X2 => Self::Rb,
            Nibble::X3 => Self::Rc,
            Nibble::X4 => Self::Rd,
            Nibble::X5 => Self::Re,
            Nibble::X6 => Self::Rf,
            Nibble::X7 => Self::Rg,
            Nibble::X8 => Self::Rh,
            Nibble::X9 => Self::Ri,
            Nibble::XA => Self::Rj,
            Nibble::XB => Self::Rk,
            Nibble::XC => Self::Ip,
            Nibble::XD => Self::Sp,
            Nibble::XE => Self::Fp,
            Nibble::XF => Self::St,
        }
    }
}
