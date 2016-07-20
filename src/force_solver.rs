use dynamics::Solver;
use dynamics::System;
use glm::*;

/* Simple, force-based approach on solving the constraints.
 * i.e.  calculate force from springs,
         change velocity based on force,
         change position based on velocity
 */

pub struct ForceSolver {
    steps: usize
}

impl ForceSolver {
    pub fn new(steps: usize) -> ForceSolver {
        ForceSolver {
            steps: steps
        }
    }
}

impl Solver for ForceSolver {
    fn solve(&self, system: &mut System, ms: f32) {
        let dt = ms / self.steps as f32;
        let friction = 0.05f32.powf(dt);
        for _ in 0..self.steps {
            for ref mut spring in &mut system.springs {
                let p1 = system.masses[spring.ends.0].pos;
                let p2 = system.masses[spring.ends.1].pos;
                let delta = p1 - p2;
                let dist = length(delta);
                let force = delta / dist * (spring.length - dist) * 10000.0;

                {
                    let ref mut m = system.masses[spring.ends.0];
                    m.vel = m.vel + (force * dt) * m.imass;
                } {
                    let ref mut m = system.masses[spring.ends.1];
                    m.vel = m.vel - (force * dt) * m.imass;
                }
            }
            for ref mut mass in &mut system.masses {
                mass.pos = mass.pos + mass.vel * dt;
                mass.vel = mass.vel * friction;
            }
        }
    }
}
