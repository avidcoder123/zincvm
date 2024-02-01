use crate::number::Number;

pub enum Instruction<'a> {
    //item.push [...bytes] -> StackItem
    ItemPush(&'a[u8]),

    //out.printRaw -> IO
    OutPrintRaw(),

    //number.push [data type] [number] -> number
    NumberPush(Number),

    //number.print [data type] -> IO
    NumberPrint
}