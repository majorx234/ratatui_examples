use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rand::{thread_rng, Rng};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Backend, CrosstermBackend, Terminal},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block},
    Frame,
};
use std::{
    env::args,
    io::{self, stdout, Stdout},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    //    let terminal = ratatui::init();
    let terminal = init_tui()?;
    let app_result = App::new().run(terminal);
    restore_tui()?;
    app_result
}

pub fn init_tui() -> io::Result<Terminal<impl Backend>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore_tui() -> io::Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

struct App {
    should_exit: bool,
    book_list: Vec<String>,
}

impl App {
    fn new() -> Self {
        let mut book_list = Vec::new();
        book_list.push("Short History of Time".to_string());
        book_list.push("War and Piece".to_string());
        book_list.push("Mobby Dick".to_string());
        book_list.push("Midsummer Dream".to_string());
        book_list.push("The Picture of Dorian Gray".to_string());
        book_list.push("Goedel Escher, Bach".to_string());
        book_list.push("The Yellow King".to_string());
        book_list.push("Rashomon".to_string());
        book_list.push("High Fidelity".to_string());
        book_list.push("A long Way down".to_string());
        book_list.push("Hanami".to_string());
        book_list.push("Entangled Life".to_string());
        book_list.push("Guns, Germs and Steel".to_string());
        book_list.push("Ready Player One".to_string());
        book_list.push("Lost Music".to_string());
        book_list.push("The Book of Drexciya".to_string());
        book_list.push("War and Piece".to_string());
        book_list.push("My Catalonia".to_string());
        book_list.push("Art of Programming".to_string());
        book_list.push("Who rules the World".to_string());

        Self {
            should_exit: false,
            book_list,
        }
    }

    fn run(mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                self.should_exit = true;
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let [title, vertical, horizontal] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .spacing(1)
        .areas(frame.area());

        frame.render_widget("Fuzzy Find a Book".bold().into_centered_line(), title);
    }
}
