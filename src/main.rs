// blang.rs is a command line rpn calculator.
// next: border, prettify, stack view, commands, modify how commands are handled to allow for function inputs (not taking immedietly)

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
    command_buffer: String,
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
        command_buffer: String::new(),
        terminal_size: TerminalSize::new(),
        current_mode: AppMode::Command,
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

fn update_input_area(stdout: &mut Stdout, context: &AppContext) {
    execute!(stdout, MoveTo(0, context.terminal_size.rows - 1), Clear(ClearType::CurrentLine)).unwrap();
    execute!(stdout, SetBackgroundColor(Color::White), SetForegroundColor(Color::Black)).unwrap();

    let buffer_to_display = match context.current_mode {
        AppMode::Insert => &context.input_buffer,
        AppMode::Command => &context.command_buffer,
    };

    let blank_line = " ".repeat(context.terminal_size.cols as usize);
    execute!(stdout, Print(&blank_line)).unwrap();

    execute!(stdout, MoveTo(0, context.terminal_size.rows - 1)).unwrap();
    let padded_input = format!(" {} ", buffer_to_display);
    execute!(stdout, Print(&padded_input)).unwrap();

    execute!(stdout, SetBackgroundColor(Color::Reset), SetForegroundColor(Color::Reset)).unwrap();
}

fn process_event(event: Event, context: &mut AppContext, stdout: &mut Stdout) {
    match event {
        Event::Key(key_event) => {
            match key_event.code {
                // catch immediete commands
                KeyCode::Tab => {
                    context.current_mode = match context.current_mode {
                        AppMode::Insert => AppMode::Command,
                        AppMode::Command => AppMode::Insert,
                    };
                },
                KeyCode::Backspace => match context.current_mode {
                    AppMode::Insert => { context.input_buffer.pop(); },
                    AppMode::Command => { context.command_buffer.pop(); },
                },
                KeyCode::Enter => match context.current_mode {
                    AppMode::Insert => {
                        handle_input_buffer(context);
                        context.input_buffer.clear();
                    },
                    AppMode::Command => {
                        handle_command_buffer(context);
                        context.command_buffer.clear();
                    },
                },
                // catch word commands or push to buffers
                KeyCode::Char(c) => match context.current_mode {
                    // if in insert mode, just push keys to buffer
                    AppMode::Insert => { context.input_buffer.push(c); },
                    // if in command mode, push keys to buffer and immedietly handle the buffer
                    AppMode::Command => {
                        context.command_buffer.push(c);
                        handle_command_buffer(context);
                    },
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

fn handle_command_buffer(context: &mut AppContext) {
    match context.command_buffer.as_str() {
        "quit" => {
            context.should_quit = LoopControl::Break;
        },
        "clear" => {
        },
        "echo" => {
        },
        "help" => {
        },
        _ => {}
    }
}

fn handle_input_buffer(context: &mut AppContext) {

}

fn update_graphics(stdout: &mut Stdout, context: &AppContext) {
    execute!(stdout, Clear(ClearType::All)).unwrap();

    let formats = [TextFormat::Bold];
    let mode_text = match context.current_mode {
        AppMode::Insert => "Insert Mode",
        AppMode::Command => "Command Mode",
    };

    print_formatted_at(stdout, "blang.rs", &formats, context.terminal_size.cols / 2 - 4, 0);
    print_formatted_at(stdout, mode_text, &formats, 0, context.terminal_size.rows - 2);

    update_input_area(stdout, context);
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
