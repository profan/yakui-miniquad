use std::ops::DerefMut;

use miniquad::*;
use miniquad::window::new_rendering_backend;
use yakui::{widgets::Pad, Color};

use yakui_miniquad::*;
use yakui_miniquad::event_handlers::YakuiMiniQuadOwnedHandler;

struct Stage {
    yakui_mq: YakuiMiniQuadOwnedHandler,
}

impl Stage {
    pub fn new(mut ctx: Box<Context>) -> Stage {
        let yakui_mq = YakuiMiniQuad::new(ctx.deref_mut()).into_owned_event_handler(ctx);
        Stage { yakui_mq }
    }
}

impl EventHandler for Stage {
    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.yakui_mq.mouse_motion_event(x, y);
    }

    fn mouse_button_down_event(&mut self, button: MouseButton, x: f32, y: f32) {
        self.yakui_mq.mouse_button_down_event(button, x, y);
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, x: f32, y: f32) {
        self.yakui_mq.mouse_button_up_event(button, x, y);
    }

    fn key_down_event(
        &mut self,
        keycode: KeyCode,
        keymods: KeyMods,
        repeat: bool,
    ) {
        self.yakui_mq.key_down_event(keycode, keymods, repeat);
    }

    fn key_up_event(&mut self, keycode: KeyCode, keymods: KeyMods) {
        self.yakui_mq.key_up_event(keycode, keymods);
    }

    fn mouse_wheel_event(&mut self, x: f32, y: f32) {
        self.yakui_mq.mouse_wheel_event(x, y);
    }

    fn char_event(&mut self, character: char, keymods: KeyMods, repeat: bool) {
        self.yakui_mq.char_event(character, keymods, repeat);
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        self.yakui_mq.resize_event(width, height);
    }

    fn update(&mut self) {
        self.yakui_mq.start();

        yakui::center(|| {
            yakui::colored_box_container(Color::CORNFLOWER_BLUE, || {
                yakui::pad(Pad::all(16.0), || {
                    yakui::text(32.0, "hello, world!");
                });
            });
        });

        self.yakui_mq.finish();
    }

    fn draw(&mut self) {
        self.yakui_mq.miniquad_ctx().begin_default_pass(Default::default());

        // draw some stuff before the UI?

        self.yakui_mq.draw();

        // ... draw some stuff after the UI!

        let ctx = self.yakui_mq.miniquad_ctx();

        ctx.end_render_pass();

        ctx.commit_frame();
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), || {
        Box::new(Stage::new(new_rendering_backend()))
    });
}
