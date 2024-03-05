#![warn(clippy::pedantic)]
#![allow(clippy::unusual_byte_groupings)]

/*!
Aphelion is a 64-bit RISC instruction set architecture.
It operates on 64-bit data and uses 32-bit wide instructions.
Aphelion aims to be a rich and featureful architecture without
succumbing to paralyzing minimalism or unwieldy complexity.
*/

pub mod registers;
pub mod nibble;
pub mod instruction;
pub mod interrupt;
pub mod io;
// TODO: useful operations here
pub mod operation;

/// DOCUMENTATION NEEDED!
pub const TODO: () = ();
