use instructions::Instruction;
use runner::run;

mod runner;
mod stack;
mod instructions;
mod number;

fn main() {
    let instructions: Vec<Instruction> = vec![
        Instruction::ItemPush(&[1]),
        Instruction::ItemPush(&[2]),
        Instruction::ItemPush(&[3]),
        Instruction::OutPrintRaw(),
        Instruction::OutPrintRaw(),
        Instruction::OutPrintRaw()
    ];
    run(instructions);
}
