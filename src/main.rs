// blang.rs is a command line rpn calculator.
// next:
//      1.   stack view, commands
//      2.   program view, text editing capabilites (unless just load files)
//      3.   variables view, arrow key to navigate, enter to select and push
//      4.   matrix view, arrow keys to navigate, input buffer routed to cells

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
    };

    input_loop(&mut context, &mut stdout);

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

fn draw_border(stdout: &mut Stdout, context: &AppContext) {
    let horizontal_edge = "─".repeat((context.terminal_size.cols - 2) as usize);
    let vertical_edge = "│";

    // Top border
    execute!(stdout, MoveTo(0, 0), Print("┌"), Print(&horizontal_edge), Print("┐")).unwrap();

    // Side borders
    for row in 1..context.terminal_size.rows - 1 {
        execute!(stdout, MoveTo(0, row), Print(vertical_edge)).unwrap();
        execute!(stdout, MoveTo(context.terminal_size.cols - 1, row), Print(vertical_edge)).unwrap();
    }

    // Bottom border
    execute!(stdout, MoveTo(0, context.terminal_size.rows - 1), Print("└"), Print(&horizontal_edge), Print("┘")).unwrap();
}

fn update_input_area(stdout: &mut Stdout, context: &AppContext) {
    // Determine the position for the input area within the border
    let start_col = 1; // Start one column to the right due to the border
    let end_col = context.terminal_size.cols - 1; // End one column before the right border
    // let start_row = 1;
    // let end_row = context.terminal_size.rows - 1;
    let input_row = context.terminal_size.rows - 2;

    // Clear the previous input area line
    execute!(stdout, MoveTo(start_col, input_row), Clear(ClearType::CurrentLine)).unwrap();

    // Set background and foreground color for input area
    //execute!(stdout, SetBackgroundColor(Color::Black), SetForegroundColor(Color::White)).unwrap();

    // Calculate the maximum length of the buffer display
    let max_buffer_length = end_col as usize - start_col as usize;
    let display_buffer = format!(" » {} ", context.input_buffer); // spaces either side for padding

    // Trim or pad the buffer to fit within the bordered area
    let padded_input = if display_buffer.len() > max_buffer_length {
        // Trim the buffer and add ellipsis if it's too long
        format!("{}...", &display_buffer[..max_buffer_length - 3])
    } else {
        // Pad the buffer if it's too short
        format!("{:width$}", display_buffer, width = max_buffer_length)
    };

    // Print the adjusted buffer
    execute!(stdout, MoveTo(start_col, input_row), Print(&padded_input)).unwrap();

    // Reset background and foreground color
    execute!(stdout, SetBackgroundColor(Color::Reset), SetForegroundColor(Color::Reset)).unwrap();
}

fn update_main_area(stdout: &mut Stdout, context: &AppContext) {

    let mode_text = match context.current_mode {
        AppMode::Stack => " stack",
        AppMode::Program => " program",
        AppMode::Matrix => " matrix",
        AppMode::Variables => " variables",
    };

    // print AppMode text
    print_formatted_at(stdout, mode_text, &[TextFormat::Bold], 1, context.terminal_size.rows - 3);
}


fn update_graphics(stdout: &mut Stdout, context: &AppContext) {
    execute!(stdout, Clear(ClearType::All)).unwrap();

    // display main stuff here (maybe move above two appmode text stuff into this)
    update_main_area(stdout, context);

    // update input area
    update_input_area(stdout, context);

    // draw border after other stuff because of calls to Clear(ClearType::CurrentLine)
    draw_border(stdout, context);

    // print title after to write over top border
    print_formatted_at(stdout, " blang.rs ", &[TextFormat::Bold], context.terminal_size.cols / 2 - 4, 0);

}

fn process_event(event: Event, context: &mut AppContext, stdout: &mut Stdout) {
    match event {
        Event::Key(key_event) => {
            match key_event.code {
                KeyCode::Tab => {
                    //context.input_buffer.clear();
                    context.current_mode = context.current_mode.next();
                },
                KeyCode::Backspace => {
                    context.input_buffer.pop();
                }
                KeyCode::Enter => {
                    parse_input(context);
                    context.input_buffer.clear();
                },
                KeyCode::Char(c) =>  {
                    context.input_buffer.push(c);
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

fn input_loop(context: &mut AppContext, stdout: &mut Stdout) {
    update_graphics(stdout, context);

    loop {
        if poll(std::time::Duration::from_millis(17)).unwrap() {
            let event = read().unwrap();
            process_event(event, context, stdout);
            match context.should_quit {
                LoopControl::Continue => {
                    update_graphics(stdout, context);
                }
                LoopControl::Break => {
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
