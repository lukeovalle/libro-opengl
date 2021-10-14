use crate::render_gl::{self, data, buffer};
use crate::resources::Resources;
use std::ffi::CString;
use std::ops::{Add, Div};
//use nalgebra::Vector3;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    pos: data::f32_f32
}

impl Vertex {
    fn vertex_attrib_pointer(gl: &gl::Gl, program: &render_gl::Program) -> Result<(), anyhow::Error> {
        let loc = unsafe {
            gl.GetAttribLocation(
                program.id(),
                CString::new("vPosition")?.as_ptr()) };
        if loc == -1 {
            return Err(anyhow::anyhow!("Error con el attrib location vPosition"))
        }

        unsafe {
            gl.EnableVertexAttribArray(loc as u32);
            data::f32_f32::vertex_attrib_pointer(gl, 0, loc as usize, 0);
        };

        Ok(())
    }
}

impl From<(f32, f32)> for Vertex {
    fn from(other: (f32, f32)) -> Self {
        Vertex { pos: other.into() }
    }
}

impl Add for Vertex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            pos: (self.pos.d0 + other.pos.d0,
                  self.pos.d1 + other.pos.d1
                 ).into()
        }
    }
}

impl Div<f32> for Vertex {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            pos: (self.pos.d0 / rhs,
                  self.pos.d1 / rhs
                 ).into()
        }
    }
}

pub struct Sierpinski {
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
    vertices: Vec<Vertex>
}

impl Sierpinski {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Sierpinski, anyhow::Error> {
        let program = render_gl::Program::from_res(gl, res, "shaders/sierpinski")?;

        let vertices = generate_points(50000);

        let vao = buffer::VertexArray::new(gl);
        vao.bind();

        let vbo = buffer::ArrayBuffer::new(gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);

        Vertex::vertex_attrib_pointer(gl, &program)?;

        vbo.unbind();
        vao.unbind();

        Ok(Sierpinski {
            program,
            _vbo: vbo,
            vao,
            vertices
        })
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();

        unsafe {
            gl.DrawArrays(
                gl::POINTS,
                0,
                self.vertices.len() as i32
            );
        }
    }
}


fn generate_points(number_points: usize) -> Vec<Vertex> {
    let mut points: Vec<Vertex> = Vec::with_capacity(number_points);

    let vertices = vec![
        Vertex { pos: (-1.0, -1.0).into() },
        Vertex { pos: (0.0, 1.0).into() },
        Vertex { pos: (1.0, -1.0).into() }
    ];

    points.push((0.25, 0.5).into());

    for i in 1..number_points {
        let j = rand::random::<usize>() % 3;

        points.push((points[i-1] + vertices[j]) / 2.0);
    }

    points
}

