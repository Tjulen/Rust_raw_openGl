extern crate gl;
extern crate glutin;

use glutin::GlContext;
use std::ffi::CString;
use std::io::prelude::*;

#[derive(PartialEq, Debug)]
struct Shader {
    vertex_shader_string: String,
    fragment_shader_string: String,
    vertices: [f32; 9],
}

impl Shader {
    fn new(vertex_source: String, fragment_source: String, vertices_vector: [f32; 9]) -> Shader {
        Shader {
            vertex_shader_string: Shader::load_shader_from_source(vertex_source),
            fragment_shader_string: Shader::load_shader_from_source(fragment_source),
            vertices: vertices_vector,
        }
    }
    fn load_shader_from_source(shader_source: String) -> String {
        let mut file = std::fs::File::open(shader_source)
            .expect("ERROR: Something went wrong with opening the file!");
        let mut file_context = String::new();

        file.read_to_string(&mut file_context)
            .expect("ERROR: Something went wrong with reading a file!");
        file_context
    }
    fn create_program(&self) -> gl::types::GLuint {
        //Build and compile shader based on its type
        unsafe {
            //vertex shader
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vertex = CString::new(self.vertex_shader_string.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vertex.as_ptr(), std::ptr::null());
            gl::CompileShader(vertex_shader);

            //Check for shader compile errors
            let mut shader_compile_success = gl::FALSE as gl::types::GLint;
            let mut info_log = Vec::with_capacity(512);

            info_log.set_len(512 - 1);
            gl::GetShaderiv(
                vertex_shader,
                gl::COMPILE_STATUS,
                &mut shader_compile_success,
            );
            if shader_compile_success != gl::TRUE as gl::types::GLint {
                gl::GetShaderInfoLog(
                    vertex_shader,
                    512,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut gl::types::GLchar,
                );

                println!(
                    "ERROR: vertex shader compiled unsuccessfully, log: {}",
                    std::str::from_utf8(&info_log).unwrap()
                );
            }

            //fragment shader
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_fragment = CString::new(self.fragment_shader_string.as_bytes()).unwrap();
            gl::ShaderSource(
                fragment_shader,
                1,
                &c_str_fragment.as_ptr(),
                std::ptr::null(),
            );
            gl::CompileShader(fragment_shader);

            //check for errors
            gl::GetShaderiv(
                fragment_shader,
                gl::COMPILE_STATUS,
                &mut shader_compile_success,
            );
            if shader_compile_success != gl::TRUE as gl::types::GLint {
                gl::GetShaderInfoLog(
                    fragment_shader,
                    512,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut gl::types::GLchar,
                );
                println!(
                    "ERROR: fragment shader compiled unsuccessfully, log: {}",
                    std::str::from_utf8(&info_log).unwrap()
                );
            }

            //link shader together into one program
            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            //check for linking errors
            let mut program_compile_success = gl::FALSE as gl::types::GLint;
            gl::GetProgramiv(
                shader_program,
                gl::LINK_STATUS,
                &mut program_compile_success,
            );
            if program_compile_success != gl::TRUE as gl::types::GLint {
                gl::GetProgramInfoLog(
                    shader_program,
                    512,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut gl::types::GLchar,
                );
                println!(
                    "ERROR: program compilation failed, log: {}",
                    std::str::from_utf8(&info_log).unwrap()
                );
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            return shader_program as gl::types::GLuint;
        }
    }
    fn create_vao(&self) -> gl::types::GLuint {
        unsafe {
            let mut buffer = 0;

            gl::GenBuffers(1, &mut buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * std::mem::size_of::<gl::types::GLuint>())
                    as gl::types::GLsizeiptr,
                &self.vertices as *const f32 as *const std::os::raw::c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            return buffer;

            // let mut vbo = 0;
            // let mut vao = 0;

            // gl::GenVertexArrays(1, &mut vao);
            // gl::GenBuffers(1, &mut vbo);

            // gl::BindVertexArray(vao);

            // gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            // gl::BufferData(
            //     gl::ARRAY_BUFFER,
            //     (vertices.len() * std::mem::size_of::<gl::types::GLfloat>())
            //         as gl::types::GLsizeiptr,
            //     &vertices[0] as *const f32 as *const std::os::raw::c_void,
            //     gl::STATIC_DRAW,
            // );

            // gl::VertexAttribPointer(
            //     0,
            //     3,
            //     gl::FLOAT,
            //     gl::FALSE,
            //     3 * std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
            //     std::ptr::null(),
            // );

            // gl::EnableVertexAttribArray(0);

            // gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            // gl::BindVertexArray(0);

            // return vao as gl::types::GLuint;
        }
    }
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world")
        .with_dimensions(1200, 700);
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }
    unsafe {
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    }

    let vert: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
    let shad = Shader::new(
        Shader::load_shader_from_source(String::from(r#"D:\Projects\Rust\open_gl_learning\src\shaders\vertex.glsl"#)),
        Shader::load_shader_from_source(String::from(r#"D:\Projects\Rust\open_gl_learning\src\shaders\fragment.glsl"#)),
        vert,
    );

    let mut running = true;
    while running {
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => running = false,
                glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                _ => (),
            },
            _ => (),
        });

        unsafe {
            gl::ClearColor(0.6, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            //draw our first triangle
            gl::UseProgram(shad.create_program());
            shad.create_vao();
            //gl::BindVertexArray(shad.create_vao());
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        gl_window.swap_buffers().unwrap();
    }
}
