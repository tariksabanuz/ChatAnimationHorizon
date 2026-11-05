// Type check of the helpers and the module against stand-ins copying the Horizon signatures they
// touch (Horizon 6475f62f). Each helper sits in its target's module, as horizon-mixins puts it.
#![allow(dead_code, unused_variables, unused_imports, clippy::all)]
extern crate self as horizon_client;
extern crate self as horizon_client_render;

pub mod text {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Text;
    impl Text { pub fn literal(s: impl Into<String>) -> Text { Text } }
}
pub mod hud {
    use std::collections::HashMap;
    #[derive(Clone, Copy)] pub struct Rect;
    pub struct Font;
    pub struct HudAtlas { pub sprites: HashMap<String, Rect>, pub font: Font }
    pub struct HudVertex;
    pub struct HudBatch { pub vertices: Vec<HudVertex>, pub width: f32, pub height: f32 }
    impl HudBatch {
        pub fn fill(&mut self, atlas: &HudAtlas, x: f32, y: f32, w: f32, h: f32, color: u32) {}
        pub fn text(&mut self, atlas: &HudAtlas, text: &str, x: f32, y: f32, color: u32, shadow: bool) -> f32 { 0.0 }
        pub fn clip_since(&mut self, start: usize, x0: f32, y0: f32, x1: f32, y1: f32) {}
    }
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> u32 { 0 }
}
pub mod hud_layout {
    use crate::text::Text;
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum PoseOp { Translate(f32, f32), Scale(f32, f32) }
    #[derive(Clone, Debug, PartialEq)]
    pub enum GuiDraw {
        Fill { x0: i32, y0: i32, x1: i32, y1: i32, color: u32 },
        Text { text: Text, x: i32, y: i32, color: u32, shadow: bool },
        Push(Vec<PoseOp>),
        Pop,
    }
}
pub mod screen_draw {
    use crate::text::Text;
    #[derive(Clone, Debug, PartialEq)]
    pub enum Draw {
        Sprite { sprite: String, x: i32, y: i32, w: i32, h: i32, color: i32 },
        Blit { texture: String, x: i32, y: i32, w: i32, h: i32, u: f32, v: f32, src: [i32; 2], tex: [i32; 2], color: i32 },
        Fill { x0: i32, y0: i32, x1: i32, y1: i32, color: i32 },
        Highlight { x0: i32, y0: i32, x1: i32, y1: i32 },
        Text { text: Text, x: i32, y: i32, color: i32, shadow: bool },
        Scissor { x0: i32, y0: i32, x1: i32, y1: i32 },
        NoScissor,
        Tooltip { lines: Vec<Text>, x: i32, y: i32 },
    }
    pub struct Measure<'a>(pub &'a crate::hud::Font);
}
pub mod ui {
    use crate::{hud::{HudAtlas, HudBatch}, text::Text};
    pub fn menu_background(batch: &mut HudBatch, atlas: &HudAtlas, x: f32, y: f32, w: f32, h: f32, in_world: bool) {}
    pub fn list_background(batch: &mut HudBatch, atlas: &HudAtlas, top: f32, bottom: f32, in_world: bool) {}
    pub fn separators(batch: &mut HudBatch, atlas: &HudAtlas, header_bottom: f32, footer_top: f32, in_world: bool) {}
    pub fn centered_text(batch: &mut HudBatch, atlas: &HudAtlas, text: &Text, x: f32, y: f32, w: f32, color: u32) {}
    pub fn button(batch: &mut HudBatch, atlas: &HudAtlas, x: f32, y: f32, w: f32, h: f32, label: &Text, active: bool, hover: bool) {}
    pub fn slider(batch: &mut HudBatch, atlas: &HudAtlas, x: f32, y: f32, w: f32, label: &Text, value: f32, hover: bool, active: bool, dragging: bool) {}
    pub fn slider_value(x: f32, w: f32, cx: f32) -> f32 { ((cx - (x + 4.0)) / (w - 8.0)).clamp(0.0, 1.0) }
    pub fn hovered(cursor: (f32, f32), x: f32, y: f32, w: f32, h: f32) -> bool { cursor.0 >= x && cursor.1 >= y && cursor.0 < x + w && cursor.1 < y + h }
}
pub mod native_chat_animation {
    //ANIM//
}
pub mod chat {
    use crate::{hud_layout::{GuiDraw, PoseOp}, screen_draw::{Draw, Measure}, text::Text};
    pub struct ChatOptions;
    impl ChatOptions { pub fn line_height(&self) -> i32 { 9 } }
    struct Line { added: u64, content: Text }
    pub struct Chat { lines: Vec<Line>, scroll: usize, pub options: ChatOptions, pub received: u64 }
    impl Chat {
        pub fn layout(&self, height: i32, ticks: f32, focused: bool) -> Vec<GuiDraw> {
            if let ::core::ops::ControlFlow::Break(v) = __layout(self, height, ticks, focused) { return v; }
            Vec::new()
        }
    }
    pub struct SuggestLayout { pub anchor_to_bottom: bool }
    pub struct ChatScreen { layout: SuggestLayout }
    impl ChatScreen {
        pub fn new(width: i32) -> Self { __opened(width); ChatScreen { layout: SuggestLayout { anchor_to_bottom: true } } }
        pub fn draw(&mut self, m: &Measure, mouse: (i32, i32), blink_ms: u64, background: i32) -> Vec<Draw> {
            if let ::core::ops::ControlFlow::Break(v) = __input(&mut *self, m, mouse, blink_ms, background) { return v; }
            Vec::new()
        }
    }
    //LAYOUT//
    //INPUT//
    //OPENED//
}
pub mod settings { pub struct Settings; }
pub mod options_ui {
    use crate::{settings::Settings, text::Text};
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Page { Main, Video }
    #[derive(Clone, Debug, PartialEq)]
    pub enum Event { ButtonClick, Close }
    enum Act { None, Done }
    enum WKind { Button { label: Text, active: bool, act: Act } }
    struct Widget { x: f32, y: f32, w: f32, h: f32, kind: WKind, list: bool }
    fn button(x: f32, y: f32, w: f32, label: Text, act: Act) -> Widget {
        Widget { x, y, w, h: 20.0, kind: WKind::Button { label, active: true, act }, list: false }
    }
    #[derive(Default)]
    pub struct OptionsUi { pub in_world: bool }
    impl OptionsUi {
        pub fn page(&self) -> Page { Page::Main }
        fn layout(&mut self, s: &Settings, w: f32, h: f32) -> (Vec<Widget>, f32, f32, f32) {
            if let ::core::ops::ControlFlow::Break(r) = __options_layout(&mut *self, s, w, h) { return r; }
            (Vec::new(), 0.0, 0.0, 0.0)
        }
    }
    //OPTIONS_LAYOUT//
}
pub mod game {
    pub struct UiResources { pub atlas: crate::hud::HudAtlas }
    pub struct UiState { pub options: crate::options_ui::OptionsUi }
    pub struct Game<'a> { gui: UiResources, cursor: (f32, f32), ui: UiState, dragging: bool, _l: std::marker::PhantomData<&'a ()> }
    pub mod native_hooks {
        use super::Game;
        pub(crate) struct NativeMod { pub(crate) id: String, pub(crate) title: String, pub(crate) description: String, pub(crate) keys: Vec<[String; 3]> }
        impl Game<'_> {
            pub(super) fn native_on(&self, mod_id: &str) -> bool { true }
            pub(super) fn native_config_dir() -> Result<std::path::PathBuf, String> { Err(String::new()) }
            pub(super) fn native_mods(&mut self, mods: &mut Vec<NativeMod>) { __listed(&mut *mods); }
            pub(super) fn native_settings(&mut self, mod_id: &str, buttons: &mut Vec<String>) { __settings(mod_id, &mut *buttons); }
            pub(super) fn native_settings_click(&mut self, mod_id: &str, button: usize) { __settings_click(mod_id, button); }
            pub(super) fn native_frame(&mut self) { __frame(&mut *self); }
        }
        //LISTED//
        //SETTINGS//
        //SETTINGS_CLICK//
        //FRAME//
    }
    pub mod options {
        use super::Game;
        use horizon_client::{hud::HudBatch, options_ui::{Event, Page}};
        pub enum KeyCode { Escape, A }
        impl PartialEq for KeyCode { fn eq(&self, o: &Self) -> bool { true } }
        type Result<T> = std::result::Result<T, String>;
        impl Game<'_> {
            pub(super) fn gui_size(&self) -> (f32, f32) { (640.0, 345.0) }
            pub(super) fn handle_option_events(&mut self, events: Vec<Event>) -> Result<()> { Ok(()) }
            pub(super) fn draw_options(&mut self, batch: &mut HudBatch) {
                if let ::core::ops::ControlFlow::Break(()) = __options_draw(&mut *self, &mut *batch) { return; }
            }
            pub(super) fn options_mouse(&mut self, pressed: bool, right: bool) -> Result<()> {
                if let ::core::ops::ControlFlow::Break(r) = __options_mouse(&mut *self, pressed, right) { return r; }
                Ok(())
            }
            pub(super) fn options_drag(&mut self) -> Result<()> {
                if let ::core::ops::ControlFlow::Break(r) = __options_drag(&mut *self) { return r; }
                Ok(())
            }
            pub(super) fn options_scroll(&mut self, amount: f32) -> Result<()> {
                if let ::core::ops::ControlFlow::Break(r) = __options_scroll(&mut *self, amount) { return r; }
                Ok(())
            }
            pub(super) fn options_key(&mut self, key: KeyCode, text: Option<&str>) -> Result<()> {
                if let ::core::ops::ControlFlow::Break(r) = __options_key(&mut *self, key, text) { return r; }
                Ok(())
            }
        }
        //OPTIONS_DRAW//
        //OPTIONS_MOUSE//
        //OPTIONS_DRAG//
        //OPTIONS_SCROLL//
        //OPTIONS_KEY//
    }
}
