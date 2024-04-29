use macroquad::prelude::*;
use yakui::use_state;
use yakui::widgets::Pad;
use yakui_core::MainAxisSize;

#[macroquad::main("Macroquad with Yakui")]
async fn main() {
    let mut stage = {
        let InternalGlContext {
            quad_context: ctx, ..
        } = unsafe { get_internal_gl() };

        raw_miniquad::Stage::new(ctx)
    };

    loop {
        clear_background(LIGHTGRAY);

        // Render some primitives in camera space

        set_camera(&Camera2D {
            zoom: vec2(1., screen_width() / screen_height()),
            ..Default::default()
        });
        draw_line(-0.4, 0.4, -0.8, 0.9, 0.05, BLUE);
        draw_rectangle(-0.3, 0.3, 0.2, 0.2, GREEN);
        draw_circle(0., 0., 0.1, YELLOW);

        {
            let mut gl = unsafe { get_internal_gl() };

            // Ensure that macroquad's shapes are not going to be lost
            gl.flush();

            gl.quad_context
                .begin_default_pass(miniquad::PassAction::Nothing);

            // Process input events
            stage.consume_events(gl.quad_context);

            stage.yakui_mq.run(gl.quad_context, |_| {
                yakui::center(|| {
                    yakui::colored_box_container(yakui_core::geometry::Color::CORNFLOWER_BLUE, || {
                        yakui::pad(Pad::all(16.0), || {
                            let mut list= yakui::widgets::List::column();
                            list.main_axis_size = MainAxisSize::Min;
                            list.show(|| {
                                let text_size = use_state(||32.0);
                                text_size.set(yakui::slider(text_size.get(), 16.0, 64.0).value.unwrap_or(text_size.get()));
                                yakui::text(text_size.get() as f32, "hello, world!");
                                if yakui::button("Button").clicked {
                                    println!("Button clicked");
                                }
                            });
                        });
                    });
                });
            });

            stage.yakui_mq.draw(gl.quad_context);

            gl.quad_context.end_render_pass();
        }

        set_default_camera();
        draw_text("HELLO", 30.0, 200.0, 30.0, BLACK);

        next_frame().await
    }
}

mod raw_miniquad {
    use macroquad::input::utils::{register_input_subscriber, repeat_all_miniquad_input};
    use miniquad::Context;

    use yakui_miniquad::YakuiMiniQuad;

    pub struct Stage {
        subscriber_id: usize,
        pub yakui_mq: YakuiMiniQuad,
    }

    impl Stage {
        pub fn new(ctx: &mut Context) -> Stage {
            let subscriber_id = register_input_subscriber();
            let yakui_mq = YakuiMiniQuad::new(ctx);
            Stage { yakui_mq, subscriber_id }
        }

        pub fn consume_events(&mut self, ctx: &mut Context) {
            let mut handler = self.yakui_mq.as_event_handler(ctx);
            repeat_all_miniquad_input(&mut handler, self.subscriber_id)
        }
    }
}