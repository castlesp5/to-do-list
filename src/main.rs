
use ratatui::{self, DefaultTerminal, Frame};


#[derive(Debug)]
pub struct ToDo {
    todo: String,
    done: bool
}


fn main() -> std::io::Result<()> {
    ratatui::run(app)?;
    Ok(())
}


fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut list: Vec<ToDo> = vec![];
    loop {
        terminal.draw(|frame| renderer(frame, &mut list))?;
        let event = crossterm::event::read()?;
        if let crossterm::event::Event::Key(eve_key) = event {
            match eve_key.code {
                crossterm::event::KeyCode::Char(' ') => {
                    list.push(ToDo { todo: String::from("new do"), done: false });
                }
                crossterm::event::KeyCode::Backspace => {
                    list.pop();
                }
                crossterm::event::KeyCode::Char('q') => { break; }
                _ => {}
            }
        }
    }

    Ok(())
}

fn renderer (frame: &mut Frame, list: &mut Vec<ToDo>) {
    frame.render_widget(format!("{:#?}", list), frame.area());
}
