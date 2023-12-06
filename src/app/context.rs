// src/app/context

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
            stack: Vec::new(),
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

