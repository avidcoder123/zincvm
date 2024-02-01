use crate::stack::Stack;
use crate::instructions::{Instruction, Instruction::*};
use crate::number::Number::*;


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
            },

            NumberPush(number) => {
                match *number {
                    Byte(n) => stack.push(&n.to_be_bytes()),
                    UByte(n) => stack.push(&n.to_be_bytes()),
                    Int(n) => stack.push(&n.to_be_bytes()),
                    UInt(n) => stack.push(&n.to_be_bytes()),
                    Long(n) => stack.push(&n.to_be_bytes()),
                    ULong(n) => stack.push(&n.to_be_bytes()),
                    LongLong(n) => stack.push(&n.to_be_bytes()),
                    ULongLong(n) => stack.push(&n.to_be_bytes()),
                    LongLongLong(n) => stack.push(&n.to_be_bytes()),
                    ULongLongLong(n) => stack.push(&n.to_be_bytes()),
                    Float(n) => stack.push(&n.to_be_bytes()),
                    Double(n) => stack.push(&n.to_be_bytes()),
                }
            },

            NumberPrint
        };

        if let Err(message) = result {
            println!("{}", message);
        }
    }
}