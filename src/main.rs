extern crate sdl2;
extern crate gl;
extern crate vec_2_10_10_10;
extern crate nalgebra;

//#[macro_use] extern crate failure;
#[macro_use] extern crate render_gl_derive;

pub mod render_gl;
pub mod resources;
mod triangle;

use std::time::Duration;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

//use failure::err_msg;

use resources::Resources;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}

fn run() -> Result<(), anyhow::Error> {
    let sdl_context = sdl2::init().map_err(|e| anyhow::anyhow!(e))?;
    let video_subsystem = sdl_context.video().unwrap();

    let res = Resources::from_relative_exe_path(Path::new("assets"))?;

    let window = video_subsystem.window("probando", 900, 700)
        .position_centered()
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut viewport = render_gl::Viewport::for_window(900, 700);
    viewport.set_used(&gl);

    let color_buffer = render_gl::ColorBuffer::from_color(nalgebra::Vector3::new(0.3, 0.3, 0.5));
    color_buffer.set_used(&gl);

    let triangle = triangle::Triangle::new(&res, &gl)?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::Window { win_event: sdl2::event::WindowEvent::Resized(w, h), .. } => {
                    viewport.update_size(w, h);
                    viewport.set_used(&gl);
                },
                _ => {}
            }
        }

        color_buffer.clear(&gl);

        triangle.render(&gl);

        window.gl_swap_window();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

