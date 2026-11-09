mod wallpaper_struct;
use wallpaper_struct::Wallpaper;
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
					])
				);
				frame.render_widget(Block::new().title("Top"), top); // TODO display keybinds
				frame.render_widget(Block::new().title("Main"), main); // TODO display wallpapers
				frame.render_widget(Block::new().title("Names"), names); // TODO handle in another file			
			});
			// TODO add borders with custom style
			if let Some(key_event) = event::read()?.as_key_press_event() { //should only refresh when something is pressed
			    match key_event.code {
			        event::KeyCode::Char('q') => return Ok(()),
			        // TODO add + for a popup that asks you to add a path to available
			        _ => {}
			    }
			}
		}
	})
}
