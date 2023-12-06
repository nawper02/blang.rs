// blang.rs is a command line rpn calculator.

// next:
//      1.   stack view, commands
//      2.   program view, text editing capabilites (unless just load files)
//      3.   variables view, arrow key to navigate, enter to select and push
//      4.   matrix view, arrow keys to navigate, input buffer routed to cells
// refactor:

#![allow(dead_code)]
#![allow(unused)]

use std::io::{stdout, Stdout};
use crossterm::cursor::{Hide, Show};
use crossterm::{ExecutableCommand, execute};
use crossterm::event::{Event, KeyCode, poll, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

mod app;
mod stack;
mod ui;
mod utils;

use crate::app::context::AppContext;
use crate::app::mode::AppMode;
use crate::ui::drawables::{BorderDrawer, Drawable, InputAreaUpdater, MainAreaUpdater};
use crate::ui::text_formatting::{print_formatted_at, TextFormat};
use crate::utils::misc::LoopControl;

fn main() {
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    stdout.execute(Hide).unwrap();

    let mut context = AppContext::default();

    program_loop(&mut context, &mut stdout);

    disable_raw_mode().unwrap();
    stdout.execute(Show).unwrap();
    println!("\nblang done. thank you.");
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
