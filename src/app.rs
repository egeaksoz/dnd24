use crate::{
    button::{AppEvent, Button},
    effect_registry::EffectRegistry,
    gruvbox::Gruvbox,
};
use crossterm::event::{Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind};
use rand::RngExt;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

pub struct App {
    pub registry: EffectRegistry,
    buttons: [Button; 6],
    pub is_running: bool,
    message: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            registry: EffectRegistry::new(),
            buttons: [
                Button::new(0, "D4".to_string()),
                Button::new(1, "D6".to_string()),
                Button::new(2, "D8".to_string()),
                Button::new(3, "D10".to_string()),
                Button::new(4, "D12".to_string()),
                Button::new(5, "D20".to_string()),
            ],
            is_running: true,
            message: "Roll a dice".to_string(),
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            self.is_running = false;
                        }
                        KeyCode::Char('1') => {
                            self.buttons[0].handle_click(&mut self.registry);
                            let dice = App::roll_dice(0);
                            self.message = format!("Rolled: {}", dice);
                        }
                        KeyCode::Char('2') => {
                            self.buttons[1].handle_click(&mut self.registry);
                            let dice = App::roll_dice(1);
                            self.message = format!("Rolled: {}", dice);
                        }
                        KeyCode::Char('3') => {
                            self.buttons[2].handle_click(&mut self.registry);
                            let dice = App::roll_dice(2);
                            self.message = format!("Rolled: {}", dice);
                        }
                        KeyCode::Char('4') => {
                            self.buttons[3].handle_click(&mut self.registry);
                            let dice = App::roll_dice(3);
                            self.message = format!("Rolled: {}", dice);
                        }
                        KeyCode::Char('5') => {
                            self.buttons[4].handle_click(&mut self.registry);
                            let dice = App::roll_dice(4);
                            self.message = format!("Rolled: {}", dice);
                        }
                        KeyCode::Char('6') => {
                            self.buttons[5].handle_click(&mut self.registry);
                            let dice = App::roll_dice(5);
                            self.message = format!("Rolled: {}", dice);
                        }
                        KeyCode::Char('c') => {
                            self.registry.handle_event(&AppEvent::ClearMessage);
                            self.message = String::from("Roll a dice");
                        }
                        _ => {}
                    }
                }
            }
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    let pos = (mouse.column, mouse.row);

                    for button in &mut self.buttons {
                        if button.area().contains(pos.into()) {
                            button.handle_click(&mut self.registry);
                            let dice = App::roll_dice(button.id as usize);
                            self.message = format!("Rolled: {}", dice);
                        }
                    }
                }
                MouseEventKind::Down(MouseButton::Right) => {
                    self.registry.handle_event(&AppEvent::ClearMessage);
                    self.message = String::from("Roll a dice");
                }
                MouseEventKind::Moved => {
                    let pos = (mouse.column, mouse.row);
                    for button in &mut self.buttons {
                        let hovered = button.area().contains(pos.into());
                        button.handle_hover(&mut self.registry, hovered);
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn roll_dice(index: usize) -> u8 {
        let mut rnd = rand::rng();
        match index {
            0 => rnd.random_range(1..5),
            1 => rnd.random_range(1..7),
            2 => rnd.random_range(1..9),
            3 => rnd.random_range(1..11),
            4 => rnd.random_range(1..13),
            5 => rnd.random_range(1..21),
            _ => unreachable!(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        self.registry.update_screen_area(area);

        // Clear the screen
        frame.render_widget(Clear, area);

        // Main layout
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Length(5), // Buttons
                Constraint::Length(3), // Message area
                Constraint::Min(0),    // Help
            ])
            .split(area);

        // Title
        let title = Paragraph::new("Dnd Dice Roller")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .style(
                Style::default()
                    .fg(Gruvbox::Yellow.into())
                    .add_modifier(Modifier::BOLD),
            );
        frame.render_widget(title, main_layout[0]);

        // Buttons layout
        let button_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(16),
                Constraint::Percentage(16),
                Constraint::Percentage(16),
                Constraint::Percentage(16),
                Constraint::Percentage(16),
                Constraint::Percentage(17),
            ])
            .split(main_layout[1]);

        // Render buttons and update their areas in the registry
        for (i, button_area) in button_layout.iter().enumerate() {
            self.registry.update_button_area(i, *button_area);
            frame.render_widget(self.buttons[i].clone(), *button_area);
        }

        // Message area
        self.registry.update_message_area(main_layout[2]);
        let message_widget = Paragraph::new(self.message.clone())
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .style(Style::default().fg(Gruvbox::Light3.into()));
        frame.render_widget(message_widget, main_layout[2]);

        // Help text
        let help_text = vec![
            Line::from(vec![
                Span::styled("Controls: ", Style::default().fg(Gruvbox::Blue.into())),
                Span::raw("1/2/3/4/5/6 - Roll dice, "),
                Span::raw("c - Clear message, "),
                Span::raw("q/Esc - Quit"),
            ]),
            Line::from(vec![
                Span::styled("Mouse: ", Style::default().fg(Gruvbox::Green.into())),
                Span::raw("Left-click - Roll dice, "),
                Span::raw("Right-click - Clear"),
            ]),
        ];

        let help = Paragraph::new(help_text)
            .block(Block::default().borders(Borders::ALL).title("Help"))
            .style(Style::default().fg(Gruvbox::Light4.into()));
        frame.render_widget(help, main_layout[3]);
    }
}
