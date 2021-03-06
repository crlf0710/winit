extern crate winit;

use winit::window::{WindowBuilder, MouseCursor};
use winit::event::{Event, WindowEvent, ElementState, KeyboardInput};
use winit::event_loop::{EventLoop, ControlFlow};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("A fantastic window!");

    let mut cursor_idx = 0;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event: WindowEvent::KeyboardInput { input: KeyboardInput { state: ElementState::Pressed, .. }, .. }, .. } => {
                println!("Setting cursor to \"{:?}\"", CURSORS[cursor_idx]);
                window.set_cursor(CURSORS[cursor_idx]);
                if cursor_idx < CURSORS.len() - 1 {
                    cursor_idx += 1;
                } else {
                    cursor_idx = 0;
                }
            },
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
                return;
            },
            _ => ()
        }
    });
}

const CURSORS: &[MouseCursor] = &[
    MouseCursor::Default, MouseCursor::Crosshair, MouseCursor::Hand,
    MouseCursor::Arrow, MouseCursor::Move, MouseCursor::Text,
    MouseCursor::Wait, MouseCursor::Help, MouseCursor::Progress,
    MouseCursor::NotAllowed, MouseCursor::ContextMenu, MouseCursor::Cell,
    MouseCursor::VerticalText, MouseCursor::Alias, MouseCursor::Copy,
    MouseCursor::NoDrop, MouseCursor::Grab, MouseCursor::Grabbing,
    MouseCursor::AllScroll, MouseCursor::ZoomIn, MouseCursor::ZoomOut,
    MouseCursor::EResize, MouseCursor::NResize, MouseCursor::NeResize,
    MouseCursor::NwResize, MouseCursor::SResize, MouseCursor::SeResize,
    MouseCursor::SwResize, MouseCursor::WResize, MouseCursor::EwResize,
    MouseCursor::NsResize, MouseCursor::NeswResize, MouseCursor::NwseResize,
    MouseCursor::ColResize, MouseCursor::RowResize
];
