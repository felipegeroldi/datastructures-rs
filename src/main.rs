mod stack;

use stack::Stack;

fn main() {
    stack_example();
}

fn stack_example() {
    println!("Create stack");
    let mut stack = Stack::<i32>::new();

    println!("Verify if stack is empty");
    println!("is_empty: {}", stack.is_empty());

    println!("\nPushing some items to stack");
    stack.push(1);
    stack.push(22);
    stack.push(50);
    stack.push(-10);

    println!("Peek item: {}", stack.peek().unwrap());

    println!("Remove from stack...");
    while let Some(item) = stack.pop() {
        println!("{}", item);
    }
}
