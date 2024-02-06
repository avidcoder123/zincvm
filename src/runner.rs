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
                stack.push(match *number {
                    Byte(n) => &n.to_be_bytes(),
                    UByte(n) => &n.to_be_bytes(),
                    Int(n) => &n.to_be_bytes(),
                    UInt(n) => &n.to_be_bytes(),
                    Long(n) => &n.to_be_bytes(),
                    ULong(n) => &n.to_be_bytes(),
                    LongLong(n) => &n.to_be_bytes(),
                    ULongLong(n) => &n.to_be_bytes(),
                    LongLongLong(n) => &n.to_be_bytes(),
                    ULongLongLong(n) => &n.to_be_bytes(),
                    Float(n) => &n.to_be_bytes(),
                    Double(n) => &n.to_be_bytes(),
                })
            },

            NumberPrint() => {
                let item = stack.pop();
                match item {
                    Err(message) => Err(message),
                    Ok(number) => {
                        match *number {
                            Byte(n) => println!("{}", i8::from_be_bytes([number[0]])),
                            UByte(n) => u8::from_be_bytes([number[0]]),
                            Int(n) => &n.to_be_bytes(),
                            UInt(n) => &n.to_be_bytes(),
                            Long(n) => &n.to_be_bytes(),
                            ULong(n) => &n.to_be_bytes(),
                            LongLong(n) => &n.to_be_bytes(),
                            ULongLong(n) => &n.to_be_bytes(),
                            LongLongLong(n) => &n.to_be_bytes(),
                            ULongLongLong(n) => &n.to_be_bytes(),
                            Float(n) => &n.to_be_bytes(),
                            Double(n) => &n.to_be_bytes(),
                        };
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