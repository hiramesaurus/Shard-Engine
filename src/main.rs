
//The sdl2 crate is a safe Rust wrapper around SDL2 C API
///https://rust-sdl2.github.io/rust-sdl2/sdl2/
extern crate sdl2;

///https://docs.rs/gl/0.11.0/gl/
extern crate gl;

use std::ffi::{CString, CStr};

pub mod render_gl;
///http://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-01-window.html
fn main() {

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4,5);

    let window = video_subsystem
        .window("Game", 900,700)             //hakee vissiin vaan builderin sitä ikkunaa varten
        .opengl()                                             //add opengl flag
        .resizable()
        .build()                                              //tekee ite ikkunan
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0,0,900,700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let shader = render_gl::Shader::from_vertex_source(
        &CStr::from_bytes_with_nul(b"<source code here>\0").unwrap());


    //application event listener(?)
    let mut event_pump = sdl.event_pump().unwrap();

    'main: loop{
        //input loop
        for event in event_pump.poll_iter(){
            match  event{
                sdl2::event::Event::Quit {..} => break 'main,
                _ =>{},
            }
        }
        //render window contents here
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
    }



}
