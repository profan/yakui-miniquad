//!
//! `yakui-miniquad` integrates yakui with miniquad.
//!
//! # Usage
//! In order to use this library, create an instance of [`YakuiMiniQuad`] and call its event-handler functions from your `miniquad::EventHandler` implementation.
//! 
//! Here's an example which just renders "hello, world" in the middle of the screen.
//! 
//! ```no_run
//! use miniquad::*;
//! use yakui_miniquad::*;
//! use yakui::{Color, widgets::Pad};
//!
//! struct Stage {
//!     yakui_mq: YakuiMiniQuad
//! }
//!
//! impl Stage {
//!    pub fn new(ctx: &mut GraphicsContext) -> Stage {
//!         let yakui_mq = YakuiMiniQuad::new(ctx);
//!         Stage {
//!             yakui_mq
//!         }
//!     }
//! }
//!
//! impl EventHandler for Stage {
//!
//!     fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
//!         self.yakui_mq.mouse_motion_event(ctx, x, y);
//!     }
//!
//!     fn mouse_button_down_event(
//!             &mut self,
//!             ctx: &mut Context,
//!             button: MouseButton,
//!             x: f32,
//!             y: f32,
//!         ) {
//!         self.yakui_mq.mouse_button_down_event(ctx, button, x, y);
//!     }
//!
//!     fn mouse_button_up_event(
//!             &mut self,
//!             ctx: &mut Context,
//!             button: MouseButton,
//!             x: f32,
//!             y: f32,
//!         ) {
//!         self.yakui_mq.mouse_button_up_event(ctx, button, x, y);
//!     }
//! 
//!     fn key_down_event(
//!             &mut self,
//!             ctx: &mut Context,
//!             keycode: KeyCode,
//!             keymods: KeyMods,
//!             repeat: bool,
//!         ) {
//!         self.yakui_mq.key_down_event(ctx, keycode, keymods, repeat);
//!     }
//! 
//!     fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
//!         self.yakui_mq.key_up_event(ctx, keycode, keymods);
//!     }
//! 
//!     fn mouse_wheel_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
//!         self.yakui_mq.mouse_wheel_event(ctx, x, y);
//!     }
//! 
//!     fn char_event(
//!             &mut self,
//!             ctx: &mut Context,
//!             character: char,
//!             keymods: KeyMods,
//!             repeat: bool,
//!         ) {
//!         self.yakui_mq.char_event(ctx, character, keymods, repeat);
//!     }
//! 
//!     fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
//!         self.yakui_mq.resize_event(ctx, width, height);
//!     }
//! 
//!     fn update(&mut self, ctx: &mut Context) {
//!
//!         self.yakui_mq.start(ctx);
//!
//!         yakui::center(|| {
//!             yakui::colored_box_container(Color::CORNFLOWER_BLUE, || {
//!                 yakui::pad(Pad::all(16.0), || {
//!                     yakui::text(32.0, "hello, world!");
//!                 });
//!             });
//!         });
//!
//!         self.yakui_mq.finish();
//!
//!     }
//!
//!     fn draw(&mut self, ctx: &mut Context) {
//!
//!         ctx.begin_default_pass(Default::default());
//!
//!         // draw some stuff before the UI?
//! 
//!         self.yakui_mq.draw(ctx);
//!
//!         // ... draw some stuff after the UI!
//!
//!         ctx.end_render_pass();
//!
//!         ctx.commit_frame();
//!
//!     }
//!
//! }
//! 
//! fn main() {
//!     miniquad::start(conf::Conf::default(), |mut ctx| {
//!         Box::new(Stage::new(&mut ctx))
//!     });
//! }
//!```

use std::{ops::Range, collections::HashMap};

use yakui::paint;
use yakui::{Yakui, Rect, event::Event, paint::PaintDom};
use yakui::input::MouseButton as YakuiMouseButton;
use yakui::input::KeyCode as YakuiKeyCode;

use miniquad::*;

pub use miniquad;
pub use yakui;

#[repr(C)]
struct YakuiVertex {
    pos: yakui::Vec2,
    texcoord: yakui::Vec2,
    color: yakui::Vec4,
}

pub struct YakuiMiniQuad {
    ui: Yakui,
    state: YakuiMiniquadState
}

impl YakuiMiniQuad {

    pub fn new(ctx: &mut GraphicsContext) -> YakuiMiniQuad {
        YakuiMiniQuad {
            ui: Yakui::new(),
            state: YakuiMiniquadState::new(ctx)
        }
    }

    pub fn ctx(&mut self) -> &mut Yakui {
        &mut self.ui
    }

    /// Updates the viewport size and calls start on the internal yakui context, binding it to the current thread.
    pub fn start(&mut self, ctx: &mut Context)
    {
        self.update(ctx);
        self.ui.start();
    }

    /// Calls finish on the internal yakui context, preparing the context for rendering.
    pub fn finish(&mut self) {
        self.ui.finish();
    }

    /// Wraps calling [`start`] and [`finish`], where [`start`] will now be called before your closure is invoked and [`finish`] will be invoked after.
    pub fn run<F>(&mut self, ctx: &mut Context, ui_update_function: F)
        where F: FnOnce(&mut Yakui) -> ()
    {
        self.update(ctx);

        self.ui.start();
        ui_update_function(&mut self.ui);
        self.ui.finish();
    }

    /// Renders the queued ui draw commands.
    pub fn draw(&mut self, ctx: &mut GraphicsContext) {
        self.state.paint(&mut self.ui, ctx);
    }

}

pub struct YakuiMiniquadState {

    main_pipeline: Pipeline,
    text_pipeline: Pipeline,
    textures: HashMap<yakui::ManagedTextureId, Texture>,
    layout: Bindings,

    default_texture: Texture,
    vertices: Buffer,
    indices: Buffer,
    commands: Vec<DrawCommand>

}

struct DrawCommand {
    index_range: Range<u32>,
    texture: Texture,
    pipeline: yakui::paint::Pipeline,
    clip: Option<Rect>,
}

impl EventHandler for YakuiMiniQuad {

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        let mouse_position = yakui::Vec2::new(x, y);
        self.ui.handle_event(Event::CursorMoved(Some(mouse_position)));
    }

    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut Context,
            button: MouseButton,
            _x: f32,
            _y: f32,
        ) {
        if let Some(mouse_button) = miniquad_mouse_button_to_yakui(button) {
            self.ui.handle_event(Event::MouseButtonChanged { button: mouse_button, down: true });
        }
    }

    fn mouse_button_up_event(
            &mut self,
            _ctx: &mut Context,
            button: MouseButton,
            _x: f32,
            _y: f32,
        ) {
        if let Some(mouse_button) = miniquad_mouse_button_to_yakui(button) {
            self.ui.handle_event(Event::MouseButtonChanged { button: mouse_button, down: false });
        }
    }

    fn key_down_event(
            &mut self,
            _ctx: &mut Context,
            keycode: KeyCode,
            _keymods: KeyMods,
            _repeat: bool,
        ) {
        if let Some(key_code) = miniquad_key_to_yakui(keycode) {
            self.ui.handle_event(Event::KeyChanged { key: key_code, down: true });
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        if let Some(key_code) = miniquad_key_to_yakui(keycode) {
            self.ui.handle_event(Event::KeyChanged { key: key_code, down: false });
        }
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        self.ui.handle_event(Event::MouseScroll { delta: yakui::Vec2 { x, y } });
    }

    fn char_event(
            &mut self,
            _ctx: &mut Context,
            character: char,
            _keymods: KeyMods,
            _repeat: bool,
        ) {
        self.ui.handle_event(Event::TextInput(character));
    }

    fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32) {
        let viewport_position = yakui::Vec2 { x: 0.0, y: 0.0 };
        let viewport_size = yakui::Vec2 { x: width, y: height };
        self.ui.handle_event(Event::ViewportChanged(Rect::from_pos_size(viewport_position, viewport_size)));
    }

    fn update(&mut self, ctx: &mut Context) {

        let (screen_w, screen_h) = ctx.screen_size();

        self.ui.set_scale_factor(ctx.dpi_scale());
        self.ui.set_surface_size(yakui::Vec2 { x: screen_w, y: screen_h });
        self.ui.set_unscaled_viewport(yakui::Rect::from_pos_size(
            Default::default(),
            [screen_w, screen_h].into(),
        ));

    }

    fn draw(&mut self, ctx: &mut Context) {
        self.state.paint(&mut self.ui, ctx);
    }

}

impl YakuiMiniquadState {

    pub fn new(ctx: &mut GraphicsContext) -> Self {

        let main_pipeline = make_main_pipeline(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("in_texcoord", VertexFormat::Float2),
                VertexAttribute::new("in_color", VertexFormat::Float4),
            ]
        );

        let text_pipeline = make_text_pipeline(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("in_texcoord", VertexFormat::Float2),
                VertexAttribute::new("in_color", VertexFormat::Float4),
            ]
        );

        let textures = HashMap::new();

        let vertex_buffers = vec![Buffer::stream(ctx, BufferType::VertexBuffer, 1)];
        let index_buffer = Buffer::stream(ctx, BufferType::IndexBuffer, 1);

        let layout = Bindings {
            vertex_buffers,
            index_buffer,
            images: Vec::new()
        };

        let default_texture = Texture::new(ctx, TextureAccess::Static, Some(&[255, 255, 255, 255]), TextureParams { format: TextureFormat::RGBA8, wrap: TextureWrap::Clamp, filter: FilterMode::Linear, width: 1, height: 1 });

        YakuiMiniquadState {
            main_pipeline,
            text_pipeline,
            textures,
            layout,
            default_texture,
            vertices: Buffer::stream(ctx, BufferType::VertexBuffer, 1),
            indices: Buffer::stream(ctx, BufferType::IndexBuffer, 1),
            commands: Vec::new()
        }

    }

    pub fn paint(&mut self, state: &mut yakui::Yakui, ctx: &mut GraphicsContext) {

        let paint = state.paint();

        self.update_textures(ctx, paint);

        if paint.calls().is_empty() {
            return;
        }

        self.update_buffers(ctx, paint);

        {

            let mut last_clip = None;

            for command in &self.commands {

                match command.pipeline {
                    yakui::paint::Pipeline::Main => ctx.apply_pipeline(&self.main_pipeline),
                    yakui::paint::Pipeline::Text => ctx.apply_pipeline(&self.text_pipeline),
                    _ => continue,
                }

                if command.clip != last_clip {
                    last_clip = command.clip;

                    let surface = paint.surface_size().as_uvec2();

                    match command.clip {
                        Some(rect) => {
                            let pos = rect.pos().as_uvec2();
                            let size = rect.size().as_uvec2();

                            let max = (pos + size).min(surface);
                            let size = yakui::UVec2::new(
                                max.x.saturating_sub(pos.x),
                                max.y.saturating_sub(pos.y),
                            );

                            // If the scissor rect isn't valid, we can skip this
                            // entire draw call.
                            if pos.x > surface.x || pos.y > surface.y || size.x == 0 || size.y == 0
                            {
                                continue;
                            }

                            ctx.apply_scissor_rect(pos.x as i32, pos.y as i32, size.x as i32, size.y as i32);
                        }
                        None => {
                            ctx.apply_scissor_rect(0, 0, surface.x as i32, surface.y as i32);
                        }
                    }
                }

                let base_element = command.index_range.start as i32;
                let number_of_elements_to_draw = (command.index_range.end - command.index_range.start) as i32;
                let command_bindings = Bindings { vertex_buffers: vec![self.vertices], index_buffer: self.indices, images: vec![command.texture] };

                ctx.apply_bindings(&command_bindings);
                ctx.draw(base_element, number_of_elements_to_draw, 1);

            }

        }

    }

    fn update_buffers(&mut self, ctx: &mut GraphicsContext, paint: &PaintDom) {

        let commands = paint.calls();
        self.commands.clear();

        let mut draw_vertices: Vec<YakuiVertex> = Vec::new();
        let mut draw_indices: Vec<u16> = Vec::new();
        let mut draw_commands = Vec::new();

        for mesh in commands {

            let vertices = mesh.vertices.iter().map(|v| YakuiVertex {
                pos: v.position,
                texcoord: v.texcoord,
                color: v.color
            });

            let base = draw_vertices.len() as u16;
            let indices: Vec<u16> = mesh.indices.iter().map(|&index| base + index as u16).collect();

            let start = draw_indices.len() as u32;
            let end = start + indices.len() as u32;

            let texture = mesh
                .texture
                .and_then(|index| self.textures.get(&index));

            draw_vertices.extend(vertices);
            draw_indices.extend(&indices);

            let new_draw_command = DrawCommand {
                index_range: start..end,
                texture: *texture.unwrap_or(&self.default_texture),
                pipeline: mesh.pipeline,
                clip: mesh.clip
            };

            draw_commands.push(new_draw_command);

        }

        // upload the buffers at last
        let size_of_vertex_data_in_bytes = draw_vertices.len() * std::mem::size_of::<yakui::paint::Vertex>();
        if self.vertices.size() < size_of_vertex_data_in_bytes {
            self.vertices.delete();
            self.vertices = Buffer::stream(ctx, BufferType::VertexBuffer, size_of_vertex_data_in_bytes);
            self.vertices.update(ctx, &draw_vertices);
            self.layout.vertex_buffers = vec![self.vertices];
        } else {
            self.vertices.update(ctx, &draw_vertices);
        }

        let size_of_index_data_in_bytes = draw_indices.len() * std::mem::size_of::<u16>();
        if self.indices.size() < size_of_index_data_in_bytes {
            self.indices.delete();
            self.indices = Buffer::index_stream(ctx, IndexType::Short, size_of_index_data_in_bytes);
            self.indices.update(ctx, &draw_indices);
            self.layout.index_buffer = self.indices;
        } else {
            self.indices.update(ctx, &draw_indices);
        }

        self.commands.extend(draw_commands);

    }

    fn update_textures(&mut self, ctx: &mut GraphicsContext, paint: &PaintDom) {

        for (id, texture) in paint.textures() {
            if self.textures.contains_key(&id) == false {
                self.textures.insert(id, make_texture(ctx, texture));
            }
        }

        for (id, change) in paint.texture_edits() {
            match change {
                yakui::paint::TextureChange::Added => {
                    let texture = paint.texture(id).unwrap();
                    self.textures.insert(id, make_texture(ctx, texture));
                },
                yakui::paint::TextureChange::Removed => {
                    if let Some(t) = self.textures.remove(&id) {
                        t.delete();
                    }
                },
                yakui::paint::TextureChange::Modified => {
                    if let Some(existing) = self.textures.get_mut(&id) {
                        let texture = paint.texture(id).unwrap();
                        existing.update(ctx, texture.data());
                    }
                }
            }
        }

    }

}

fn miniquad_mouse_button_to_yakui(button: MouseButton) -> Option<yakui::input::MouseButton> {
    match button {
        MouseButton::Left => Some(YakuiMouseButton::One),
        MouseButton::Middle => Some(YakuiMouseButton::Two),
        MouseButton::Right => Some(YakuiMouseButton::Three),
        MouseButton::Unknown => None
    }
}

fn miniquad_key_to_yakui(key: KeyCode) -> Option<YakuiKeyCode> {
    match key {
        KeyCode::Space => Some(YakuiKeyCode::Space),
        KeyCode::Apostrophe => Some(YakuiKeyCode::Quote),
        KeyCode::Comma => Some(YakuiKeyCode::Comma),
        KeyCode::Minus => Some(YakuiKeyCode::Minus),
        KeyCode::Period => Some(YakuiKeyCode::Period),
        KeyCode::Slash => Some(YakuiKeyCode::Slash),
        KeyCode::Key0 => Some(YakuiKeyCode::Digit0),
        KeyCode::Key1 => Some(YakuiKeyCode::Digit1),
        KeyCode::Key2 => Some(YakuiKeyCode::Digit2),
        KeyCode::Key3 => Some(YakuiKeyCode::Digit3),
        KeyCode::Key4 => Some(YakuiKeyCode::Digit4),
        KeyCode::Key5 => Some(YakuiKeyCode::Digit5),
        KeyCode::Key6 => Some(YakuiKeyCode::Digit6),
        KeyCode::Key7 => Some(YakuiKeyCode::Digit7),
        KeyCode::Key8 => Some(YakuiKeyCode::Digit8),
        KeyCode::Key9 => Some(YakuiKeyCode::Digit9),
        KeyCode::Semicolon => Some(YakuiKeyCode::Semicolon),
        KeyCode::Equal => Some(YakuiKeyCode::Equal),
        KeyCode::A => Some(YakuiKeyCode::KeyA),
        KeyCode::B => Some(YakuiKeyCode::KeyB),
        KeyCode::C => Some(YakuiKeyCode::KeyC),
        KeyCode::D => Some(YakuiKeyCode::KeyD),
        KeyCode::E => Some(YakuiKeyCode::KeyE),
        KeyCode::F => Some(YakuiKeyCode::KeyF),
        KeyCode::G => Some(YakuiKeyCode::KeyG),
        KeyCode::H => Some(YakuiKeyCode::KeyH),
        KeyCode::I => Some(YakuiKeyCode::KeyI),
        KeyCode::J => Some(YakuiKeyCode::KeyJ),
        KeyCode::K => Some(YakuiKeyCode::KeyK),
        KeyCode::L => Some(YakuiKeyCode::KeyL),
        KeyCode::M => Some(YakuiKeyCode::KeyM),
        KeyCode::N => Some(YakuiKeyCode::KeyN),
        KeyCode::O => Some(YakuiKeyCode::KeyO),
        KeyCode::P => Some(YakuiKeyCode::KeyP),
        KeyCode::Q => Some(YakuiKeyCode::KeyQ),
        KeyCode::R => Some(YakuiKeyCode::KeyR),
        KeyCode::S => Some(YakuiKeyCode::KeyR),
        KeyCode::T => Some(YakuiKeyCode::KeyT),
        KeyCode::U => Some(YakuiKeyCode::KeyU),
        KeyCode::V => Some(YakuiKeyCode::KeyV),
        KeyCode::W => Some(YakuiKeyCode::KeyW),
        KeyCode::X => Some(YakuiKeyCode::KeyX),
        KeyCode::Y => Some(YakuiKeyCode::KeyY),
        KeyCode::Z => Some(YakuiKeyCode::KeyZ),
        KeyCode::LeftBracket => Some(YakuiKeyCode::BracketLeft),
        KeyCode::Backslash => Some(YakuiKeyCode::Backslash),
        KeyCode::RightBracket => Some(YakuiKeyCode::BracketRight),
        KeyCode::GraveAccent => Some(YakuiKeyCode::Backquote),
        KeyCode::World1 => None,  // #FIXME: what is wordl1 in yakui?
        KeyCode::World2 => None, // #FIXME: what is world2 in yakui?
        KeyCode::Escape => Some(YakuiKeyCode::Escape),
        KeyCode::Enter => Some(YakuiKeyCode::Enter),
        KeyCode::Tab => Some(YakuiKeyCode::Tab),
        KeyCode::Backspace => Some(YakuiKeyCode::Backspace),
        KeyCode::Insert => Some(YakuiKeyCode::Insert),
        KeyCode::Delete => Some(YakuiKeyCode::Delete),
        KeyCode::Right => Some(YakuiKeyCode::ArrowRight),
        KeyCode::Left => Some(YakuiKeyCode::ArrowLeft),
        KeyCode::Down => Some(YakuiKeyCode::ArrowDown),
        KeyCode::Up => Some(YakuiKeyCode::ArrowUp),
        KeyCode::PageUp => Some(YakuiKeyCode::PageUp),
        KeyCode::PageDown => Some(YakuiKeyCode::PageDown),
        KeyCode::Home => Some(YakuiKeyCode::Home),
        KeyCode::End => Some(YakuiKeyCode::End),
        KeyCode::CapsLock => Some(YakuiKeyCode::CapsLock),
        KeyCode::ScrollLock => Some(YakuiKeyCode::ScrollLock),
        KeyCode::NumLock => Some(YakuiKeyCode::NumLock),
        KeyCode::PrintScreen => Some(YakuiKeyCode::PrintScreen),
        KeyCode::Pause => Some(YakuiKeyCode::Pause),
        KeyCode::F1 => Some(YakuiKeyCode::F1),
        KeyCode::F2 => Some(YakuiKeyCode::F2),
        KeyCode::F3 => Some(YakuiKeyCode::F3),
        KeyCode::F4 => Some(YakuiKeyCode::F4),
        KeyCode::F5 => Some(YakuiKeyCode::F5),
        KeyCode::F6 => Some(YakuiKeyCode::F6),
        KeyCode::F7 => Some(YakuiKeyCode::F7),
        KeyCode::F8 => Some(YakuiKeyCode::F8),
        KeyCode::F9 => Some(YakuiKeyCode::F9),
        KeyCode::F10 => Some(YakuiKeyCode::F10),
        KeyCode::F11 => Some(YakuiKeyCode::F11),
        KeyCode::F12 => Some(YakuiKeyCode::F12),
        KeyCode::F13 => Some(YakuiKeyCode::F13),
        KeyCode::F14 => Some(YakuiKeyCode::F14),
        KeyCode::F15 => Some(YakuiKeyCode::F15),
        KeyCode::F16 => Some(YakuiKeyCode::F16),
        KeyCode::F17 => Some(YakuiKeyCode::F17),
        KeyCode::F18 => Some(YakuiKeyCode::F18),
        KeyCode::F19 => Some(YakuiKeyCode::F19),
        KeyCode::F20 => Some(YakuiKeyCode::F20),
        KeyCode::F21 => Some(YakuiKeyCode::F21),
        KeyCode::F22 => Some(YakuiKeyCode::F22),
        KeyCode::F23 => Some(YakuiKeyCode::F23),
        KeyCode::F24 => Some(YakuiKeyCode::F24),
        KeyCode::F25 => None, // #FIXME: do we even care about F25? seems very unlikely
        KeyCode::Kp0 => Some(YakuiKeyCode::Numpad0),
        KeyCode::Kp1 => Some(YakuiKeyCode::Numpad1),
        KeyCode::Kp2 => Some(YakuiKeyCode::Numpad2),
        KeyCode::Kp3 => Some(YakuiKeyCode::Numpad3),
        KeyCode::Kp4 => Some(YakuiKeyCode::Numpad4),
        KeyCode::Kp5 => Some(YakuiKeyCode::Numpad5),
        KeyCode::Kp6 => Some(YakuiKeyCode::Numpad6),
        KeyCode::Kp7 => Some(YakuiKeyCode::Numpad7),
        KeyCode::Kp8 => Some(YakuiKeyCode::Numpad8),
        KeyCode::Kp9 => Some(YakuiKeyCode::Numpad9),
        KeyCode::KpDecimal => Some(YakuiKeyCode::NumpadDecimal),
        KeyCode::KpDivide => Some(YakuiKeyCode::NumpadDivide),
        KeyCode::KpMultiply => Some(YakuiKeyCode::NumpadMultiply),
        KeyCode::KpSubtract => Some(YakuiKeyCode::NumpadSubtract),
        KeyCode::KpAdd => Some(YakuiKeyCode::NumpadAdd),
        KeyCode::KpEnter => Some(YakuiKeyCode::NumpadEnter),
        KeyCode::KpEqual => Some(YakuiKeyCode::NumpadEqual),
        KeyCode::LeftShift => Some(YakuiKeyCode::ShiftLeft),
        KeyCode::LeftControl => Some(YakuiKeyCode::ControlLeft),
        KeyCode::LeftAlt => Some(YakuiKeyCode::AltLeft),
        KeyCode::LeftSuper => Some(YakuiKeyCode::Super), // #FIXME: is this left super or right super? are they the same?
        KeyCode::RightShift => Some(YakuiKeyCode::ShiftRight),
        KeyCode::RightControl => Some(YakuiKeyCode::ControlRight),
        KeyCode::RightAlt => Some(YakuiKeyCode::AltRight),
        KeyCode::RightSuper => Some(YakuiKeyCode::Super), // #FIXME: is this left super or right super? are they the same?
        KeyCode::Menu => Some(YakuiKeyCode::ContextMenu),
        KeyCode::Unknown => None,
    }
}

fn resolve_texture_format(format: paint::TextureFormat) -> TextureFormat {
    match format {
        paint::TextureFormat::Rgba8Srgb => TextureFormat::RGBA8,
        paint::TextureFormat::R8 => TextureFormat::Alpha,
        _ => panic!("[yakui-miniquad]: got unexpected texture format: {:?}", format),
    }
}

fn make_texture(ctx: &mut GraphicsContext, texture: &paint::Texture) -> Texture {
    let texture_format = resolve_texture_format(texture.format());
    let dimensions = texture.size();
    Texture::new(
        ctx,
        TextureAccess::Static,
        Some(texture.data()),
        TextureParams {
            format: texture_format,
            wrap: TextureWrap::Clamp,
            filter: FilterMode::Linear,
            width: dimensions.x,
            height: dimensions.y
        }
    )
}

fn make_alpha_blend_state() -> BlendState {
    BlendState::new(Equation::Add, BlendFactor::Value(BlendValue::SourceAlpha), BlendFactor::OneMinusValue(BlendValue::SourceAlpha))
}

fn make_premultiplied_alpha_blend_state() -> BlendState {
    BlendState::new(Equation::Add, BlendFactor::One, BlendFactor::OneMinusValue(BlendValue::SourceAlpha))
}

fn make_main_pipeline(ctx: &mut GraphicsContext, buffers: &[BufferLayout], attributes: &[VertexAttribute]) -> Pipeline {

    let main_shader = Shader::new(ctx, yakui_shader_main::VERTEX, yakui_shader_main::FRAGMENT, yakui_shader_main::meta()).expect("[yakui-miniquad]: could not compile main shader!");

    let pipeline_params = PipelineParams {
        cull_face: CullFace::Nothing,
        front_face_order: FrontFaceOrder::CounterClockwise,
        depth_test: Comparison::Never,
        depth_write: false,
        depth_write_offset: None,
        color_blend: Some(make_alpha_blend_state()),
        alpha_blend: None, // set none so that we use the same as for color blending when blending alpha
        stencil_test: None,
        color_write: (true, true, true, true),
        primitive_type: PrimitiveType::Triangles
    };

    Pipeline::with_params(
        ctx,
        buffers,
        attributes,
        main_shader,
        pipeline_params
    )

}

fn make_text_pipeline(ctx: &mut GraphicsContext, buffers: &[BufferLayout], attributes: &[VertexAttribute]) -> Pipeline {

    let text_shader = Shader::new(ctx, yakui_shader_text::VERTEX, yakui_shader_text::FRAGMENT, yakui_shader_text::meta()).expect("[yakui-miniquad]: could not compile text shader!");

    let pipeline_params = PipelineParams {
        cull_face: CullFace::Nothing,
        front_face_order: FrontFaceOrder::CounterClockwise,
        depth_test: Comparison::Never,
        depth_write: false,
        depth_write_offset: None,
        color_blend: Some(make_premultiplied_alpha_blend_state()),
        alpha_blend: None, // set none so that we use the same as for color blending when blending alpha
        stencil_test: None,
        color_write: (true, true, true, true),
        primitive_type: PrimitiveType::Triangles
    };

    Pipeline::with_params(
        ctx,
        buffers,
        attributes,
        text_shader,
        pipeline_params
    )

}

mod yakui_shader_main {

    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 in_pos;
    attribute vec2 in_texcoord;
    attribute vec4 in_color;

    varying lowp vec2 out_texcoord;
    varying lowp vec4 out_color;

    void main() {
        lowp vec2 adjusted = in_pos * vec2(2.0, -2.0) + vec2(-1.0, 1.0);
        gl_Position = vec4(adjusted, 0, 1);
        out_texcoord = in_texcoord;
        out_color = in_color;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 out_texcoord;
    varying lowp vec4 out_color;

    uniform sampler2D color_texture;

    void main() {
        lowp vec4 color = texture2D(color_texture, out_texcoord);
        gl_FragColor = out_color * color;
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["color_texture".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![],
            },
        }
    }

}

mod yakui_shader_text {

    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 in_pos;
    attribute vec2 in_texcoord;
    attribute vec4 in_color;

    varying lowp vec2 out_texcoord;
    varying lowp vec4 out_color;

    void main() {
        lowp vec2 adjusted = in_pos * vec2(2.0, -2.0) + vec2(-1.0, 1.0);
        gl_Position = vec4(adjusted, 0, 1);
        out_texcoord = in_texcoord;
        out_color = in_color;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 out_texcoord;
    varying lowp vec4 out_color;

    uniform sampler2D coverage_texture;

    void main() {
        lowp float coverage = texture2D(coverage_texture, out_texcoord).r;
        lowp float alpha = coverage * out_color.a;

        gl_FragColor = vec4(out_color.rgb * alpha, alpha);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["coverage_texture".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![],
            },
        }
    }

}