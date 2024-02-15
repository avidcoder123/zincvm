use crate::stack::Stack;
use crate::instructions::{Instruction, Instruction::*};

fn coerceArraySize<Arr>(bytes: &[u8], len: usize) -> Result<Arr, &str> {
    if bytes.len() != len {
        Err("Cannot coerce item of size " + bytes.len() + "into size " + len)
    } else {
        Ok()
    }
}


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
                match num1 {
                    Ok(n1) => match num2 {
                        Ok(n2) => {
                            let num1 = i32::from_be_bytes(&n1[0..4]);
                        },
                        Err(msg) => Err(msg)
                    },
                    Err(msg) => Err(msg)
                }
            }
        };

        if let Err(message) = result {
            println!("{}", message);
        }
    }
}