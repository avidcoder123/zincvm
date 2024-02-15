pub enum Instruction<'a> {
    //item.push [...bytes] -> StackItem
    ItemPush(&'a[u8]),

    //out.printRaw -> IO
    OutPrintRaw(),

    //i32.push [number] -> number
    I32Push(i32),

    //number.print -> IO
    I32Print(),

    //number.add [data type] -> number
    I32Add()
}