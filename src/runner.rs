use crate::stack::Stack;
use crate::instructions::{Instruction, Instruction::*};


pub fn run(instructions: Vec<Instruction>) {
    let stack = Stack::new();

    for i in 0..instructions.len() {
        match &instructions[i] {
            ItemPush(bytes) => {
                println!("Push");
            },

            _ => ()
        }
    }
}