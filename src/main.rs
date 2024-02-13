use instructions::Instruction;
use runner::run;

mod runner;
mod stack;
mod instructions;
mod number;

fn main() {
    let instructions: Vec<Instruction> = vec![
        Instruction::NumberPush(number::Number::Int(100)),
        Instruction::NumberPrint(number::Number::Int(0))
    ];
    run(instructions);
}
