// blang.rs is a command line rpn calculator.

// next:
//      0.
//      1.   stack view -- impl mode behavior to select stack items, commands (for all types)
//      2.   program view, text editing capabilites (unless just load files)
//      3.   variables view, arrow key to navigate, enter to select and push
//      4.   matrix view, arrow keys to navigate, input buffer routed to cells
// refactor:

#![allow(dead_code)]
#![allow(unused)]

mod data;
mod stack;
mod ui;
mod utils;
mod control;

use std::io::{stdout, Stdout};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{poll, read};
use crossterm::ExecutableCommand;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use data::context::AppContext;
use data::context::AppMode;
use control::{flow, parsing, visualization};
use ui::drawables::{BorderDrawer, Drawable, InputAreaUpdater, MainAreaUpdater};
use ui::text_formatting::{print_formatted_at, TextFormat};
use utils::misc::LoopControl;

fn main() {
    let mut stdout = stdout();
    let mut context = AppContext::default();

    init(&mut stdout);
    program_loop(&mut context, &mut stdout);
    tini(&mut stdout);
}

fn init(stdout: &mut Stdout) {
    enable_raw_mode().unwrap();
    stdout.execute(Hide).unwrap();
}

fn program_loop(context: &mut AppContext, stdout: &mut Stdout) {
    // initial graphics update
    visualization::update_graphics(stdout, context);

    loop {
        // check for events at 60hz
        if poll(std::time::Duration::from_millis(17)).unwrap() {
            let event = read().unwrap();
            flow::process_event(event, context, stdout);
            // exit condition
            match context.should_quit {
                LoopControl::Continue => {
                    // refresh
                    visualization::update_graphics(stdout, context);
                }
                LoopControl::Break => {
                    // quit program
                    break
                }
            }
        }
    }
}

fn tini(stdout: &mut Stdout) {
    disable_raw_mode().unwrap();
    stdout.execute(Show).unwrap();
    println!("\nblang done. thank you.");
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
