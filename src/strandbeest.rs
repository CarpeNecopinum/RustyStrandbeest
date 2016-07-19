use dynamics::System;
use dynamics::Mass;

use glm::vec2;
use glm::Vec2;

// https://en.wikipedia.org/wiki/Theo_Jansen
pub fn make_strandbeest(around: &Vec2) -> System {
    let mut system = System::new();

    // Positions just eye-balled use the springs to snap them to the right places
    let pivot = system.push_mass(Mass::pinned(vec2(0.0, 0.0)));
    let b = system.push_mass(Mass::unit(vec2(0.0, 41.5)));
    let j = system.push_mass(Mass::unit(vec2(30.0, 41.5)));
    let d = system.push_mass(Mass::unit(vec2(-40.1, 0.0)));
    let c = system.push_mass(Mass::unit(vec2(0.0, -39.0)));
    let f = system.push_mass(Mass::unit(vec2(-40.1, -39.0)));
    let i = system.push_mass(Mass::unit(vec2(0.0, -79.0)));


    system.push_auto_spring(pivot, b);
    system.push_auto_spring(pivot, d);
    system.push_sized_spring(b, d, 55.8);
    system.push_sized_spring(b, j, 50.0);
    system.push_sized_spring(pivot, c, 39.3);
    system.push_sized_spring(d, f, 39.4);
    system.push_sized_spring(f, c, 36.7);
    system.push_sized_spring(c, i, 49.0);
    system.push_sized_spring(f, i, 65.7);
    system.push_sized_spring(j, c, 61.9);

    system
}
