use std::ops::{Deref, DerefMut};
use miniquad::{Context, EventHandler, KeyCode, KeyMods, MouseButton};
use yakui_core::Yakui;
use crate::YakuiMiniQuad;

/// Wrapper for [YakuiMiniQuad] implementing [EventHandler] and deleting all
/// managed textures on drop
pub struct YakuiMiniQuadOwnedHandler {
    ctx: Box<Context>,
    quad: YakuiMiniQuad,
}

impl YakuiMiniQuadOwnedHandler {
    pub fn miniquad_ctx(&mut self) -> &mut Context {
        self.ctx.deref_mut()
    }

    pub(crate) fn new(ctx: Box<Context>, quad: YakuiMiniQuad) -> Self {
        Self { ctx, quad }
    }
    /// See [YakuiMiniQuad]::start
    pub fn start(&mut self) {
        self.quad.start(self.ctx.deref_mut())
    }

    /// See [YakuiMiniQuad]::run
    pub fn run<F>(&mut self, ctx: &mut Context, ui_update_function: F)
        where
            F: FnOnce(&mut Yakui) -> (),
    {
        self.quad.run(ctx, ui_update_function)
    }
}

impl Drop for YakuiMiniQuadOwnedHandler {
    fn drop(&mut self) {
        self.quad.state.drop_textures(self.ctx.deref_mut())
    }
}

impl Deref for YakuiMiniQuadOwnedHandler {
    type Target = YakuiMiniQuad;

    fn deref(&self) -> &Self::Target {
        &self.quad
    }
}

impl DerefMut for YakuiMiniQuadOwnedHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.quad
    }
}

impl <'a> EventHandler for YakuiMiniQuadOwnedHandler {
    fn update(&mut self) {
        self.quad.update(self.ctx.deref_mut());
    }

    fn draw(&mut self) {
        self.quad.draw(self.ctx.deref_mut());
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        self.quad.resize_event(self.ctx.deref_mut(), width, height);
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.quad.mouse_motion_event(self.ctx.deref_mut(), x, y)
    }

    fn mouse_wheel_event(&mut self, x: f32, y: f32) {
        self.quad.mouse_wheel_event(self.ctx.deref_mut(), x, y);
    }

    fn mouse_button_down_event(&mut self, button: MouseButton, x: f32, y: f32) {
        self.quad.mouse_button_down_event(self.ctx.deref_mut(), button, x, y)
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, x: f32, y: f32) {
        self.quad.mouse_button_up_event(self.ctx.deref_mut(), button, x, y);
    }

    fn char_event(&mut self, character: char, keymods: KeyMods, repeat: bool) {
        self.quad.char_event(self.ctx.deref_mut(), character, keymods, repeat);
    }

    fn key_down_event(&mut self, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
        self.quad.key_down_event(self.ctx.deref_mut(), keycode, keymods, repeat);
    }

    fn key_up_event(&mut self, keycode: KeyCode, keymods: KeyMods) {
        self.quad.key_up_event(self.ctx.deref_mut(), keycode, keymods);
    }
}

/// Wrapper for [YakuiMiniQuad] implementing [EventHandler] without holding
/// onto owned context reference
pub struct YakuiMiniQuadRefHandler<'a> {
    ctx: &'a mut Context,
    quad: &'a mut YakuiMiniQuad,
}

impl<'a> YakuiMiniQuadRefHandler<'a> {
    pub(crate) fn new(ctx: &'a mut Context, quad: &'a mut YakuiMiniQuad) -> Self {
        Self { ctx, quad }
    }
}

impl <'a> EventHandler for YakuiMiniQuadRefHandler<'a> {
    fn update(&mut self) {
        self.quad.update(self.ctx);
    }

    fn draw(&mut self) {
        self.quad.draw(self.ctx);
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        self.quad.resize_event(self.ctx, width, height);
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.quad.mouse_motion_event(self.ctx, x, y)
    }

    fn mouse_wheel_event(&mut self, x: f32, y: f32) {
        self.quad.mouse_wheel_event(self.ctx, x, y);
    }

    fn mouse_button_down_event(&mut self, button: MouseButton, x: f32, y: f32) {
        self.quad.mouse_button_down_event(self.ctx, button, x, y)
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, x: f32, y: f32) {
        self.quad.mouse_button_up_event(self.ctx, button, x, y);
    }

    fn char_event(&mut self, character: char, keymods: KeyMods, repeat: bool) {
        self.quad.char_event(self.ctx, character, keymods, repeat);
    }

    fn key_down_event(&mut self, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
        self.quad.key_down_event(self.ctx, keycode, keymods, repeat);
    }

    fn key_up_event(&mut self, keycode: KeyCode, keymods: KeyMods) {
        self.quad.key_up_event(self.ctx, keycode, keymods);
    }
}