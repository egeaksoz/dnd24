use color_eyre::eyre::{Ok, Result};
use crossterm::event::{
    self,
    KeyCode::{self},
};
use rand::RngExt;
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier},
    text::{Line, Text},
    widgets::{Block, List, ListState, Paragraph},
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut list_state = ListState::default().with_selected(Some(0));
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| render(frame, &mut list_state))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Down => list_state.select_next(),
                    KeyCode::Up => list_state.select_previous(),
                    KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                    _ => {}
                }
            }
        }
    })
}

fn render(frame: &mut Frame, list_state: &mut ListState) {
    // Widgets and Layouts
    let outer_block = Block::bordered().title(" DnD 2024 ");
    let roll_layout = Layout::vertical([Constraint::Percentage(80), Constraint::Percentage(20)]);
    let [dice_choices, result] = outer_block.inner(frame.area()).layout(&roll_layout);
    let dice_options = ["4", "6", "8", "10", "20"];
    let dice_list = List::new(dice_options)
        .style(Color::Red)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");
    let result_text = Text::from(vec![Line::from(vec![
        "You've rolled: ".into(),
        roll_dice(list_state).to_string().into(),
    ])]);
    let result_paragraph = Paragraph::new(result_text);

    // Rendering
    frame.render_stateful_widget(dice_list, dice_choices, list_state);
    frame.render_widget(result_paragraph, result);
    frame.render_widget(outer_block, frame.area());
}

fn roll_dice(list_state: &mut ListState) -> u8 {
    let index: usize = list_state.selected().unwrap();
    let mut rnd = rand::rng();
    match index {
        0 => rnd.random_range(1..5),
        1 => rnd.random_range(1..7),
        2 => rnd.random_range(1..9),
        3 => rnd.random_range(1..11),
        4 => rnd.random_range(1..21),
        _ => unreachable!(),
    }
}
