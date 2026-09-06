use ratatui::{
    self, DefaultTerminal, Frame, layout::{Alignment, Constraint, Layout}, style::{self, Color::{Black, Gray, Green, White}, Modifier}, widgets::{Block, List, ListItem, ListState, Paragraph},
};

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
    let mut input = String::new();
    let mut choose: bool = false;
    let mut selected: usize = 0;
    loop {
        terminal.draw(|frame| renderer(frame, &mut list, &input, selected, choose))?;
        let event = crossterm::event::read()?;
        if selected == list.len() && selected > 0 { selected -= 1}
        if let crossterm::event::Event::Key(eve_key) = event {
            match eve_key.code {
                crossterm::event::KeyCode::Char(c) => {
                    if choose {
                        input.push(c);
                    }
                    else if c == 'i' {
                        choose = true;
                    }
                    else {
                        if c == 'j' && selected < list.len() {
                            selected += 1;
                        }
                        if c == 'k' && selected > 0 {
                            selected -= 1;
                        }
                        if c == 'd' && list.len() > 0 {
                            list.remove(selected);
                        }
                        if c == 'q' {
                            break;
                        }
                    }
                }
                crossterm::event::KeyCode::Backspace => {
                    input.pop();
                }
                crossterm::event::KeyCode::Enter => {
                    if choose {
                        if !input.is_empty() {
                            list.push(ToDo { todo: input.clone(), done: false });
                        }
                        input.clear();
                        choose = false;
                    }
                    else {
                        if list.len() > 0 {
                            list[selected].done = !list[selected].done;
                        }
                    }
                }
                crossterm::event::KeyCode::Esc => { 
                    choose = false;
                    input.clear();
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn renderer (frame: &mut Frame, list: &mut Vec<ToDo>, input: &str, selected: usize, chosen: bool) {
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
    
    let input_box = Paragraph::new(input)
        .style(style::Style::default()
        .fg(Black)
        .bg(Gray))
        .block(Block::bordered().title("New To-Do"));

    let mut choosable_text = String::new();
    if chosen {
        choosable_text = String::from("INSERT           <ENTER>: add a new to-do        <ESC>: quit INSERT mode     <Q>: quit the app");
    }
    else {
        choosable_text = String::from("NORMAL           <ENTER>: done/undone        <i>: enter INSERT mode      <j - k> : navigate the list     <d> : remove an element          <Q>: quit the app");
    }
    let choosability = Paragraph::new(choosable_text)
        .alignment(Alignment::Left)
        .style(style::Style::default()
            .fg(Black)
            .bg(Green));

    frame.render_widget(header, areas[0]);
    frame.render_stateful_widget(todos, areas[1], &mut state);
    frame.render_widget(input_box, areas[2]);
    frame.render_widget(choosability, areas[3]);

    if chosen {
        frame.set_cursor_position((
            areas[2].x + input.len() as u16 + 1,
            areas[2].y + 1,
        ));
    }

}
