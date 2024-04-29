use std::ops::DerefMut;

use miniquad::*;
use miniquad::window::new_rendering_backend;
use yakui::{widgets::Pad, Color};

use yakui_miniquad::*;

struct Stage {
    ctx: Box<Context>,
    yakui_mq: YakuiMiniQuad,
}

impl Stage {
    pub fn new(mut ctx: Box<Context>) -> Stage {
        let yakui_mq = YakuiMiniQuad::new(ctx.deref_mut());
        Stage {
            ctx,
            yakui_mq
        }
    }
}

impl EventHandler for Stage {
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
        self.ctx.begin_default_pass(Default::default());

        // draw some stuff before the UI?

        self.yakui_mq.draw(self.ctx.deref_mut());

        // ... draw some stuff after the UI!

        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        self.yakui_mq.resize_event(width, height);
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.yakui_mq.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, x: f32, y: f32) {
        self.yakui_mq.mouse_wheel_event(x, y);
    }

    fn mouse_button_down_event(&mut self, button: MouseButton, x: f32, y: f32) {
        self.yakui_mq.mouse_button_down_event(button, x, y);
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, x: f32, y: f32) {
        self.yakui_mq.mouse_button_up_event(button, x, y);
    }

    fn char_event(&mut self, character: char, keymods: KeyMods, repeat: bool) {
        self.yakui_mq.char_event(character, keymods, repeat);
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
}

fn main() {
    miniquad::start(conf::Conf::default(), || {
        Box::new(Stage::new(new_rendering_backend()))
    });
}
