// blang.rs is a command line rpn calculator.

// crossterm gives us control over the console.
use crossterm::{
    execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor, SetAttribute, Attribute},
    cursor::{MoveTo, Hide, Show},
    event::{poll, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, size},
    ExecutableCommand,
};
use std::io::{stdout, Write, Stdout};

enum Format {
    Bold,
    Italic,
    Underlined,
}

fn main() {
    let mut stdout = stdout();

    // enable raw mode
    enable_raw_mode().unwrap();

    // hide the cursor
    stdout.execute(Hide).unwrap();

    // input buffer
    let mut input_buffer = String::new();

    // enter user input loop
    input_loop(&mut input_buffer, &mut stdout);

    // clean up
    disable_raw_mode().unwrap();
    stdout.execute(Show).unwrap(); // Show the cursor again
    println!("\nExited.");
}

fn print_formatted_at(stdout: &mut Stdout, text: &str, formats: &[Format], x: u16, y: u16) {
    execute!(stdout, MoveTo(x, y)).unwrap();
    for format in formats {
        match format {
            Format::Bold => execute!(stdout, SetAttribute(Attribute::Bold)).unwrap(),
            Format::Italic => execute!(stdout, SetAttribute(Attribute::Italic)).unwrap(),
            Format::Underlined => execute!(stdout, SetAttribute(Attribute::Underlined)).unwrap(),
        }
    }
    execute!(stdout, Print(text)).unwrap();
    execute!(stdout, SetAttribute(Attribute::Reset)).unwrap();
}

fn update_input_area(stdout: &mut Stdout, input_buffer: &String, cols: u16, rows: u16) {
    // Clear the bottom line
    execute!(stdout, MoveTo(0, rows - 1), Clear(ClearType::CurrentLine)).unwrap();

    // Set background and foreground colors for the bottom line
    execute!(
        stdout,
        SetBackgroundColor(Color::White),
        SetForegroundColor(Color::Black)
    ).unwrap();

    // Fill the bottom line with spaces to create a white background
    let blank_line = " ".repeat(cols as usize);
    execute!(stdout, Print(&blank_line)).unwrap();

    // Move the cursor back to the beginning of the line and print the input buffer
    execute!(stdout, MoveTo(0, rows - 1)).unwrap();
    let padded_input = format!(" {} ", input_buffer); // Adding space before and after the text
    execute!(stdout, Print(&padded_input)).unwrap();

    // Reset colors
    execute!(
        stdout,
        SetBackgroundColor(Color::Reset),
        SetForegroundColor(Color::Reset)
    ).unwrap();
}

fn process_event(event: Event, input_buffer: &mut String, stdout: &mut Stdout, cols: u16, rows: u16) -> (bool, u16, u16) {
    match event {
        Event::Key(key_event) => {
            match key_event.code {
                KeyCode::Char('q') => return (false, cols, rows),
                KeyCode::Backspace => {input_buffer.pop();},
                KeyCode::Enter => {
                    //process_buffer(input_buffer);
                    input_buffer.clear();
                },
                KeyCode::Char(c) => {input_buffer.push(c);},
                _ => {},
            }
            (true, cols, rows) // Continue the loop
        },
        Event::Resize(new_cols, new_rows) => {
            update_graphics(stdout, input_buffer, new_cols, new_rows);
            (true, new_cols, new_rows) // New dimensions
        },
        _ => (true, cols, rows), // Continue the loop with current dimensions
    }
}

fn update_graphics(stdout: &mut Stdout, input_buffer: &String, cols: u16, rows: u16) {
    // Clear the screen and redraw title and input area
    execute!(stdout, Clear(ClearType::All)).unwrap();
    let formats = [Format::Bold, Format::Italic]; // Adjust as needed
    print_formatted_at(stdout, "blang.rs", &formats, cols / 2 - 4, 0); // Center title
    update_input_area(stdout, input_buffer, cols, rows);
}

fn input_loop(input_buffer: &mut String, stdout: &mut Stdout) {
    let (mut cols, mut rows) = size().unwrap();

    loop {

        update_graphics(stdout, input_buffer, cols, rows);


        if poll(std::time::Duration::from_millis(500)).unwrap() {

            let event = read().unwrap();
            let (continue_loop, new_cols, new_rows) = process_event(event, input_buffer, stdout, cols, rows);

            if !continue_loop {
                break;
            }

            // Update cols and rows with possibly new dimensions
            cols = new_cols;
            rows = new_rows;
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
