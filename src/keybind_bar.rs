use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::symbols::merge::MergeStrategy;
use ratatui::prelude::{Style, Line, Span};
use ratatui::Frame;
use ratatui::layout::Rect;
use crate::style::BORDER_SET;
pub fn create_keybind_bar(frame: &mut Frame, names: Rect){
	let block = Block::bordered()
		.border_set(BORDER_SET)
		.merge_borders(MergeStrategy::Replace);
	let line = Line::from(vec![
	        Span::styled("q", Style::new().blue()),
	        Span::styled("-", Style::new().white()),
	        Span::styled("quit", Style::new().blue()),
	]);
	let paragraph = Paragraph::new(line).block(block).wrap(Wrap { trim: false });
	frame.render_widget(paragraph, names);
}
// TODO add theming 
