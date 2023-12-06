use crate::control::flow::{matrix_mode_flow, program_mode_flow, stack_mode_flow, variables_mode_flow};
use crate::data::context::{AppContext, AppMode};
use crate::utils::misc::LoopControl;

pub(crate) fn parse_quick_cmds(context: &mut AppContext) {
    match context.input_buffer.as_str() {
        "quit" => {
            context.should_quit = LoopControl::Break;
            context.input_buffer.clear()
        },
        "*" => {
            context.input_buffer.clear()
        },
        "/" => {
            context.input_buffer.clear()
        },
        "+" => {
            context.input_buffer.clear()
        },
        "-" => {
            context.input_buffer.clear()
        },
        "^" => {
            context.input_buffer.clear()
        },
        _ => {}
    }
}

pub(crate) fn parse_input(context: &mut AppContext) {
    // parse input into usable data.
    let buf: String = context.input_buffer.clone();
    let parsed: Vec<&str> = buf.split_whitespace().collect(); // placeholder

    // route parsed input into respective flow.
    match context.current_mode {
        AppMode::Stack => {
            stack_mode_flow(parsed);
        }
        AppMode::Program => {
            program_mode_flow(parsed);
        }
        AppMode::Variables => {
            variables_mode_flow(parsed);
        }
        AppMode::Matrix => {
            matrix_mode_flow(parsed);
        }
    }
}
