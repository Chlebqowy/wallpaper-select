use crate::style::BORDER_SET_POPUP;
use ratatui::widgets::{Block, Clear, Paragraph};
use ratatui::layout::Constraint;
use ratatui::prelude::Alignment;
use ratatui::Frame;
pub fn gen_popup(frame: &mut Frame, title: String, text: String)
{
	let area = frame.area();
	let popup_area = area.centered(Constraint::Percentage(80), Constraint::Length(3));
	let popup_block = Block::bordered().title(title).border_set(BORDER_SET_POPUP); // TODO invert fg and bg or also make this render manually
	let paragraph = Paragraph::new(text).block(popup_block).alignment(Alignment::Center);
	frame.render_widget(Clear, popup_area);
	frame.render_widget(paragraph, popup_area);
}
pub fn err_popup(frame: &mut Frame, text: String)
{
	gen_popup(frame, "Error".to_string(), text); //TODO also apply style
}
// TODO add popups with input
