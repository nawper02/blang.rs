use crate::data::context::AppContext;
use crate::stack::item::StackItem;

pub(crate) fn route_function_call(name: String, args: Vec<String>, context: &mut AppContext) -> Result<(), String> {
    match name.as_str() {
        "add" => add(args, context),
        "dup" => dup(args, context),
        _ => Err(format!("Unknown function: {}", name)),
    }
}

fn add(args: Vec<String>, context: &mut AppContext) -> Result<(), String> {
    // Basic implementation of add function
    // This example simply adds two numbers from the stack
    let a = context.stack.pop().ok_or("Error: Stack is empty")?;
    let b = context.stack.pop().ok_or("Error: Stack is empty")?;

    match (a, b) {
        (StackItem::Number(num1), StackItem::Number(num2)) => {
            context.stack.push(StackItem::Number(num1 + num2));
            Ok(())
        },
        _ => Err("Error: Both arguments must be numbers".to_string()),
    }
}

fn dup(_args: Vec<String>, context: &mut AppContext) -> Result<(), String> {
    // Check if there is an element on top of the stack
    if let Some(top_element) = context.stack.last().cloned() {
        // Push the duplicate of the top element onto the stack
        context.stack.push(top_element);
        Ok(())
    } else {
        Err("Error: Stack is empty".to_string())
    }
}

