const STACK_SIZE: usize = 65_535;

pub struct StackItem {
    //The item's data
    data: Vec<u8>,

    //The children of a stack item
    children: Vec<StackItem>
}

impl StackItem {
    pub fn new(bytes: Vec<u8>) -> StackItem {
        StackItem {
            data: bytes,
            children: Vec::<StackItem>::new()
        }
    }
}

pub struct Stack {
    //A stack of abstract items.
    pub stack_items: Vec<StackItem>
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack_items: Vec::<StackItem>::new()
        }
    }

    pub fn push(&mut self, item: StackItem) -> Result<(), &str> {
        self.stack_items.push(item);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<(), &str> {
        if self.stack_items.len() > 0 {
            self.stack_items.pop();
            Ok(())
        } else {
            Err("PopError: cannot pop from empty stack.")
        }
    }
}