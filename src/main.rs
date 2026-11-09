mod wallpaper_struct;
use wallpaper_struct::Wallpaper;
mod keybind_bar;
mod style;
mod popup;
use crate::keybind_bar::create_keybind_bar;
use crate::popup::gen_popup;
//use ratatui::Frame;
use ratatui::crossterm::event;
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::Block;
fn main() -> std::io::Result<()>{
	Wallpaper::load();
	let mut show_popup: bool = false;
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
				create_keybind_bar(frame, names);
				
				if show_popup {
					 gen_popup(frame, "Test".to_string(), "Hi".to_string());
					 show_popup=false;
				}
			});
			if let Some(key_event) = event::read()?.as_key_press_event() { //should only refresh when something is pressed
			    match key_event.code {
			        event::KeyCode::Char('p') => show_popup=true,
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
// TODO also add an option for making themes and a better installer app
