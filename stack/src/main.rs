struct Stack<T> {
    elem: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack {
            elem: Vec::new(),
        }
    }

    fn size(&self) -> usize {
        self.elem.len()
    }

    fn top(&self) -> Option<&T> {
        self.elem.last()
    }

    fn push(&mut self, x: T) {
        self.elem.push(x);
    }

    fn pop(&mut self) -> Option<T> {
        self.elem.pop()
    }
}

fn main() {
    let mut stack: Stack<i32> = Stack::new();

    assert_eq!(stack.size(), 0);

    stack.push(1);
    stack.push(2);

    assert_eq!(stack.size(), 2);
    assert_eq!(stack.top().unwrap(), &2);
    assert_eq!(stack.pop().unwrap(), 2);
    assert_eq!(stack.top().unwrap(), &1);

    stack.pop();

    assert_eq!(stack.size(), 0);
    assert_eq!(stack.top(), None);
    assert_eq!(stack.pop(), None);
}
