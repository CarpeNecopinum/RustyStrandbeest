use glm::*;

pub struct Mass {
    pub pos: Vec2,
    pub vel: Vec2,
    pub imass: f32
}

impl Mass {
    pub fn pinned(position: Vec2) -> Mass {
        Mass {
            pos: position,
            vel: vec2(0.0, 0.0),
            imass: 0.0
        }
    }

    pub fn unit(position: Vec2) -> Mass {
        Mass {
            pos: position,
            vel: vec2(0.0, 0.0),
            imass: 1.0
        }
    }
}

pub struct Spring {
    pub ends: (usize, usize),
    pub length: f32,
    pub strength: f32
}

impl Spring {
    pub fn unit(end_points: (usize, usize), length: f32) -> Spring {
        Spring {
            ends: end_points,
            length: length,
            strength: 1.0
        }
    }
}

pub struct System {
    pub masses: Vec<Mass>,
    pub springs: Vec<Spring>
}

impl System {
    pub fn new() -> System {
        System {
            masses: Vec::new(),
            springs: Vec::new()
        }
    }

    pub fn push_mass(&mut self, mass: Mass) -> usize {
        self.masses.push(mass);
        self.masses.len() - 1
    }

    pub fn push_auto_spring(&mut self, from: usize, to: usize) -> usize {
        let ps = (&self.masses[from], &self.masses[to]);

        let spring = Spring::unit((from, to), distance(ps.0.pos, ps.1.pos));
        self.springs.push(spring);
        self.springs.len() - 1
    }

    pub fn push_sized_spring(&mut self, from: usize, to: usize, length: f32) -> usize {
        let spring = Spring::unit((from, to), length);
        self.springs.push(spring);
        self.springs.len() - 1
    }
}

pub trait Solver {
    fn solve(&self, system: &mut System, ms: f32);
}
