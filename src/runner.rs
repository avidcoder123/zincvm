use crate::stack::Stack;
use crate::instructions::{Instruction, Instruction::*};
use crate::number::Number::{self, *};


pub fn run(instructions: Vec<Instruction>) {
    let mut stack: Stack = Stack::new();

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

            I32Push(num) => {
                let item = num.to_be_bytes();
                stack.push(&item);
                Ok(())
            },

            I32Print() => {
                let num = stack.pop();
                match num {
                    Ok(bytes) => {
                        let mut num: [u8; 4] = [0, 0, 0, 0];
                        for i in 0..4 {
                            num[i] = bytes[i];
                        }
                        let num = i32::from_be_bytes(num);
                        println!("{}", num);
                        Ok(())
                    },
                    Err(msg) => Err(msg)
                }
            },

            I32Add() => {
                let num1 = stack.pop();
                let num2 = stack.pop();
                
            }
        };

        if let Err(message) = result {
            println!("{}", message);
        }
    }
}