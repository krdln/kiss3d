//! A batched line renderer.

use gl;
use gl::types::*;
use na::{Pnt3, Mat4};
use resource::{BufferType, AllocationType, GPUVector, Shader, ShaderAttribute, ShaderUniform};
use camera::Camera;

#[path = "error.rs"]
mod error;

/// Structure which manages the display of short-living lines.
pub struct LineRenderer {
    shader:    Shader,
    pos:       ShaderAttribute<Pnt3<f32>>,
    color:     ShaderAttribute<Pnt3<f32>>,
    view:      ShaderUniform<Mat4<f32>>,
    lines:     GPUVector<Pnt3<GLfloat>>
}

impl LineRenderer {
    /// Creates a new lines manager.
    pub fn new() -> LineRenderer {
        let mut shader = Shader::new_from_str(LINES_VERTEX_SRC, LINES_FRAGMENT_SRC);

        shader.use_program();

        LineRenderer {
            lines:     GPUVector::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
            pos:       shader.get_attrib::<Pnt3<f32>>("position").unwrap(),
            color:     shader.get_attrib::<Pnt3<f32>>("color").unwrap(),
            view:      shader.get_uniform::<Mat4<f32>>("view").unwrap(),
            shader:    shader
        }
    }
 
    /// Indicates whether some lines have to be drawn.
    pub fn needs_rendering(&self) -> bool {
        self.lines.len() != 0
    }

    /// Adds a line to be drawn during the next frame. Lines are not persistent between frames.
    /// This method must be called for each line to draw, and at each update loop iteration.
    pub fn draw_line(&mut self, a: Pnt3<GLfloat>, b: Pnt3<GLfloat>, color: Pnt3<GLfloat>) {
        for lines in self.lines.data_mut().iter_mut() {
            lines.push(a);
            lines.push(color);
            lines.push(b);
            lines.push(color);
        }
    }

    /// Actually draws the lines.
    pub fn render(&mut self, pass: usize, camera: &mut Camera) {
        if self.lines.len() == 0 { return }

        self.shader.use_program();
        self.pos.enable();
        self.color.enable();

        camera.upload(pass, &mut self.view);

        self.color.bind_sub_buffer(&mut self.lines, 1, 1);
        self.pos.bind_sub_buffer(&mut self.lines, 1, 0);

        verify!(gl::DrawArrays(gl::LINES, 0, (self.lines.len() / 2) as i32));

        self.pos.disable();
        self.color.disable();

        for lines in self.lines.data_mut().iter_mut() {
            lines.clear()
        }
    }
}

/// Vertex shader used by the material to display line.
pub static LINES_VERTEX_SRC:   &'static str = A_VERY_LONG_STRING;
/// Fragment shader used by the material to display line.
pub static LINES_FRAGMENT_SRC: &'static str = ANOTHER_VERY_LONG_STRING;

const A_VERY_LONG_STRING: &'static str =
   "#version 120
    attribute vec3 position;
    attribute vec3 color;
    varying   vec3 Color;
    uniform   mat4   view;
    void main() {
        gl_Position = view * vec4(position, 1.0);
        Color = color;
    }";

const ANOTHER_VERY_LONG_STRING: &'static str =
   "#version 120
    varying vec3 Color;
    void main() {
        gl_FragColor = vec4(Color, 1.0);
    }";
