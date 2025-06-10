use iced::advanced::text::Highlighter;
use iced::Color;
use std::ops::Range;

// Custom highlighter for comparing lines between left and right text editors
#[derive(Debug, Clone)]
pub struct LineComparerHighlighter {
    pub right_text: String,
    pub current_line: usize,
}

impl Highlighter for LineComparerHighlighter {
    type Settings = String;
    type Highlight = Color;
    type Iterator<'a> = std::vec::IntoIter<(Range<usize>, Self::Highlight)>;

    fn new(settings: &Self::Settings) -> Self {
        Self {
            right_text: settings.clone(),
            current_line: 0,
        }
    }

    fn update(&mut self, new_settings: &Self::Settings) {
        self.right_text = new_settings.clone();
    }

    fn change_line(&mut self, line: usize) {
        self.current_line = line;
    }

    fn highlight_line(&mut self, line: &str) -> Self::Iterator<'_> {
        //Seems like ok to use itrator here instead of creating a vector
        let mut right_lines: std::str::Lines<'_> = self.right_text.lines();
        let is_different = if let Some(right_line) = right_lines.nth(self.current_line) {
            line != right_line
        } else {
            !line.trim().is_empty()
        };

        let mut highlights = Vec::new();
        if is_different && !line.trim().is_empty() {
            highlights.push((0..line.len(), Color::from_rgb(1.0, 0.0, 0.0)));
        }

        self.current_line += 1;
        highlights.into_iter()
    }

    fn current_line(&self) -> usize {
        self.current_line
    }
}
