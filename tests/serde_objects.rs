#![cfg(feature = "serde")]

extern crate serde;
extern crate winit;

use winit::window::{MouseCursor};
use winit::event::{
    KeyboardInput, TouchPhase, ElementState, MouseButton, MouseScrollDelta, VirtualKeyCode,
    ModifiersState
};
use winit::dpi::{LogicalPosition, PhysicalPosition, LogicalSize, PhysicalSize};
use serde::{Serialize, Deserialize};

fn needs_serde<S: Serialize + Deserialize<'static>>() {}

#[test]
fn window_serde() {
    needs_serde::<MouseCursor>();
}

#[test]
fn events_serde() {
    needs_serde::<KeyboardInput>();
    needs_serde::<TouchPhase>();
    needs_serde::<ElementState>();
    needs_serde::<MouseButton>();
    needs_serde::<MouseScrollDelta>();
    needs_serde::<VirtualKeyCode>();
    needs_serde::<ModifiersState>();
}

#[test]
fn dpi_serde() {
    needs_serde::<LogicalPosition>();
    needs_serde::<PhysicalPosition>();
    needs_serde::<LogicalSize>();
    needs_serde::<PhysicalSize>();
}
