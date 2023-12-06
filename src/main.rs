// blang.rs is a command line rpn calculator.

// next:
//      1.   stack view, commands
//      2.   program view, text editing capabilites (unless just load files)
//      3.   variables view, arrow key to navigate, enter to select and push
//      4.   matrix view, arrow keys to navigate, input buffer routed to cells
// refactor:
//      1. move stuff to separate files

#![allow(dead_code)]
#![allow(unused)]

use crossterm::{
    execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor, SetAttribute, Attribute},
    cursor::{MoveTo, Hide, Show},
    event::{poll, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, size},
    ExecutableCommand,
};
use std::io::{stdout, Stdout};

enum TextFormat {
    Bold,
    Italic,
    Underlined,
}

#[derive(PartialEq)]
enum AppMode {
    Stack,
    Program,
    Matrix,
    Variables,
}

impl AppMode {
    fn next(&self) -> AppMode {
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

enum StackItem {
    Number(f64),
    Array(Vec<f64>),
}

struct TerminalSize {
    cols: u16,
    rows: u16,
}

impl TerminalSize {
    fn new() -> TerminalSize {
        let (cols, rows) = size().unwrap();
        TerminalSize { cols, rows }
    }

    fn update(&mut self, new_cols: u16, new_rows: u16) {
        self.cols = new_cols;
        self.rows = new_rows;
    }
}

struct AppContext {
    input_buffer: String,
    terminal_size: TerminalSize,
    current_mode: AppMode,
    should_quit: LoopControl,
    stack: Vec<StackItem>
}

trait Drawable {
    fn draw(stdout: &mut Stdout, context: &AppContext); // perhaps in the future it will take &self.
}

struct BorderDrawer;

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

struct InputAreaUpdater;

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

struct MainAreaUpdater;

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

struct StackDisplay;

impl Drawable for StackDisplay {
    fn draw(stdout: &mut Stdout, context: &AppContext) {
        let stack_display_start = 1; // Top row
        let stack_display_end = context.terminal_size.rows - 3 - 1; // Just above the mode text row
        let stack_size = context.stack.len();
        let total_display_rows = (stack_display_end - stack_display_start) as usize;

        for row in 0..total_display_rows {
            let display_row = stack_display_end - 1 - row as u16;
            // Calculate the index in the stack for this row
            let display_index = stack_size.saturating_sub(total_display_rows) + row;

            let line = if display_index < stack_size {
                // If the index is within the stack, display the stack item
                let item = &context.stack[display_index];
                format!("{:2}: {}", display_index, format_stack_item(item))
            } else {
                // If the index is outside the stack, display just the index
                format!("{:2}: ", display_index)
            };
            execute!(stdout, MoveTo(2, display_row), Print(line)).unwrap();
        }
    }
}

fn main() {
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    stdout.execute(Hide).unwrap();

    let mut context = AppContext {
        input_buffer: String::new(),
        terminal_size: TerminalSize::new(),
        current_mode: AppMode::Stack,
        should_quit: LoopControl::Continue,
        stack: Vec::new(),
    };

    program_loop(&mut context, &mut stdout);

    disable_raw_mode().unwrap();
    stdout.execute(Show).unwrap();
    println!("\nblang done. thank you.");
}

fn print_formatted_at(stdout: &mut Stdout, text: &str, formats: &[TextFormat], x: u16, y: u16) {
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
fn format_stack_item(item: &StackItem) -> String {
    match item {
        StackItem::Number(num) => format!("{:.2}", num),
        StackItem::Array(arr) => {
            let formatted_elements: Vec<String> = arr.iter().map(|n| format!("{:.2}", n)).collect();
            format!("[{}]", formatted_elements.join(", "))
        },
    }
}

fn update_graphics(stdout: &mut Stdout, context: &AppContext) {
    execute!(stdout, Clear(ClearType::All)).unwrap();

    InputAreaUpdater::draw(stdout, context);
    MainAreaUpdater::draw(stdout, context);
    BorderDrawer::draw(stdout, context);

    // print title after to write over top border
    print_formatted_at(stdout, " blang.rs ", &[TextFormat::Bold], context.terminal_size.cols / 2 - 4, 0);

}

fn process_event(event: Event, context: &mut AppContext, stdout: &mut Stdout) {
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
                    parse_input(context);
                    context.input_buffer.clear();
                },
                KeyCode::Char(c) =>  {
                    context.input_buffer.push(c);
                    // for single character commands (getch style)
                    handle_quick_commands(context)
                },
                _ => {},
            }
        },
        Event::Resize(new_cols, new_rows) => {
            context.terminal_size.update(new_cols, new_rows);
            update_graphics(stdout, context);
        },
        _ => {},
    }
}

fn handle_quick_commands(context: &mut AppContext) {
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

fn parse_input(context: &mut AppContext) {
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

fn program_loop(context: &mut AppContext, stdout: &mut Stdout) {
    // initial graphics update
    update_graphics(stdout, context);

    loop {
        // check for events at 60hz
        if poll(std::time::Duration::from_millis(17)).unwrap() {
            let event = read().unwrap();
            process_event(event, context, stdout);
            // exit condition
            match context.should_quit {
                LoopControl::Continue => {
                    // refresh
                    update_graphics(stdout, context);
                }
                LoopControl::Break => {
                    // quit program
                    break
                }
            }
        }
    }
}

/*
             @@@@@@@@@@@@@%                        @@@@@@@.
        ,@@@@@@@@@@@@      @@@@.                         .,@@@,
      @@@@@@@@@@@@@@@          @@,                           .@@@.
    @@@@@@@@@@@@@@@@@            @@,                            @@@.
   @@@@@@@@@@@@@@@@@@             @@*         @@@@@@@@@@/         @@,
  @@@@@@@@@@@@@@@@@@@              @@,      @@@@@@@@*. .@@@        @@.
  @@ Kin Blandford @@              (@#.    @@@@@@@@@*.   @@*.      @@,
  @@@@@@@@@@@@@@@@@@@              *@%.@@, @@@@@@@@@*.   @@*.      @@*
  @@@@@@@@@@@@@@@@@@@              @@*.@@, @@@@@@@@@*.  @@@,       @@*
  @@@@@@@@@@@@@@@@@@@             @@#,@@*. @@*@@@@@@@@@@@*.        @@*
  @@@@@@@@@@@@@@@@@@@            @@/,@@*,  @@*   .@@/,.            @@*
  @@,.@@@@@@@@@@@@@@@          @@@*.@@*.   @@*    @@*.             @@*
  @@,  .@@@@@@@@@@@@@       @@@*,(@@*,/@@@/*,..@@@@@@@@&           @@*
  @@,      ,@@@@@@@@@@@@@@@**.#@@@*,@@@*.@@@@/    @@@@@@@@@@@.     @@*
  @@,            ..@@,..  @@@@**. @@*,@@@         @@@@@@@@@@@@@@.  @@*
  @@,              @@,    @@,   @@/,@@@           @@@@@@@@@@@@@@@@,@@*
  @@,           @@@@@@@,  @@,  @@*.@@             @@@@@@@@@@@@@@@@@@@*
  @@,        @@@   @@@@@@@@@, @@*.@@              @@@@@@@@@@@@@@@@@@@*
  @@,       @@     @@@@@@@@@, @@*/@#              @@@@@@@@@@@@@@@@@@@*
  @@,      &@      @@@@@@@@@, .*.@@               @@@@@@@@@@@@@@@@@@@*
  @@,       @@     @@@@@@@@/.     @@              @@@@@@@@@@@@@@@@@@@,
   @@.       @@@   @@@@@@@*.      @@              @@@@@@@@@@@@@@@@@@*.
   .@@.        .,@@@@@(*,          @@.            @@@@@@@@@@@@@@@@@/,
     @@@.                           .@@           @@@@@@@@@@@@@@@@*.
       @@@,                           .@@@        @@@@@@@@@@@@@@*.
         .*@@@@@                         .@@@@@.  @@@@@@@@@@/*.
              .,,*/(,                         .,*//***,..
 */
