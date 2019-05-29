extern crate sdl2;
extern crate gl;

use std::os::raw::{c_int, c_void, c_uint};

pub mod render_gl;
pub mod resources;

use crate::resources::Resources;
use std::path::Path;

fn main() {
    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("Rengine", 600, 400)
        .opengl()
//        .resizable()
//        .borderless()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();

    let gl = gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_pump = sdl.event_pump().unwrap();

    unsafe {
        gl.Viewport(0, 0, 600, 400);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let shader_program = render_gl::Program::from_res(
        &gl, &res, "shaders/triangle"
    ).unwrap();

    shader_program.set_used();

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl.GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<f32>()) as isize, // size of data in bytes
            vertices.as_ptr() as *const c_void, // pointer to data
            gl::STATIC_DRAW
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0); // unbind buffer
    }

    let mut vao: c_uint = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl.BindVertexArray(vao);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.EnableVertexAttribArray(0);
        gl.VertexAttribPointer(
            0, // index of the vertex attribute
            3, // number of components per vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized
            (6 * std::mem::size_of::<f32>()) as c_int,
            std::ptr::null() // offset of the first component
        );
        gl.EnableVertexAttribArray(1);
        gl.VertexAttribPointer(
            1, // index of the vertex attribute
            3, // number of components per vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized
            (6 * std::mem::size_of::<f32>()) as c_int,
            (3 * std::mem::size_of::<f32>()) as *const c_void // offset of the first component
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }

    'main: loop {
        for event in event_pump.poll_iter() {
            // handle user input here
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.set_used();
        unsafe {
            gl.BindVertexArray(vao);
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index
                3 // number of indices to be rendered
            )
        }

        window.gl_swap_window();
    }
}