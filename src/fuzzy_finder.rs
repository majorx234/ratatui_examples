use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use ratatui::{
    layout::{Constraint, Direction, Layout, Position},
    prelude::{Backend, Buffer, CrosstermBackend, Rect, Terminal},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph, Widget},
    Frame,
};
use std::{
    env::args,
    io::{self, stdout, Stdout},
};
mod helper;
use helper::highlight_char_in_text;

fn main() -> Result<()> {
    color_eyre::install()?;
    //    let terminal = ratatui::init();
    let mut terminal = init_tui()?;
    let mut app: App = App::new();
    let app_result = app.run(&mut terminal);
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
    search_input: String,
    search_input_character_index: usize,
    book_list: Vec<String>,
    result_list: Vec<(i64, Vec<usize>, String)>,
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
            search_input: "".to_string(),
            search_input_character_index: 0,
            result_list: Vec::new(),
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.search_input_character_index.saturating_sub(1);
        self.search_input_character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.search_input_character_index.saturating_add(1);
        self.search_input_character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.search_input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.search_input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.search_input_character_index)
            .unwrap_or(self.search_input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.search_input_character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.search_input_character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.search_input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.search_input.chars().skip(current_index);

            self.search_input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.search_input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.search_input_character_index = 0;
    }

    fn submit_search(&mut self) {
        //self.search(self.input.clone());
        let search_string = self.search_input.clone();
        self.search_input.clear();
        self.reset_cursor();

        // doing matcher stuff
        let matcher = SkimMatcherV2::default();
        let mut result_list: Vec<(i64, Vec<usize>, String)> = Vec::new();
        for item in self.book_list.iter() {
            if let Some((score, indices)) = matcher.fuzzy_indices(item, &search_string) {
                result_list.push((score, indices, item.clone()));
            }
        }
        result_list.sort_by(|(s1score, _, _), (s2score, _, _)| s2score.cmp(s1score));
        self.result_list = result_list;
    }

    fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press
                && key.code == KeyCode::Char('c')
                && key.modifiers == KeyModifiers::CONTROL
            {
                self.should_exit = true;
            } else {
                match key.code {
                    KeyCode::Enter => self.submit_search(),
                    KeyCode::Char(to_insert) => self.enter_char(to_insert),
                    KeyCode::Backspace => self.delete_char(),
                    KeyCode::Left => self.move_cursor_left(),
                    KeyCode::Right => self.move_cursor_right(),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let [title, input_area, result_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .spacing(1)
        .areas(frame.area());
        let search_input = Paragraph::new(self.search_input.as_str())
            .style(Style::default())
            .block(Block::bordered().title("Input"));
        frame.render_widget(search_input, input_area);
        frame.set_cursor_position(Position::new(
            input_area.x + self.search_input_character_index as u16 + 1,
            input_area.y + 1,
        ));
        let book_list_filtered: Vec<ListItem> = self
            .result_list
            .iter()
            .enumerate()
            .map(|(i, m)| {
                // let content = Line::from(Span::raw(format!("{i}: {} {}", m.0, m.2,)));
                let content = highlight_char_in_text(&m.2, &m.1);
                ListItem::new(content)
            })
            .collect();
        let book_list_filtered =
            List::new(book_list_filtered).block(Block::bordered().title("search results"));
        frame.render_widget(book_list_filtered, result_area);
    }
}
