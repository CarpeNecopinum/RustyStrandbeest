#[macro_use]
extern crate glium;
extern crate glm;

mod dynamics;
mod view;

use glm::*;
use glium::Surface;

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let mut model = dynamics::System::new();
    let pivot = model.push_mass(dynamics::Mass::pinned(vec2(0.0, 0.0)));
    let mass_d = model.push_mass(dynamics::Mass::unit(vec2(-40.1, 0.0)));
    model.push_auto_spring(pivot, mass_d);

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
