#![allow(clippy::unusual_byte_groupings)]

use asteroid_rs::instruction::Instruction;

fn main() {
    let inst = Instruction(0);
    println!("{inst}");
    let inst = Instruction(0xFFFFFFFF);
    println!("{inst}");
}