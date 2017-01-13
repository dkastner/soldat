#[macro_use]
extern crate log;
extern crate env_logger;

extern crate glium;
use glium::{Display, DisplayBuild, Surface};
use glium::glutin;

use std::thread;
use std::time::{Duration, Instant};


fn main() {
    env_logger::init().unwrap();

    let display = glutin::WindowBuilder::new().
		with_fullscreen(glutin::get_primary_monitor()).
        build_glium().
        unwrap();
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();

    loop {
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return break,
                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _,
                                             Some(glutin::VirtualKeyCode::Escape)) => return break,
                _ => ()
            }
        }

        let now = Instant::now();
        accumulator += now - previous_clock;
        previous_clock = now;

        let fixed_time_stamp = Duration::new(0, 16666667);
        while accumulator >= fixed_time_stamp {
            accumulator -= fixed_time_stamp;
            // update_state();
            draw(&display);
        }

        thread::sleep(fixed_time_stamp - accumulator);
    }
}


fn draw(ref mut display: &glium::backend::glutin_backend::GlutinFacade) {
     let mut target = display.draw();
     target.clear_color(0.0, 0.0, 0.0, 1.0);
     target.finish().unwrap();
}


