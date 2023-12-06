use std::io::Stdout;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};

use crate::app::context::AppContext;
use crate::ui::drawables::{BorderDrawer, Drawable, InputAreaUpdater, MainAreaUpdater};
use crate::ui::text_formatting::{print_formatted_at, TextFormat};

pub fn update_graphics(stdout: &mut Stdout, context: &AppContext) {
    execute!(stdout, Clear(ClearType::All)).unwrap();

    InputAreaUpdater::draw(stdout, context);
    MainAreaUpdater::draw(stdout, context);
    BorderDrawer::draw(stdout, context);

    // print title after to write over top border
    print_formatted_at(stdout, " blang.rs ", &[TextFormat::Bold], context.terminal_size.cols / 2 - 4, 0);

}
