const STACK_SIZE: usize = 65_535;

pub struct StackItem {
    //The item's data
    data: Vec<u8>,

    //The children of a stack item
    children: Vec<StackItem>
}

pub struct Stack {
    //A stack of abstract stack items which point to byte ranges on the stack.
    pub stack_items: Vec<StackItem>
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack_items: Vec::<StackItem>::new()
        }
    }
}