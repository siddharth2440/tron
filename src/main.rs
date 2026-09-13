use crate::app::App;
use winit::event_loop::{ControlFlow, EventLoop};

mod app;
mod state;
mod texture;
mod coords;
mod cube;

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().expect("Eventloop New error");

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();

    if let Err(event_loop_app_run_err) = event_loop.run_app(&mut app) {
        println!("Error: EventLoop run error: {event_loop_app_run_err}");
        std::process::exit(1);
    }
}
