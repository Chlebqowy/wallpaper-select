use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::symbols::merge::MergeStrategy;
use ratatui::prelude::{Style, Line, Span};
use crate::style::BORDER_SET;
pub fn create_keybind_bar() -> ratatui::widgets::Paragraph<'static>{
	let block = Block::bordered()
		.border_set(BORDER_SET)
		.merge_borders(MergeStrategy::Replace);
	let line = Line::from(vec![
	        Span::styled("q", Style::new().blue()),
	        Span::styled("-", Style::new().white()),
	        Span::styled("quit", Style::new().blue()),
	]);
	Paragraph::new(line).block(block).wrap(Wrap { trim: false })
}
