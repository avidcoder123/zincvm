const STACK_SIZE: usize = 65_535;

pub struct StackItem {
    //The item's start index
    start: usize,

    //The item's size in bytes
    size: usize
}

pub struct Stack {
    //A flat stack of bytes.
    pub stack: [u8; STACK_SIZE],

    //A pointer to the first unused index of the stack.
    pub ptr: usize,

    //A stack of abstract items.
    pub stack_items: Vec<StackItem>
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack_items: Vec::<StackItem>::new(),
            ptr: 0,
            stack: [0; STACK_SIZE]
        }
    }

    //Add an item to the stack
    pub fn push(&mut self, item: &[u8]) -> Result<(), &str> {
        let stack_ref = StackItem {
            start: self.ptr,
            size: item.len()
        };

        for byte in item.iter() {
            self.stack[self.ptr] = *byte;
            self.ptr += 1;
        }

        self.stack_items.push(stack_ref);
        Ok(())
    }

    //Remove the top object from the stack
    pub fn pop(&mut self) -> Result<&[u8], &str> {
        let item: Option<StackItem> = self.stack_items.pop();
        match item {
            Some(item) => {
                let slice: &[u8] = &self.stack[self.ptr - item.size .. self.ptr];
                self.ptr -= item.size;
                Ok(slice)
            },
            None => Err("Cannot pop from empty stack.")
        }
    }
}