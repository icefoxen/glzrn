//! New api fiddling goes here
use std::path::PathBuf;

use crate::dpi::{LogicalPosition, LogicalSize};
use crate::event::Event;
use crate::event_loop;
use crate::window::WindowId;

pub trait EventHandler {
    fn handle(&mut self, _event: Event<()>);
}

pub struct Heck {
    el: event_loop::EventLoop<()>,
}

impl Heck {
    pub fn poll_events(&mut self) -> impl Iterator<Item = Event<()>> {
        None.into_iter()
    }

    pub fn run(self, handler: impl EventHandler + 'static) {
        // we own handler we can do what ever we want to it
        let mut handler = handler;
        self.el.run(move |event, _, controlflow| {
            *controlflow = event_loop::ControlFlow::Poll;
            handler.handle(event);
        });
    }
}
