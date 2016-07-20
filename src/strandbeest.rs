use dynamics::System;
use dynamics::Mass;
use dynamics::Solver;
use force_solver::ForceSolver;

use glm::vec2;
use glm::Vec2;

pub struct Strandbeest {
    pub system: System,
    rotor_inner: usize,
    rotor_outer: usize,
    angle: f32
}

impl Strandbeest {
    pub fn step(&mut self, delta_t: f32) {
        {
            self.angle += delta_t;
            let center = self.system.masses[self.rotor_inner].pos;
            let target = vec2(self.angle.cos(), self.angle.sin()) * 15.0 + center;
            self.system.masses[self.rotor_outer].pos = target;
        }
        {
            let solver = ForceSolver::new(100);
            solver.solve(&mut self.system, delta_t);
        }
    }

    pub fn new(around: &Vec2) -> Strandbeest {
        let mut system = System::new();

        let p = around.clone();

        // Positions just eye-balled use the springs to snap them to the right places
        let pivot = system.push_mass(Mass::pinned(vec2( 0.0,  0.0) + p));
        let j     = system.push_mass(Mass::pinned(vec2(30.0, 41.5) + p));
        let l     = system.push_mass(Mass::pinned(vec2(38.0,  7.8) + p));
        let b     = system.push_mass(Mass::unit(vec2(  0.0,  41.5) + p));
        let d     = system.push_mass(Mass::unit(vec2(-40.1,   0.0) + p));
        let c     = system.push_mass(Mass::unit(vec2(  0.0, -39.0) + p));
        let f     = system.push_mass(Mass::unit(vec2(-60.1, -39.0) + p));
        let i     = system.push_mass(Mass::unit(vec2(  0.0, -79.0) + p));

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

        Strandbeest {
            system: system,
            rotor_inner: l,
            rotor_outer: j,
            angle: 0.0
        }
    }
}
