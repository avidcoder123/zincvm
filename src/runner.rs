use crate::stack::Stack;
use crate::instructions::{Instruction, Instruction::*};


pub fn run(instructions: Vec<Instruction>) {
    let mut stack = Stack::new();

    for i in 0..instructions.len() {
        let result = match &instructions[i] {
            ItemPush(bytes) => {
                stack.push(*bytes)
            },

            OutPrintRaw() => {
                let item = stack.pop();
                match item {
                    Err(message) => Err(message),
                    Ok(slice) => {
                        println!("{:?}", slice);
                        Ok(())
                    }
                }
            }
        };

        if let Err(message) = result {
            println!("{}", message);
        }
    }
}