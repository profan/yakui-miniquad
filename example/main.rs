use miniquad::*;
use yakui_miniquad::*;
use yakui::{Color, widgets::Pad};

struct Stage {
    yakui_mq: YakuiMiniQuad
}

impl Stage {
    pub fn new(ctx: &mut GraphicsContext) -> Stage {
        let yakui_mq = YakuiMiniQuad::new(ctx);
        Stage {
            yakui_mq
        }
    }
}

impl EventHandler for Stage {

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        self.yakui_mq.mouse_motion_event(ctx, x, y);
    }

    fn mouse_button_down_event(
            &mut self,
            ctx: &mut Context,
            button: MouseButton,
            x: f32,
            y: f32,
        ) {
        self.yakui_mq.mouse_button_down_event(ctx, button, x, y);
    }

    fn mouse_button_up_event(
            &mut self,
            ctx: &mut Context,
            button: MouseButton,
            x: f32,
            y: f32,
        ) {
        self.yakui_mq.mouse_button_up_event(ctx, button, x, y);
    }

    fn key_down_event(
            &mut self,
            ctx: &mut Context,
            keycode: KeyCode,
            keymods: KeyMods,
            repeat: bool,
        ) {
        self.yakui_mq.key_down_event(ctx, keycode, keymods, repeat);
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        self.yakui_mq.key_up_event(ctx, keycode, keymods);
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        self.yakui_mq.mouse_wheel_event(ctx, x, y);
    }

    fn char_event(
            &mut self,
            ctx: &mut Context,
            character: char,
            keymods: KeyMods,
            repeat: bool,
        ) {
        self.yakui_mq.char_event(ctx, character, keymods, repeat);
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        self.yakui_mq.resize_event(ctx, width, height);
    }

    fn update(&mut self, ctx: &mut Context) {

        self.yakui_mq.run(ctx, |_| {

            yakui::center(|| {
                yakui::colored_box_container(Color::CORNFLOWER_BLUE, || {
                    yakui::pad(Pad::all(16.0), || {
                        yakui::text(32.0, "hello, world!");    
                    });
                });
            });

        });

    }

    fn draw(&mut self, ctx: &mut Context) {

        ctx.begin_default_pass(Default::default());

        // draw some stuff before the UI?

        self.yakui_mq.draw(ctx);

        // ... draw some stuff after the UI!

        ctx.end_render_pass();

        ctx.commit_frame();

    }

}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        Box::new(Stage::new(&mut ctx))
    });
}