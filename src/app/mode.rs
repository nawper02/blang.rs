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

#[derive(PartialEq)]
enum LoopControl {
    Continue,
    Break,
}