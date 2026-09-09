use std::sync::Arc;

use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};
use crate::state::State;

#[derive(Debug, Default)]
pub struct App {
    state: Option<State>
}

impl ApplicationHandler for App {

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::Destroyed => {
                println!("Window has been destroyed.");
            },
            WindowEvent::Focused(focused) => {
                println!("Window being focused: {focused}");
            },
            WindowEvent::CursorEntered { device_id } => {
                println!("Cursor Entered device_id: {device_id:?}");
            },
            WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                println!("Keyboard Input {device_id:?} {event:?} {is_synthetic}");
            },
            WindowEvent::MouseInput { device_id, state, button } => {
                println!("Mouse Input {device_id:?} {event:?} {button:?}");
            },
            WindowEvent::Resized(physical_size) => {
                println!("Window Resized: {physical_size:?}");
            },
            WindowEvent::MouseWheel { device_id, delta, phase } => {
                println!("Mouse Wheen Detected: {device_id:?} {delta:?} {phase:?}");
            },
            WindowEvent::Occluded(is_occluded) => {
                println!("Window is Occluded:{is_occluded}");
            },
            WindowEvent::RedrawRequested => {
                println!("Redraw Requested");
            },
            _ => {
                println!("Unhandled Window Event.");
            },
        }
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: ()) {
        // A UserEvent is an event that our application sends to itself through the event loop.
    }


    fn memory_warning(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // This is a notification that the operating system is warning our application about memory pressure.   
    }

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // Create a window and initialize the things.
        let window = Arc::new(
            event_loop.create_window( Window::default_attributes()).expect("Unable to create new window")
        );
    }


    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // called when winit is about to go back to waiting for events.
    }

    fn device_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        // This one is particularly interesting for games and graphics engines.
        
        // A DeviceEvent represents low-level input from a physical device, rather than an event associated 
        // with a particular window.
    }

    fn exiting(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // when the event loop is about to exit.
    }

    fn new_events(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, cause: winit::event::StartCause) {
        
    }

    fn suspended(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // indicates that the application has been suspended        
    }

}