use crossterm::terminal::size;

pub(crate) struct TerminalSize {
    pub cols: u16,
    pub rows: u16,
}

impl TerminalSize {
    pub fn new() -> TerminalSize {
        let (cols, rows) = size().unwrap();
        TerminalSize { cols, rows }
    }

    pub fn update(&mut self, new_cols: u16, new_rows: u16) {
        self.cols = new_cols;
        self.rows = new_rows;
    }
}