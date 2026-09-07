use std::fs;

use ratatui::{
    self, DefaultTerminal, Frame, layout::{Alignment, Constraint, Layout}, style::{self, Color::{Black, Gray, Green, White}, Modifier}, widgets::{Block, List, ListItem, ListState, Paragraph},
};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
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
    let mut input = String::new();
    let mut choose: i32 = 0;
    let mut selected: usize = 0;
    loop {
        terminal.draw(|frame| renderer(frame, &mut list, &input, selected, choose))?;
        let event = crossterm::event::read()?;
        if selected == list.len() && selected > 0 { selected -= 1}
        if let crossterm::event::Event::Key(eve_key) = event {
            match eve_key.code {
                crossterm::event::KeyCode::Char(c) => {
                    if choose > 0 {
                        input.push(c);
                    }
                    else if c == 'a' {
                        choose = 1;
                    }
                    else if c == 'e' && list.len() > 0 {
                        choose = 2;
                        input = list[selected].todo.clone();
                    }
                    else {
                        if c == 'j' && selected < list.len() {
                            selected += 1;
                        }
                        else if c == 'k' && selected > 0 {
                            selected -= 1;
                        }
                        else if c == 'K' {
                            if choose == 0 && selected > 0 {
                                let text = list.remove(selected);
                                list.insert(selected - 1, text);
                                selected -= 1
                            }
                        }
                        else if c == 'J' {
                            if choose == 0 && selected + 1 < list.len(){
                                let text = list.remove(selected);
                                list.insert(selected + 1, text);
                                selected += 1
                            }
                        }
                        else if c == 'd' && list.len() > 0 {
                            list.remove(selected);
                        }
                        else if c == 'q' {
                            break;
                        }
                        else if c == 's' {
                            choose = 3;
                        }
                        else if c == 'o' {
                            choose = 4;
                        }
                    }
                }
                crossterm::event::KeyCode::Backspace => {
                    input.pop();
                }
                crossterm::event::KeyCode::Enter => {
                    if choose > 0 {
                        if !input.is_empty() {
                            match choose {
                               1 => {list.push(ToDo { todo: input.clone(), done: false });}
                               2 => {list[selected].todo = input.clone();}
                               3 => {
                                    if !input.ends_with(".json") {
                                        choose = 5;
                                        continue;
                                    }
                                    let js = serde_json::to_string_pretty(&list)?;
                                    std::fs::write(&input, &js)?;
                                    choose = 8;
                                    continue;
                                    }
                               4 => {
                                    if !input.ends_with(".json") {
                                        choose = 5;
                                        continue;
                                    }
                                    match fs::read_to_string(&input) {
                                        Ok(content) => {
                                           match serde_json::from_str(&content) {
                                                Ok(js2) => { list = js2 }
                                                Err(_) => {choose = 6; continue;}
                                           }
                                        }
                                        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                                            choose = 7;
                                            continue;
                                        }
                                        Err(_) => {choose = 5; continue;}
                                    }
                               }
                               _ => {}
                            }
                            input.clear();
                            choose = 0;
                        }
                    }
                    else {
                        if list.len() > 0 {
                            list[selected].done = !list[selected].done;
                        }
                    }
                }
                crossterm::event::KeyCode::Esc => { 
                    choose = 0;
                    input.clear();
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn renderer (frame: &mut Frame, list: &mut Vec<ToDo>, input: &str, selected: usize, chosen: i32) {
    let items: Vec<ListItem> = list
        .iter()
        .map(|todo| {
            let symbol = if todo.done { "✓" } else { "○" };

            ListItem::new(format!("{} {}", symbol, todo.todo))
        })
        .collect();


    let areas = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(5),
            Constraint::Length(1),
    ]).split(frame.area());

    let header = Paragraph::new("To-Do App V1.0")
        .alignment(Alignment::Center)
        .style(style::Style::default()
        .fg(Black)
        .bg(White));

    let todos = List::new(items)
        .block(Block::bordered().title("List"))
        .highlight_symbol("")
        .highlight_style(
            style::Style::default()
                .bg(White)
                .fg(Black)
                .add_modifier(Modifier::BOLD)
        );

    let mut state = ListState::default();
    state.select(Some(selected));
    
    if chosen > 0 && chosen < 3 {
        let input_box = Paragraph::new(input)
            .style(style::Style::default()
            .fg(Black)
            .bg(Gray))
            .block(Block::bordered().title("INSERT").title_bottom(ratatui::text::Line::from("www.github.com/castlesp5").alignment(Alignment::Right)));
        frame.render_widget(input_box, areas[2]);
    }

    let mut choosable_text = String::new();
    if chosen > 0 {
        match chosen {
            1 => {choosable_text = String::from("INSERT           <ENTER>: add a new to-do        <ESC>: quit INSERT mode");}
            2 => {choosable_text = String::from("EDIT             <ENTER>: submit modifications   <ESC>: quit EDIT mode");}
            3 => {choosable_text = String::from(format!("Path to write into : {}", input));}
            4 => {choosable_text = String::from(format!("Path to JSON file : {}", input));}
            5 => {choosable_text = String::from(format!("Wrong path format, file should be '.json'. Press <ESC> to return"));}
            6 => {choosable_text = String::from(format!("ERROR: Malformed JSON file. Press <ESC> to return"));}
            7 => {choosable_text = String::from(format!("File Not Found. Press <ESC> to return"));}
            8 => {choosable_text = String::from(format!("File was saved into '{}'. Press <ESC> to return", input));}
            _ => {}
        }
    }
    else {
        choosable_text = String::from("NORMAL           <ENTER>: done/undone        <a - e>: add/edit an item      <j - k> : navigate the list     <d> : remove an element          <Q>: quit the app");
    }
    let choosability = Paragraph::new(choosable_text)
        .alignment(Alignment::Left)
        .style(style::Style::default()
            .fg(Black)
            .bg(Green));

    frame.render_widget(header, areas[0]);
    frame.render_stateful_widget(todos, areas[1], &mut state);
    frame.render_widget(choosability, areas[3]);

    if chosen > 0 {
        match chosen {
            1 => {frame.set_cursor_position((
                areas[2].x + input.len() as u16 + 1,
                areas[2].y + 1,
            ));}
            2 => {frame.set_cursor_position((
                areas[2].x + input.len() as u16 + 1,
                areas[2].y + 1,
            ));}
            3 => {frame.set_cursor_position((
                    areas[3].x + input.len() as u16 + 21,
                    areas[3].y + 1,
            ));}
            _ => {}
        }
    }

}
