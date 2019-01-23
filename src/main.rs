
//The sdl2 crate is a safe Rust wrapper around SDL2 C API
///https://rust-sdl2.github.io/rust-sdl2/sdl2/
extern crate sdl2;

///http://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-01-window.html
fn main() {

    let sdl = sdl2::init().unwrap();
    let vidoe_subsystem = sdl.video().unwrap();
    let window = vidoe_subsystem
        .window("Game", 900,700)//hakee vissiin vaan builderin sitä ikkunaa varten
        .resizable()
        .build()//tekee ite ikkunan
        .unwrap();

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
    }

}