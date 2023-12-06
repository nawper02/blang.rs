// src/app/context

use super::mode::AppMode;
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

