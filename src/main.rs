use instructions::Instruction;
use runner::run;

mod runner;
mod stack;
mod instructions;

fn main() {
    let instructions: Vec<Instruction> = vec![
        Instruction::ItemPush(&[0, 0, 255, 255]),
        Instruction::OutPrintRaw()
    ];
    run(instructions);
}
