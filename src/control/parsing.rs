use crate::data::context::{AppContext, AppMode};
use crate::utils::misc::LoopControl;

pub(crate) fn parse_quick_cmds(context: &mut AppContext) {
    match context.input_buffer.as_str() {
        "quit" => {
            context.should_quit = LoopControl::Break;
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
    // what we do with the input depends on what mode we are in.
    match context.current_mode {
        AppMode::Stack => {

        }
        AppMode::Program => {

        }
        AppMode::Variables => {

        }
        AppMode::Matrix => {

        }
    }
}
