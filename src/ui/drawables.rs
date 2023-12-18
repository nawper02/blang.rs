use std::io::Stdout;
use crossterm::execute;
use crossterm::style::{Print};
use crossterm::cursor::MoveTo;
use crossterm::terminal::{Clear, ClearType};

use crate::data::context::AppContext;
use crate::data::context::AppMode;
use crate::ui::text_formatting::{print_formatted_at, format_stack_item, TextFormat};
use crate::stack::item::StackItem;

pub(crate) trait Drawable {
    fn draw(stdout: &mut Stdout, context: &AppContext); // perhaps in the future it will take &self.
}

pub(crate) struct BorderDrawer;

impl Drawable for BorderDrawer {
    fn draw(stdout: &mut Stdout, context: &AppContext) {
        let horizontal_edge = "─".repeat((context.terminal_size.cols - 2) as usize);
        let vertical_edge = "│";

        execute!(stdout, MoveTo(0, 0), Print("┌"), Print(&horizontal_edge), Print("┐")).unwrap();
        for row in 1..context.terminal_size.rows - 1 {
            execute!(stdout, MoveTo(0, row), Print(vertical_edge)).unwrap();
            execute!(stdout, MoveTo(context.terminal_size.cols - 1, row), Print(vertical_edge)).unwrap();
        }
        execute!(stdout, MoveTo(0, context.terminal_size.rows - 1), Print("└"), Print(&horizontal_edge), Print("┘")).unwrap();
    }
}

pub(crate) struct InputAreaUpdater;

impl Drawable for InputAreaUpdater {
    fn draw(stdout: &mut Stdout, context: &AppContext) {
        let start_col = 1;
        let end_col = context.terminal_size.cols - 1;
        let input_row = context.terminal_size.rows - 2;

        execute!(stdout, MoveTo(start_col, input_row), Clear(ClearType::CurrentLine)).unwrap();
        let max_buffer_length = end_col as usize - start_col as usize;
        let display_buffer = format!(" » {} ", context.input_buffer);

        let padded_input = if display_buffer.len() > max_buffer_length {
            format!("{}...", &display_buffer[..max_buffer_length - 3])
        } else {
            format!("{:width$}", display_buffer, width = max_buffer_length)
        };

        execute!(stdout, MoveTo(start_col, input_row), Print(&padded_input)).unwrap();
    }
}

pub(crate) struct MainAreaUpdater;

impl Drawable for MainAreaUpdater {
    fn draw(stdout: &mut Stdout, context: &AppContext) {
        let mode_text = match context.current_mode {
            AppMode::Stack => " stack",
            AppMode::Program => " program",
            AppMode::Matrix => " matrix",
            AppMode::Variables => " variables",
        };

        print_formatted_at(stdout, mode_text, &[TextFormat::Bold], 1, context.terminal_size.rows - 3);

        match context.current_mode {
            AppMode::Stack => StackDisplay::draw(stdout, context),
            AppMode::Program => {},  // Implement as needed
            AppMode::Matrix => {},   // Implement as needed
            AppMode::Variables => {}, // Implement as needed
        }
    }
}

pub(crate) struct StackDisplay;

impl Drawable for StackDisplay {
    fn draw(stdout: &mut Stdout, context: &AppContext) {
        let stack_display_start = 1; // Top row
        let stack_display_end = context.terminal_size.rows - 3 - 2; // Just above the mode text row

        let mut display_row = stack_display_end;

        // Iterate over the stack in reverse
        for stack_index in (0..context.stack.len()).rev() {
            let item = &context.stack[stack_index];
            let display_index = context.stack.len() - 1 - stack_index; // Correct the index order

            match item {
                StackItem::Number(num) => {
                    // For Number, display it on a single line
                    if display_row > stack_display_start {
                        let line = format!("{:2}: {:.2}", display_index, num);
                        execute!(stdout, MoveTo(2, display_row), Print(line)).unwrap();
                        display_row -= 1;
                    }
                },
                StackItem::Array(arr) => {
                    // For Array, display its dimensions
                    if display_row > stack_display_start {
                        let array_type = if arr.len() == 1 { "1D" } else { "2D" };
                        let dimensions = if arr.len() == 1 { format!("{} elements", arr[0].len()) } else { format!("{}x{} elements", arr.len(), arr[0].len()) };
                        let line = format!("{:2}: {} Array [{}]", display_index, array_type, dimensions);
                        execute!(stdout, MoveTo(2, display_row), Print(line)).unwrap();
                        display_row -= 1;
                    }
                },
            }
            if display_row <= stack_display_start {
                break; // Stop if we've reached the top of the display area
            }
        }

        // Fill remaining lines with '~'
        while display_row > stack_display_start {
            execute!(stdout, MoveTo(2, display_row), Print(format!(" ~"))).unwrap();
            display_row -= 1;
        }
    }
}
