use crate::hud_layout::{GuiDraw, PoseOp};
use crate::native_chat_animation as ca;
use crate::screen_draw::Draw;
use crate::text::Text;

/// A chat of two lines as `Chat::layout` gives it (line height 9, bottom at 100).
fn chat(tag: bool) -> Vec<GuiDraw> {
    let mut v = vec![GuiDraw::Push(vec![PoseOp::Scale(1.0, 1.0), PoseOp::Translate(4.0, 0.0)])];
    for i in 0..2 {
        let bottom = 100 - i * 9;
        v.push(GuiDraw::Fill { x0: -4, y0: bottom - 9, x1: 328, y1: bottom, color: 0x8000_0000 });
    }
    for i in 0..2 {
        let bottom = 100 - i * 9;
        v.push(GuiDraw::Text { text: Text, x: 0, y: bottom - 8, color: 0xFFFF_FFFF, shadow: true });
        if tag {
            v.push(GuiDraw::Fill { x0: -4, y0: bottom - 9, x1: -2, y1: bottom, color: 0xFFAA_AAAA });
        }
    }
    v.push(GuiDraw::Pop);
    v
}

fn offset(d: &[GuiDraw]) -> f32 {
    match &d[0] {
        GuiDraw::Push(ops) => ops.iter().filter_map(|o| match o { PoseOp::Translate(0.0, y) => Some(*y), _ => None }).sum(),
        _ => 0.0,
    }
}

// The tests share the module's state: run them one at a time.
static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn a_new_message_rises_into_place_and_fades_in() {
    let _l = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    ca::set_active(true);
    // First sight of the chat: nothing moves.
    let mut d = chat(false);
    ca::messages(&mut d, 5, 10, 1, 9, false);
    assert_eq!(offset(&d), 0.0);
    // A message arrives: the chat starts a line lower, the new line nearly transparent.
    let mut d = chat(false);
    ca::messages(&mut d, 6, 20, 1, 9, false);
    let first = offset(&d);
    assert!(first > 8.0 && first <= 9.0, "{first}");
    let alpha = |d: &[GuiDraw], y: i32| d.iter().find_map(|x| match x { GuiDraw::Text { y: ty, color, .. } if *ty == y => Some(color >> 24), _ => None }).unwrap();
    assert!(alpha(&d, 92) < 40, "new line faded");
    assert_eq!(alpha(&d, 83), 255, "older line as it was");
    // Half way: higher, more opaque; after the duration: in place.
    std::thread::sleep(std::time::Duration::from_millis(60));
    let mut mid = chat(false);
    ca::messages(&mut mid, 6, 20, 1, 9, false);
    assert!(offset(&mid) < first && offset(&mid) > 0.0);
    assert!(alpha(&mid, 92) > alpha(&d, 92));
    std::thread::sleep(std::time::Duration::from_millis(120));
    let mut done = chat(false);
    ca::messages(&mut done, 6, 20, 1, 9, false);
    assert_eq!(offset(&done), 0.0);
    assert_eq!(alpha(&done, 92), 255);
}

#[test]
fn scrolled_up_the_chat_stays_put() {
    let _l = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    ca::set_active(true);
    let mut d = chat(false);
    ca::messages(&mut d, 100, 1, 1, 9, false);
    let mut d = chat(false);
    ca::messages(&mut d, 101, 2, 1, 9, true);
    assert_eq!(offset(&d), 0.0);
}

#[test]
fn the_indicator_bar_is_left_out() {
    let _l = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    ca::set_active(true);
    let mut d = chat(true);
    ca::messages(&mut d, 200, 1, 1, 9, false);
    assert!(!d.iter().any(|x| matches!(x, GuiDraw::Fill { x0: -4, x1: -2, .. })));
    assert_eq!(d.iter().filter(|x| matches!(x, GuiDraw::Fill { .. })).count(), 2, "backgrounds kept");
    ca::set_active(false);
    let mut d = chat(true);
    ca::messages(&mut d, 201, 2, 1, 9, false);
    assert!(d.iter().any(|x| matches!(x, GuiDraw::Fill { x0: -4, x1: -2, .. })), "off in Mods: as Horizon draws it");
    ca::set_active(true);
}

#[test]
fn the_input_field_slides_up_from_the_bottom() {
    let _l = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    ca::set_active(true);
    ca::input_opened();
    let first = ca::input_offset();
    assert!(first >= 12 && first <= 14, "{first}");
    let mut d = vec![Draw::Fill { x0: 2, y0: 286, x1: 478, y1: 298, color: -1 }, Draw::Text { text: Text, x: 4, y: 288, color: -1, shadow: true }];
    ca::shift(&mut d, first);
    assert!(matches!(d[0], Draw::Fill { y0, y1, .. } if y0 == 286 + first && y1 == 298 + first));
    std::thread::sleep(std::time::Duration::from_millis(200));
    assert_eq!(ca::input_offset(), 0);
}

#[test]
fn the_button_takes_the_free_half_of_the_last_row() {
    // Options at 480 x 289: FOV row, then the grid; Credits alone in the last row.
    let (w, h) = (480.0, 289.0);
    let cx = 240.0;
    let mut grid = Vec::new();
    for r in 0..5 {
        let y = 90.0 + r as f32 * 24.0;
        grid.push([cx - 155.0, y, 150.0]);
        if r < 4 {
            grid.push([cx + 5.0, y, 150.0]);
        }
    }
    let others: Vec<[f32; 4]> = grid.iter().map(|g| [g[0], g[1], g[2], 20.0]).collect();
    let r = ca::place(&grid, &others, w, h).unwrap();
    assert_eq!(r, [cx + 4.0, 186.0, 150.0, 20.0]);
    // Another mod took that half: the gap above the grid.
    let mut grid2 = grid.clone();
    grid2.push([cx + 4.0, 186.0, 150.0]);
    let mut others2 = others.clone();
    others2.push([cx + 4.0, 186.0, 150.0, 20.0]);
    let r = ca::place(&grid2, &others2, w, h).unwrap();
    assert_eq!(r[1], 66.0);
}

#[test]
fn settings_read_back_and_the_screen_changes_them() {
    let _l = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let c = ca::parse(&ca::to_text(&ca::DEFAULT));
    assert_eq!(c, ca::DEFAULT);
    assert_eq!(ca::parse("message_ms=9999\nhide_indicator=0").message_ms, ca::MAX_MS);
    // The screen at 480 x 289: Animate Messages' button at (285, 52).
    ca::menu_open();
    let before = ca::config().animate_messages;
    assert_eq!(ca::menu_mouse(480.0, 289.0, 300.0, 60.0, true), ca::MenuOut::Click);
    assert_eq!(ca::config().animate_messages, !before);
    // Its reset (at 410) puts it back.
    assert_eq!(ca::menu_mouse(480.0, 289.0, 415.0, 60.0, true), ca::MenuOut::Click);
    assert_eq!(ca::config().animate_messages, ca::DEFAULT.animate_messages);
    // Dragging the Duration slider (row at 77) to its far end: 750 ms.
    ca::menu_mouse(480.0, 289.0, 300.0, 85.0, true);
    ca::menu_drag(480.0, 289.0, 500.0);
    ca::menu_mouse(480.0, 289.0, 500.0, 85.0, false);
    assert_eq!(ca::config().message_ms, 750);
    ca::menu_mouse(480.0, 289.0, 415.0, 85.0, true);
    assert_eq!(ca::config().message_ms, 150);
    // Done.
    assert_eq!(ca::menu_mouse(480.0, 289.0, 240.0, 270.0, true), ca::MenuOut::Close);
    assert!(!ca::menu_is_open());
}
