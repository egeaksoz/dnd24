use crate::{effect_registry::EffectRegistry, gruvbox::Gruvbox};
use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    widgets::{Block, Borders, Paragraph, Widget},
};
use tachyonfx::RefRect;

#[derive(Debug, Clone)]
pub enum AppEvent {
    ButtonClicked(u8),
    ButtonHovered(u8),
    ButtonUnhovered(u8),
    ShowMessage(),
    ClearMessage,
}

#[derive(Clone)]
pub struct Button {
    pub id: u8,
    text: String,
    area: RefRect,
    is_hovered: bool,
}

impl Button {
    pub fn new(id: u8, text: String) -> Self {
        Self {
            id,
            text,
            area: RefRect::default(),
            is_hovered: false,
        }
    }

    pub fn handle_click(&mut self, registry: &mut EffectRegistry) {
        registry.handle_event(&AppEvent::ButtonClicked(self.id));
        registry.handle_event(&AppEvent::ShowMessage());
    }

    pub fn handle_hover(&mut self, registry: &mut EffectRegistry, hovered: bool) {
        if hovered && !self.is_hovered {
            registry.handle_event(&AppEvent::ButtonHovered(self.id));
            self.is_hovered = true;
        } else if !hovered && self.is_hovered {
            registry.handle_event(&AppEvent::ButtonUnhovered(self.id));
            self.is_hovered = false;
        }
    }

    pub fn area(&self) -> Rect {
        self.area.get()
    }
}

impl Widget for Button {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        // Update the RefRect - this is crucial for dynamic effects!
        self.area.set(area);

        // Render the button
        let button = Paragraph::new(self.text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Button {}", self.id + 1))
                    .style(Style::default().fg(Gruvbox::Light4.into())),
            )
            .alignment(Alignment::Center)
            .style(Style::default().fg(Gruvbox::Light2.into()));

        button.render(area, buf);
    }
}
