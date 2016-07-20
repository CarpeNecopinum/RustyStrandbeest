use glm;
use glium::*;
use glium::index::PrimitiveType;
use glium::index::NoIndices;
use glium::backend::Facade;

#[derive(Copy, Clone)]
struct Vert {
    aPosition: [f32; 2]
}
implement_vertex!(Vert, aPosition);

impl Vert {
    pub fn glm(source: &glm::Vec2) -> Vert {
        let p = source.clone();
        Vert {
            aPosition: [p.x, p.y]
        }
    }
}

const VERT_SHADER: &'static str = r#"
    #version 130
    uniform mat4 projection;

    in vec2 aPosition;
    void main() {
        gl_Position = projection * vec4(aPosition, 0.0, 1.0);
    }
"#;

const FRAG_SHADER: &'static str = r#"
    #version 130

    out vec3 fColor;
    void main() {
        fColor = vec3(1,0,1);
    }
"#;

pub struct Renderer {
    spring_indices: IndexBuffer<u16>,
    masses_buffer: VertexBuffer<Vert>,
    program: Program
}

use dynamics;

impl Renderer {

    pub fn new<F>(f: &F) -> Renderer where F: Facade {
        Renderer {
            spring_indices: IndexBuffer::empty_dynamic(f, PrimitiveType::LinesList, 1).unwrap(),
            masses_buffer: VertexBuffer::empty_dynamic(f, 1).unwrap(),
            program: Program::from_source(f, VERT_SHADER, FRAG_SHADER, None).unwrap()
        }
    }

    pub fn update<F>(&mut self, f: &F, model: &dynamics::System) where F: Facade {
        let masses = model.masses.iter().map(|mass| Vert::glm(&mass.pos)).collect::<Vec<Vert>>();
        self.masses_buffer = VertexBuffer::new(f, &masses).unwrap();

        let mut indices = Vec::new();
        for spring in &model.springs {
            indices.push(spring.ends.0 as u16);
            indices.push(spring.ends.1 as u16);
        }
        self.spring_indices = IndexBuffer::new(f, PrimitiveType::LinesList, &indices).unwrap();
    }

    pub fn render(&self, frame: &mut Frame) {
        let just_points = NoIndices(PrimitiveType::Points);
        let params = DrawParameters {
            point_size: Some(10.0),
            .. Default::default()
        };

        let (x,y) = frame.get_dimensions();
        let aspect = x as f32 / y as f32;
        let uniforms = uniform!{
            projection: [
                [0.01 / aspect, 0.0          , 0.0, 0.0],
                [ 0.0, 0.01, 0.0, 0.0],
                [ 0.0, 0.0          , 1.0, 0.0],
                [ 0.0, 0.0          , 0.0, 1.0]
            ]
        };

        frame.draw(
            &self.masses_buffer, &self.spring_indices,
            &self.program,
            &uniforms,
            &params
        ).unwrap();

        frame.draw(
            &self.masses_buffer, &just_points,
            &self.program,
            &uniforms,
            &params
        ).unwrap();
    }
}
