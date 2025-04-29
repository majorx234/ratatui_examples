use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Backend, Buffer, CrosstermBackend, Rect, Terminal},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, Gauge, Widget},
    DefaultTerminal, Frame,
};
use std::{
    env::args,
    io::{self, stdout, Stdout},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    //    let terminal = ratatui::init();
    let mut terminal = init_tui()?;
    let mut app: App = App::new();
    let app_result = app.run(&mut terminal);
    restore_tui()?;
    app_result
}

pub fn init_tui() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore_tui() -> io::Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub struct App {
    should_exit: bool,
}

impl App {
    fn new() -> Self {
        Self { should_exit: false }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press
                && (key.code == KeyCode::Char('q')
                    || key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL)
            {
                self.should_exit = true;
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buffer: &mut Buffer)
    where
        Self: Sized,
    {
        Line::from("Progress overview").bold().render(area, buffer);
    }
}
