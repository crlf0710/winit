use super::wayland::core::{Display, Registry, Compositor, Shell, Output,
                           Seat, Pointer, default_display, WSurface, SurfaceId};

use std::collections::{VecDeque, HashMap};
use std::sync::{Arc, Mutex};

use Event;

pub struct WaylandContext {
    pub display: Display,
    pub registry: Registry,
    pub compositor: Compositor,
    pub shell: Shell,
    pub seat: Seat,
    pub pointer: Option<Pointer<WSurface>>,
    windows_event_queues: Arc<Mutex<HashMap<SurfaceId, Arc<Mutex<VecDeque<Event>>>>>>,
    current_pointer_surface: Arc<Mutex<Option<SurfaceId>>>,
    pub outputs: Vec<Arc<Output>>
}

impl WaylandContext {
    pub fn new() -> Option<WaylandContext> {
        let display = match default_display() {
            Some(d) => d,
            None => return None,
        };
        let registry = display.get_registry();
        // let the registry get its events
        display.sync_roundtrip();
        let compositor = match registry.get_compositor() {
            Some(c) => c,
            None => return None,
        };
        let shell = match registry.get_shell() {
            Some(s) => s,
            None => return None,
        };
        let seat = match registry.get_seats().into_iter().next() {
            Some(s) => s,
            None => return None,
        };
        let outputs = registry.get_outputs().into_iter().map(Arc::new).collect::<Vec<_>>();
        // let the other globals get their events
        display.sync_roundtrip();

        let current_pointer_surface = Arc::new(Mutex::new(None));

        // rustc has trouble finding the correct type here, so we explicit it.
        let windows_event_queues = Arc::new(Mutex::new(
            HashMap::<SurfaceId, Arc<Mutex<VecDeque<Event>>>>::new()
        ));

        // handle inputs
        let mut pointer = seat.get_pointer();
        if let Some(ref mut p) = pointer {
            // set the enter/leave callbacks
            let current_surface = current_pointer_surface.clone();
            p.set_enter_action(move |_, sid, x, y| {
                *current_surface.lock().unwrap() = Some(sid);
            });
            let current_surface = current_pointer_surface.clone();
            p.set_leave_action(move |_, sid| {
                *current_surface.lock().unwrap() = None;
            });
            // set the events callbacks
            let current_surface = current_pointer_surface.clone();
            let event_queues = windows_event_queues.clone();
            p.set_motion_action(move |_, _, x, y| {
                // dispatch to the appropriate queue
                let sid = *current_surface.lock().unwrap();
                if let Some(sid) = sid {
                    let map = event_queues.lock().unwrap();
                    if let Some(queue) = map.get(&sid) {
                        queue.lock().unwrap().push_back(Event::Moved(x as i32,y as i32))
                    }
                }
            });
            let current_surface = current_pointer_surface.clone();
            let event_queues = windows_event_queues.clone();
            p.set_button_action(move |_, sid, b, s| {
                use super::wayland::core::ButtonState;
                use MouseButton;
                use ElementState;
                let button = match b {
                    0x110 => MouseButton::Left,
                    0x111 => MouseButton::Right,
                    0x112 => MouseButton::Middle,
                    _ => return
                };
                let state = match s {
                    ButtonState::WL_POINTER_BUTTON_STATE_RELEASED => ElementState::Released,
                    ButtonState::WL_POINTER_BUTTON_STATE_PRESSED => ElementState::Pressed
                };
                // dispatch to the appropriate queue
                let sid = *current_surface.lock().unwrap();
                if let Some(sid) = sid {
                    let map = event_queues.lock().unwrap();
                    if let Some(queue) = map.get(&sid) {
                        queue.lock().unwrap().push_back(Event::MouseInput(state, button))
                    }
                }
            });
        }
        Some(WaylandContext {
            display: display,
            registry: registry,
            compositor: compositor,
            shell: shell,
            seat: seat,
            pointer: pointer,
            windows_event_queues: windows_event_queues,
            current_pointer_surface: current_pointer_surface,
            outputs: outputs
        })
    }

    pub fn register_surface(&self, sid: SurfaceId, queue: Arc<Mutex<VecDeque<Event>>>) {
        self.windows_event_queues.lock().unwrap().insert(sid, queue);
    }

    pub fn deregister_surface(&self, sid: SurfaceId) {
        self.windows_event_queues.lock().unwrap().remove(&sid);
    }
}