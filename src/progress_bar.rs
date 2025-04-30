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
    symbols::border,
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
    progress_bar_color: Color,
    progress_name: String,
    progress_ratio: f64,
}

impl App {
    fn new() -> Self {
        Self {
            should_exit: false,
            progress_bar_color: Color::Blue,
            progress_name: "Process 1".to_string(),
            progress_ratio: 0.5,
        }
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
        let vertical_layout =
            Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)]);
        let [title_area, gauge_area] = vertical_layout.areas(area);

        Line::from("Progress overview")
            .bold()
            .render(title_area, buffer);

        let instruction = Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
            " Change color ".into(),
            "<C> ".blue().bold(),
        ])
        .centered();

        let block = Block::bordered()
            .title(Line::from("Progress overview").bold())
            .title_bottom(instruction)
            .border_set(border::THICK);

        let progress_bar = Gauge::default()
            .gauge_style(Style::default().fg(self.progress_bar_color))
            .block(block)
            .label(format!(
                "{}: {}%",
                self.progress_name,
                self.progress_ratio * 100.0
            ))
            .ratio(self.progress_ratio);
        progress_bar.render(
            Rect {
                x: gauge_area.left(),
                y: gauge_area.top(),
                width: gauge_area.width,
                height: 3,
            },
            buffer,
        );
    }
}
