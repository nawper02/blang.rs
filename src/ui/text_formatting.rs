use std::io::Stdout;
use crossterm::execute;
use crossterm::style::{Attribute, SetAttribute, Print};
use crossterm::cursor::MoveTo;
use crate::stack::item::StackItem;

pub(crate) enum TextFormat {
    Bold,
    Italic,
    Underlined,
}

pub(crate) fn print_formatted_at(stdout: &mut Stdout, text: &str, formats: &[TextFormat], x: u16, y: u16) {
    execute!(stdout, MoveTo(x, y)).unwrap();
    for format in formats {
        match format {
            TextFormat::Bold => execute!(stdout, SetAttribute(Attribute::Bold)).unwrap(),
            TextFormat::Italic => execute!(stdout, SetAttribute(Attribute::Italic)).unwrap(),
            TextFormat::Underlined => execute!(stdout, SetAttribute(Attribute::Underlined)).unwrap(),
        }
    }
    execute!(stdout, Print(text)).unwrap();
    execute!(stdout, SetAttribute(Attribute::Reset)).unwrap();
}

// Helper function to format a StackItem for display
pub(crate) fn format_stack_item(item: &StackItem) -> String {
    match item {
        StackItem::Number(num) => format!("{:.2}", num),
        StackItem::Array(arr) => {
            let formatted_elements: Vec<String> = arr.iter().map(|n| format!("{:.2}", n)).collect();
            format!("[{}]", formatted_elements.join(", "))
        },
    }
}