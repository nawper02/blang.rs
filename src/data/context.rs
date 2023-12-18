// src/data/context

use crate::stack::functions::route_function_call;
use crate::utils::misc::LoopControl;
use crate::utils::terminal::TerminalSize;
use crate::stack::item::StackItem;

pub struct AppContext {
    pub input_buffer: String,
    pub terminal_size: TerminalSize,
    pub current_mode: AppMode,
    pub should_quit: LoopControl,
    pub stack: Vec<StackItem>,
}

impl Default for AppContext {
    fn default() -> Self {
        AppContext {
            input_buffer: String::new(),
            terminal_size: TerminalSize::new(),
            current_mode: AppMode::Stack,
            should_quit: LoopControl::Continue,
            stack: vec![
                StackItem::Number(3.14),
                StackItem::Array(vec![vec![1.0, 2.0, 3.0]]), // Represents a 1D array
                StackItem::Array(vec![vec![4.0, 5.0, 6.0], vec![7.0, 8.0, 9.0]]), // Represents a 2D array
            ],
            //stack: Vec::new(),
        }
    }
}

#[derive(PartialEq)]
pub(crate) enum AppMode {
    Stack,
    Program,
    Matrix,
    Variables,
}

impl AppMode {
    pub(crate) fn next(&self) -> AppMode {
        match self {
            AppMode::Stack => AppMode::Program,
            AppMode::Program => AppMode::Matrix,
            AppMode::Matrix => AppMode::Variables,
            AppMode::Variables => AppMode::Stack,
        }
    }
}

pub(crate) trait ContextInteraction {
    fn on_up_arrow(&mut self);
    fn on_down_arrow(&mut self);
    fn on_left_arrow(&mut self);
    fn on_right_arrow(&mut self);
}

impl ContextInteraction for AppContext {
    fn on_up_arrow(&mut self) {
        match self.current_mode {
            AppMode::Stack => { /* Stack-specific logic */ },
            AppMode::Program => { /* Program-specific logic */ },
            AppMode::Matrix => { /* Matrix-specific logic */ },
            AppMode::Variables => { /* Variables-specific logic */ },
        }
    }

    fn on_down_arrow(&mut self) {
        // Implement the logic for down arrow
    }

    fn on_left_arrow(&mut self) {
        // Implement the logic for left arrow
    }

    fn on_right_arrow(&mut self) {
        match self.current_mode {
            AppMode::Stack => {
                // Use route_function_call to call the dup function
                if let Err(e) = route_function_call("dup".to_string(), Vec::new(), self) {
                    // todo: handle error
                }
            },
            AppMode::Program => { /* Program-specific logic */ },
            AppMode::Matrix => { /* Matrix-specific logic */ },
            AppMode::Variables => { /* Variables-specific logic */ },
        }
    }
}
