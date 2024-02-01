pub enum Instruction<'a> {
    //item.push [...bytes] -> StackItem
    ItemPush(&'a[u8]),

    //out.printRaw -> IO
    OutPrintRaw()
}