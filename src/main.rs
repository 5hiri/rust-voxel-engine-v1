use std::f32::consts::PI;

use glium::{glutin::surface::WindowSurface, implement_vertex, uniform, Display, Frame, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

fn main() {
    // We start by creating the EventLoop, this can only be done once per process.
    // This also needs to happen on the main thread to make the program portable.
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Voxel Engine V1")
        .build(&event_loop);

    implement_vertex!(Vertex, position, color);
    let triangle = vec![
        Vertex { position: [-0.5, 0.5, 0.0], color: [1.0, 0.0, 0.0] },// Top Left
        Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] }, // Bottom Left
        Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] }, // Bottom Right
    ];
    let triangle2 = vec![
        Vertex { position: [0.5, 0.5, 0.0], color: [0.0, 1.0, 0.0] }, // Top Right
        Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] }, // Bottom Right
        Vertex { position: [-0.5, 0.5, 0.0], color: [1.0, 0.0, 0.0] }, // Top Left
    ];

    // Now we wait until the program is closed
    let mut t: f32 = 0.0;
    #[allow(deprecated)]
    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                // This event is sent by the OS when you close the Window, or request the program to quit via the taskbar.
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                glium::winit::event::WindowEvent::RedrawRequested => {
                    // Start rendering by creating a new frame
                    // We update `t`
                        t += 0.02;
                        // We use the sine of t as an offset, this way we get a nice smooth animation
                    let x_off = t.sin() * 0.5;
                    let mut frame = display.draw();
                    // Which we fill with an opaque blue color
                    frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
                    // Draw triangle
                    draw_triangle(triangle.clone(), &display, &mut frame);
                    draw_triangle(triangle2.clone(), &display, &mut frame);
                    // By finishing the frame swap buffers and thereby make it visible on the window
                    frame.finish().unwrap();
                },
                _ => (),
            },
            glium::winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        };
    })
    .unwrap();
}

fn draw_triangle(triangle:Vec<Vertex>, display:&Display<WindowSurface>, frame:&mut Frame) {
    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec3 color;
        out vec3 vertex_color;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            mat4 modelview = view * model;
            vertex_color = color;
            gl_Position = perspective * modelview * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec3 vertex_color;
        out vec4 color;

        void main() {
            color = vec4(vertex_color, 1.0); // We need an alpha value as well
        }
    "#;

    let perspective = {
        let (width, height) = frame.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;

        let fov: f32 = PI / 3.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
    };
    let view = view_matrix(&[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0]);

    let uniforms = uniform! {
        view: view,
        perspective: perspective,
        model: [
            [0.5, 0.0, 0.0, 0.0],
            [0.0, 0.5, 0.0, 0.0],
            [0.0, 0.0, 0.5, 0.0],
            [0.0, 0.0, 2.0, 1.0f32]
        ]
    };

    let vertex_buffer = glium::VertexBuffer::new(display, &triangle).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullingDisabled,
        .. Default::default()
    };

    frame.draw(&vertex_buffer, &indices, &program, &uniforms,
        &params).unwrap();
}

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}