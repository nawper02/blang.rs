// blang.rs is a command line rpn calculator.

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
    Insert,
    Command,
}

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
    // ... other fields ...
}

fn main() {
    let mut stdout = stdout();

    // enable raw mode
    enable_raw_mode().unwrap();

    // hide the cursor
    stdout.execute(Hide).unwrap();

    // initialize app context
    let mut context = AppContext {
        input_buffer: String::new(),
        terminal_size: TerminalSize::new(),
        current_mode: AppMode::Command,
    };

    // enter user input loop
    input_loop(&mut context, &mut stdout);

    // clean up
    disable_raw_mode().unwrap();
    stdout.execute(Show).unwrap(); // Show the cursor again
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

fn update_input_area(stdout: &mut Stdout, context: &AppContext) {
    // Clear the bottom line
    execute!(stdout, MoveTo(0, context.terminal_size.rows - 1), Clear(ClearType::CurrentLine)).unwrap();

    // Set background and foreground colors for the bottom line
    execute!(
        stdout,
        SetBackgroundColor(Color::White),
        SetForegroundColor(Color::Black)
    ).unwrap();

    // Fill the bottom line with spaces to create a white background
    let blank_line = " ".repeat(context.terminal_size.cols as usize);
    execute!(stdout, Print(&blank_line)).unwrap();

    // Move the cursor back to the beginning of the line and print the input buffer
    execute!(stdout, MoveTo(0, context.terminal_size.rows - 1)).unwrap();
    let padded_input = format!(" {} ", &context.input_buffer);
    execute!(stdout, Print(&padded_input)).unwrap();

    // Reset colors
    execute!(
        stdout,
        SetBackgroundColor(Color::Reset),
        SetForegroundColor(Color::Reset)
    ).unwrap();
}

fn process_event(event: Event, context: &mut AppContext, stdout: &mut Stdout) -> LoopControl {
    match event {
        Event::Key(key_event) => match key_event.code {
            KeyCode::Char('q') if context.current_mode == AppMode::Command => return LoopControl::Break,
            KeyCode::Esc => context.current_mode = AppMode::Command,
            KeyCode::Char('i') => context.current_mode = AppMode::Insert,
            KeyCode::Backspace => {
                if context.current_mode == AppMode::Insert {
                    context.input_buffer.pop();
                }
            },
            KeyCode::Enter => {
                if context.current_mode == AppMode::Insert {
                    // process_buffer(&context.input_buffer);
                    context.input_buffer.clear();
                }
            },
            KeyCode::Char(c) => {
                if context.current_mode == AppMode::Insert {
                    context.input_buffer.push(c);
                }
            },
            _ => {},
        },
        Event::Resize(new_cols, new_rows) => {
            context.terminal_size.update(new_cols, new_rows);
            update_graphics(stdout, context);
        },
        _ => {},
    }
    LoopControl::Continue
}

fn update_graphics(stdout: &mut Stdout, context: &AppContext) {
    execute!(stdout, Clear(ClearType::All)).unwrap();

    let formats = [TextFormat::Bold];
    let mode_text = match context.current_mode {
        AppMode::Insert => "Insert Mode",
        AppMode::Command => "Command Mode",
    };

    print_formatted_at(stdout, "blang.rs", &formats, context.terminal_size.cols / 2 - 4, 0);
    print_formatted_at(stdout, mode_text, &formats, 0, context.terminal_size.rows - 2); // Display mode

    update_input_area(stdout, context);
}

fn input_loop(context: &mut AppContext, stdout: &mut Stdout) {
    update_graphics(stdout, context);

    loop {
        if poll(std::time::Duration::from_millis(17)).unwrap() {
            let event = read().unwrap();
            match process_event(event, context, stdout) {
                LoopControl::Continue => {
                    update_graphics(stdout, context);
                },
                LoopControl::Break => break,
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
