#[macro_use]
extern crate glium;
extern crate glm;

mod strandbeest;
mod dynamics;
mod view;

use glm::*;
use glium::Surface;

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let mut model = strandbeest::make_strandbeest(&vec2(0.0, 0.0));
    let mut view = view::Renderer::new(&display);

    loop {
        // listing the events produced by the window and waiting to be received
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,   // the window has been closed by the user
                _ => ()
            }
        }

        view.update(&display, &model);

        let mut target = display.draw();
        target.clear_color(0.5, 0.5, 0.5, 1.0);

        view.render(&mut target);

        target.finish().unwrap();
    }
}
