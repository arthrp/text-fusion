use iced::widget::{column, container, row, scrollable, text, text_editor};
use iced::{Background, Color, Element, Length, Task};
use iced::advanced::text::highlighter::Format;
use std::ops::Range;

const STATUS_TEXT_HEIGHT: u16 = 80;

mod highlighter;
use highlighter::LineComparerHighlighter;

#[derive(Debug, Clone)]
pub enum Message {
    LeftTextChanged(text_editor::Action),
    RightTextChanged(text_editor::Action),
}

#[derive(Default)]
struct TextFusion {
    left_text: text_editor::Content,
    right_text: text_editor::Content,
    differences_count: usize,
}


fn update(state: &mut TextFusion, message: Message) -> Task<Message> {
    match message {
        Message::LeftTextChanged(action) => {
            state.left_text.perform(action);
            state.differences_count = count_different_lines(&state.left_text.text(), &state.right_text.text());
        }
        Message::RightTextChanged(action) => {
            state.right_text.perform(action);
            state.differences_count = count_different_lines(&state.left_text.text(), &state.right_text.text());
        }
    }
    Task::none()
}

fn has_first_line_content(content: &text_editor::Content) -> bool {
    let text = content.text();
    if let Some(first_line) = text.lines().next() {
        !first_line.trim().is_empty()
    } else {
        false
    }
}

fn count_different_lines(left_text: &str, right_text: &str) -> usize {
    let left_lines: Vec<&str> = left_text.lines().collect();
    let right_lines: Vec<&str> = right_text.lines().collect();
    let mut count = 0;
    
    let max_lines = left_lines.len().max(right_lines.len());
    for i in 0..max_lines {
        let left_line = left_lines.get(i).unwrap_or(&"");
        let right_line = right_lines.get(i).unwrap_or(&"");
        if left_line != right_line && !left_line.trim().is_empty() {
            count += 1;
        }
    }
    
    count
}

fn view(state: &TextFusion) -> Element<Message> {
    let has_content = has_first_line_content(&state.left_text);
    let has_differences = state.differences_count > 0;

    let left_editor = text_editor(&state.left_text)
    .on_action(Message::LeftTextChanged)
    .placeholder("Enter text here...")
    .height(Length::Shrink)
    .highlight_with::<LineComparerHighlighter>(
        state.right_text.text().to_string(),
        |highlight, _theme| {
            Format {
                color: Some(*highlight),
                ..Default::default()
            }
        }
    )
    .style(move |theme, status| {
        let mut style = text_editor::default(theme, status);
        if has_content {
            if has_differences {
                style.background = Background::Color(Color::from_rgb(1.0, 0.95, 0.95)); // Light pink for differences
            } else {
                style.background = Background::Color(Color::from_rgb(0.9, 1.0, 0.9)); // Light green for match
            }
        }
        style
    });
    
    let left_input = container(
        scrollable(left_editor)
    )
    .padding(10)
    .width(Length::Fill)
    .height(Length::Fill);

    let right_input = container(
        scrollable(
            text_editor(&state.right_text)
                .on_action(Message::RightTextChanged)
                .placeholder("Enter text here...")
                .height(Length::Shrink)
        )
    )
    .padding(10)
    .width(Length::Fill)
    .height(Length::Fill);

    let text_inputs_row = row![left_input, right_input]
        .spacing(20)
        .height(Length::Fill);


    // Status text showing comparison result
    let status_text = if state.differences_count == 0 && has_content {
        text("✓ Texts match perfectly").style(|_theme| text::Style {
            color: Some(Color::from_rgb(0.0, 0.6, 0.0)),
        })
    } else if state.differences_count > 0 {
        text(format!("⚠ {} line(s) differ", state.differences_count)).style(|_theme| text::Style {
            color: Some(Color::from_rgb(0.8, 0.0, 0.0)),
        })
    } else {
        text("Type in both input fields to compare").style(|_theme| text::Style {
            color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
        })
    };

    let content = column![
        text_inputs_row,
        container(status_text)
            .center_x(Length::Fill)
            .padding(5)
            .height(STATUS_TEXT_HEIGHT)
    ]
    .spacing(10)
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

fn main() -> iced::Result {
    iced::application("Text Fusion - Compare Tool", update, view)
        .window_size((1000.0, 600.0))
        .run()
}

