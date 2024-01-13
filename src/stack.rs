use std::mem;

#[derive(Debug)]
pub struct Stack<T> {
    top: Option<StackItem<T>>
}

#[derive(Debug)]
struct StackItem<T> {
    data: T,
    next: Option<Box<StackItem<T>>>
}

impl<T> StackItem<T> {
    fn new(data: T) -> StackItem<T> {
        StackItem::<T> {
            data: data,
            next: None
        }
    }
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack::<T> { top: None }
    }

    pub fn is_empty(&self) -> bool {
        self.top.is_none()
    }

    pub fn push(&mut self, data: T) {
        let mut stack_item = StackItem::new(data);
        if let Some(top) = mem::replace(&mut self.top, None) {
            stack_item.next = Some(Box::new(top));
        }

        self.top = Some(stack_item);
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(top) = mem::replace(&mut self.top, None) {
            self.top = match top.next {
                Some(n) => Some(*n),
                None => None,
            };
            Some(top.data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.top {
            Some(top) => Some(&top.data),
            None => None
        }
    }
}