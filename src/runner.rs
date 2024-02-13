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

            NumberPush(number) => {
                let result = match *number {
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
                };
                

                result
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
            },

            NumberAdd() => {
                let item1 = stack.pop();
                let item2 = stack.pop();

                if let Err(msg) = item1 {
                    return Err(msg);
                } else if let item2 = Err(i) {
                    return Err(i);
                } else {
                    Ok(())
                }
            }
        };

        if let Err(message) = result {
            println!("{}", message);
        }
    }
}