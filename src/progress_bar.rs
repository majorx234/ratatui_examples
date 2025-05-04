use color_eyre::owo_colors::OwoColorize;
use color_eyre::Result;
use crossbeam_channel::{unbounded, Receiver, Sender};
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
    thread::{self, sleep},
};
mod dummy_thread;
use dummy_thread::Dummy;

fn main() -> Result<()> {
    color_eyre::install()?;
    //    let terminal = ratatui::init();
    let mut terminal = init_tui()?;
    let (thread_join_handle, tx_close, rx_status) = Dummy::start();
    let mut app: App = App::new(thread_join_handle, tx_close, rx_status);
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

enum KeyMsg {
    CLOSE,
    CHANGE_COLOR,
}

pub struct App {
    should_exit: bool,
    progress_bar_color_idx: u8,
    progress_name: String,
    progress_ratio: f64,
    progress_thread_jh: thread::JoinHandle<()>,
    input_thread_jh: thread::JoinHandle<()>,
    tx_close: Sender<bool>,
    tx_close_input: Sender<bool>,
    rx_progress: Receiver<f64>,
    rx_keymsg: Receiver<KeyMsg>,
}

impl App {
    fn new(
        progress_thread_jh: thread::JoinHandle<()>,
        tx_close: Sender<bool>,
        rx_progress: Receiver<f64>,
    ) -> Self {
        let (tx_close_input, rx_close_input) = unbounded::<bool>();
        let (tx_keymsg, rx_keymsg) = unbounded();
        let input_thread_jh = std::thread::spawn(move || {
            let mut run = true;

            while run {
                println!("input get key");
                if let Event::Key(key) = event::read().expect("read sth from input") {
                    if key.kind == KeyEventKind::Press
                        && (key.code == KeyCode::Char('q')
                            || key.code == KeyCode::Char('c')
                                && key.modifiers == KeyModifiers::CONTROL)
                    {
                        let _ = tx_keymsg.send(KeyMsg::CLOSE);
                    } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('c') {
                        let _ = tx_keymsg.send(KeyMsg::CHANGE_COLOR);
                    }
                }
                match rx_close_input.try_recv() {
                    Ok(close) => {
                        println!("receive close: {}", close);
                        run = !close;
                    }
                    Err(_) => {}
                }
            }
            println!("input end");
        });
        Self {
            should_exit: false,
            progress_bar_color_idx: 0,
            progress_name: "Process 1".to_string(),
            progress_ratio: 0.0,
            progress_thread_jh,
            tx_close,
            tx_close_input,
            rx_progress,
            input_thread_jh,
            rx_keymsg,
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
            sleep(std::time::Duration::from_millis(10));
        }
        let _ = self.tx_close.send(true);
        let _ = self.tx_close_input.send(true);
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Ok(key_msg) = self.rx_keymsg.try_recv() {
            match key_msg {
                KeyMsg::CHANGE_COLOR => {
                    self.progress_bar_color_idx += 1;
                }
                KeyMsg::CLOSE => {
                    self.should_exit = true;
                }
            }
        }
        if let Ok(progress_msg) = self.rx_progress.try_recv() {
            self.progress_ratio = progress_msg / 100.0;
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
        let color_map = [
            Color::Green,
            Color::Blue,
            Color::Red,
            Color::Cyan,
            Color::Magenta,
        ];
        let color_idx = (self.progress_bar_color_idx % 5) as usize;
        let color = color_map[color_idx];
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
            .gauge_style(Style::default().fg(color))
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
