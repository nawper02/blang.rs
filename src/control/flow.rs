use crossterm::event::{Event, KeyCode};
use std::io::Stdout;
use crate::data::context::AppContext;
use crate::control::{parsing, visualization};

// function that takes parsed inputs and routes them to functions in functions.rs
pub(crate) fn stack_mode_flow(parsed: Vec<&str>) {println!("{:?}", parsed)}
pub(crate) fn program_mode_flow(parsed: Vec<&str>) {}
pub(crate) fn variables_mode_flow(parsed: Vec<&str>) {}
pub(crate) fn matrix_mode_flow(parsed: Vec<&str>) {}

pub(crate) fn process_event(event: Event, context: &mut AppContext, stdout: &mut Stdout) {
    match event {
        Event::Key(key_event) => {
            match key_event.code {
                KeyCode::Tab => {
                    // change mode
                    //context.input_buffer.clear();
                    context.current_mode = context.current_mode.next();
                },
                KeyCode::Backspace => {
                    // delete
                    context.input_buffer.pop();
                }
                KeyCode::Enter => {
                    // send buffer to be parsed
                    parsing::parse_input(context);
                    context.input_buffer.clear();
                },
                KeyCode::Char(c) =>  {
                    context.input_buffer.push(c);
                    // for single character commands (getch style)
                    parsing::parse_quick_cmds(context)
                },
                _ => {},
            }
        },
        Event::Resize(new_cols, new_rows) => {
            context.terminal_size.update(new_cols, new_rows);
            visualization::update_graphics(stdout, context);
        },
        _ => {},
    }
}
