mod wallpaper_struct;
use wallpaper_struct::Wallpaper;
mod keybind_bar;
mod style;
use crate::keybind_bar::create_keybind_bar;
//use ratatui::Frame;
use ratatui::crossterm::event;
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::Block;
fn main() -> std::io::Result<()>{
	Wallpaper::load();
	ratatui::run(|terminal| {
		loop{
			terminal.draw(|frame| {
				let area = frame.area();
				let [top, main, names] = area.layout(
					&Layout::vertical([
			    		Constraint::Fill(1),
			    		Constraint::Fill(5),
			    		Constraint::Fill(1),
			    		
					]).spacing(-1)
				);
				frame.render_widget(Block::new().title("Top"), top); // TODO display keybinds
				frame.render_widget(Block::new().title("Main"), main); // TODO display wallpapers
				frame.render_widget(create_keybind_bar(), names); // TODO handle in another file			
			});
			// TODO add borders with custom style
			if let Some(key_event) = event::read()?.as_key_press_event() { //should only refresh when something is pressed
			    match key_event.code {
			        event::KeyCode::Char('q') => return Ok(()),
			        // TODO add + for a popup that asks you to add a path to available
			        // TODO maybe make it so that this and the keybind bar get the keys from the same source so that they are the same
			        // maybe like an array of structs with key, name, function
			        // but needs actual keybinds implemented so I need to add + next which needs the popup
			        _ => {}
			    }
			}
		}
	})
}
