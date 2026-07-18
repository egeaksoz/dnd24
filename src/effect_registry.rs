use crate::{button::AppEvent, gruvbox::Gruvbox};
use ratatui::{layout::Rect, style::Color};
use tachyonfx::{
    Duration, EffectManager, Motion, RefRect,
    fx::{self, dynamic_area, parallel, sequence},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EffectId {
    #[default]
    Button1,
    Button2,
    Button3,
    Button4,
    Button5,
    Button6,
    StatusMessage,
}

pub struct EffectRegistry {
    effects: EffectManager<EffectId>,
    button_areas: [RefRect; 6],
    message_area: RefRect,
    screen_area: RefRect,
}

impl EffectRegistry {
    pub fn new() -> Self {
        Self {
            effects: EffectManager::default(),
            button_areas: [
                RefRect::default(),
                RefRect::default(),
                RefRect::default(),
                RefRect::default(),
                RefRect::default(),
                RefRect::default(),
            ],
            message_area: RefRect::default(),
            screen_area: RefRect::default(),
        }
    }

    pub fn handle_event(&mut self, event: &AppEvent) {
        match event {
            AppEvent::ButtonClicked(button_id) => {
                self.create_click_effect(*button_id as usize);
            }
            AppEvent::ButtonHovered(button_id) => {
                self.create_hover_effect(*button_id as usize);
            }
            AppEvent::ButtonUnhovered(button_id) => {
                self.create_unhover_effect(*button_id as usize);
            }
            AppEvent::ShowMessage() => {
                self.create_message_effect();
            }
            AppEvent::ClearMessage => {
                self.clear_message_effect();
            }
        }
    }

    fn create_click_effect(&mut self, button_id: usize) {
        if button_id >= 6 {
            return;
        }

        let effect_id = match button_id {
            0 => EffectId::Button1,
            1 => EffectId::Button2,
            2 => EffectId::Button3,
            3 => EffectId::Button4,
            4 => EffectId::Button5,
            5 => EffectId::Button6,
            _ => return,
        };

        let button_area = self.button_areas[button_id].clone();

        // Create a dramatic click effect
        let click_effect = sequence(&[
            // Flash bright
            fx::fade_to(
                Gruvbox::YellowBright,
                Gruvbox::YellowBright,
                Duration::from_millis(100),
            ),
            // Pulse effect
            parallel(&[
                fx::fade_to(Gruvbox::Light0, Gruvbox::Light0, Duration::from_millis(150)),
                fx::fade_to(
                    Gruvbox::OrangeBright,
                    Gruvbox::OrangeBright,
                    Duration::from_millis(150),
                ),
            ]),
            // Return to normal with slight glow
            fx::fade_to(Gruvbox::Blue, Gruvbox::Blue, Duration::from_millis(200)),
            // Fade to normal
            fx::fade_to(Color::Reset, Color::Reset, Duration::from_millis(300)),
        ]);

        let effect = dynamic_area(button_area, click_effect);
        self.effects.add_unique_effect(effect_id, effect);
    }

    fn create_hover_effect(&mut self, button_id: usize) {
        if button_id >= 6 {
            return;
        }

        let effect_id = match button_id {
            0 => EffectId::Button1,
            1 => EffectId::Button2,
            2 => EffectId::Button3,
            3 => EffectId::Button4,
            4 => EffectId::Button5,
            5 => EffectId::Button6,
            _ => return,
        };

        let button_area = self.button_areas[button_id].clone();

        // Gentle hover effect
        let paint_effect = fx::paint(Gruvbox::Dark0Soft, Gruvbox::Blue, Duration::from_millis(200));
        let hover_effect = fx::never_complete(paint_effect);
        let effect = dynamic_area(button_area, hover_effect);
        self.effects.add_unique_effect(effect_id, effect);
    }

    fn create_unhover_effect(&mut self, button_id: usize) {
        if button_id >= 6 {
            return;
        }

        let effect_id = match button_id {
            0 => EffectId::Button1,
            1 => EffectId::Button2,
            2 => EffectId::Button3,
            3 => EffectId::Button4,
            4 => EffectId::Button5,
            5 => EffectId::Button6,
            _ => return,
        };

        let button_area = self.button_areas[button_id].clone();

        // Fade back to normal
        let unhover_effect = fx::paint(Color::Reset, Color::Reset, Duration::from_millis(0));
        let effect = dynamic_area(button_area, unhover_effect);
        self.effects.add_unique_effect(effect_id, effect);
    }

    fn create_message_effect(&mut self) {
        let message_area = self.message_area.clone();

        // Complex message effect with entrance, display, and exit
        let message_effect = sequence(&[
            // Slide in from left
            fx::slide_in(
                Motion::LeftToRight,
                10,
                0,
                Gruvbox::Green,
                Duration::from_millis(300),
            ),
            // Gentle pulse while displayed
            fx::ping_pong(fx::fade_to(
                Gruvbox::GreenBright,
                Gruvbox::GreenBright,
                Duration::from_millis(800),
            )),
            // Hold for 2 seconds
            fx::sleep(Duration::from_millis(2000)),
            // Fade out
            fx::fade_to(Color::Reset, Color::Reset, Duration::from_millis(400)),
        ]);

        let effect = dynamic_area(message_area, message_effect);
        self.effects
            .add_unique_effect(EffectId::StatusMessage, effect);
    }

    fn clear_message_effect(&mut self) {
        let message_area = self.message_area.clone();
        let clear_effect = fx::fade_to(Color::Reset, Color::Reset, Duration::from_millis(200));
        let effect = dynamic_area(message_area, clear_effect);
        self.effects
            .add_unique_effect(EffectId::StatusMessage, effect);
    }

    pub fn update_button_area(&mut self, button_id: usize, area: Rect) {
        if button_id < 6 {
            self.button_areas[button_id].set(area);
        }
    }

    pub fn update_message_area(&mut self, area: Rect) {
        self.message_area.set(area);
    }

    pub fn update_screen_area(&mut self, area: Rect) {
        self.screen_area.set(area);
    }

    pub fn process_effects(
        &mut self,
        duration: Duration,
        buf: &mut ratatui::buffer::Buffer,
        area: Rect,
    ) {
        self.effects.process_effects(duration, buf, area);
    }
}
