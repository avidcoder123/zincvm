use crate::stack::Stack;
use crate::instructions::{Instruction, Instruction::*};
use crate::number::Number::{self, *};


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
                let to_push: &[u8] = match *number {
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
                };

                stack.push(to_push);

                Ok(())
            },

            NumberPrint(numType) => {
                let item = stack.pop();
                match item {
                    Err(message) => Err(message),
                    Ok(number) => {
                        match numType {
                            Byte(n) => println!("{}", i8::from_be_bytes(number.try_into().unwrap())),
                            UByte(n) => println!("{}", u8::from_be_bytes(number.try_into().unwrap())),
                            Int(n) => println!("{}", i16::from_be_bytes(number.try_into().unwrap())),
                            UInt(n) => println!("{}", u16::from_be_bytes(number.try_into().unwrap())),
                            Long(n) => println!("{}", i32::from_be_bytes(number.try_into().unwrap())),
                            ULong(n) => println!("{}", u32::from_be_bytes(number.try_into().unwrap())),
                            LongLong(n) => println!("{}", i64::from_be_bytes(number.try_into().unwrap())),
                            ULongLong(n) => println!("{}", u64::from_be_bytes(number.try_into().unwrap())),
                            LongLongLong(n) => println!("{}", i128::from_be_bytes(number.try_into().unwrap())),
                            ULongLongLong(n) => println!("{}", u128::from_be_bytes(number.try_into().unwrap())),
                            Float(n) =>println!("{}", f32::from_be_bytes(number.try_into().unwrap())),
                            Double(n) => println!("{}", f64::from_be_bytes(number.try_into().unwrap())),
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