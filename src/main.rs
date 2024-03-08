#![allow(clippy::unusual_byte_groupings)]

use aphelion_util::{instruction::{instruction_set::{BranchCond, InstructionSet}, Instruction}, registers::Register};

fn main() {
    /* let inst = Instruction(0);
    println!("{inst}");
    let inst = Instruction(0xFFFFFFFF);
    println!("{inst}"); */
    let inst: Instruction = InstructionSet::Asri { rd: Register::Rb, r1: Register::Rc, imm16: 69 }.to_instruction();
    println!("{inst}");
    let inst = InstructionSet::Branch { cc: BranchCond::Bltu, imm20: 500 }.to_instruction();
    println!("{inst}");
}
