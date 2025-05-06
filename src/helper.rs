use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
};

pub fn highlight_char_in_text<'a>(text: &'a str, indices: &'a [usize]) -> Line<'a> {
    let mut span_vec = Vec::new();
    let mut last_string = Vec::new();
    for (idx, itm) in text.chars().enumerate() {
        if indices.contains(&idx) {
            if !last_string.is_empty() {
                span_vec.push(Span::from(last_string.into_iter().collect::<String>()));
            }
            last_string = Vec::new();
            let highlight_span = Span::styled(
                format!("{}", itm.clone()),
                Style::default().fg(Color::Green),
            );
            span_vec.push(highlight_span);
        } else {
            last_string.push(itm.clone());
        }
    }
    if !last_string.is_empty() {
        span_vec.push(Span::from(last_string.into_iter().collect::<String>()));
    }
    span_vec.into()
}
