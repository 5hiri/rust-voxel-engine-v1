use glium::{glutin::surface::WindowSurface, implement_vertex, uniform, Display, Frame, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
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

    implement_vertex!(Vertex, position);
    let triangle = vec![
        Vertex { position: [-0.5, 0.5, 0.0] }, // Top Left
        Vertex { position: [-0.5, -0.5, 0.0] }, // Bottom Left
        Vertex { position: [0.5, -0.5, 0.0] }, // Bottom Right
    ];
    let triangle2 = vec![
        Vertex { position: [-0.5, 0.5, 0.0] }, // Top Left
        Vertex { position: [0.5, 0.5, 0.0] }, // Top Right
        Vertex { position: [0.5, -0.5, 0.0] }, // Bottom Right
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
                    frame.clear_color(0.0, 0.0, 0.0, 1.0);
                    // Draw triangle
                    draw_triangle(triangle.clone(), &display, &mut frame, &x_off);
                    draw_triangle(triangle2.clone(), &display, &mut frame, &x_off);
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

fn draw_triangle(triangle:Vec<Vertex>, display:&Display<WindowSurface>, frame:&mut Frame, x_off:&f32) {
    let vertex_shader_src = r#"
        #version 140

        in vec3 position;

        uniform float x;

        void main() {
            vec3 pos = position;
            pos.x += x;
            gl_Position = vec4(pos, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let vertex_buffer = glium::VertexBuffer::new(display, &triangle).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

    frame.draw(&vertex_buffer, &indices, &program, &uniform! { x: x_off.clone() },
        &Default::default()).unwrap();
}