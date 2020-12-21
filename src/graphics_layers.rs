extern crate image;

use glium::glutin;
use glium::Surface;
use glium::texture::Texture2d;
use glium::backend::glutin::glutin::event::Event;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use std::rc::Rc;
use self::image::image_dimensions;
use glium::texture::RawImage2d;
use std::collections::HashMap;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

#[derive(Copy, Clone)]
struct Rectangle {
    corners: [f32; 4],
    texture_key: &'static str,
}

impl Rectangle{

    fn draw(self, frame: &mut glium::Frame, display: &glium::Display, program: &glium::Program, tex_cache: &HashMap<&str, Texture2d>){
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let v0 = Vertex{ position: [self.corners[0], self.corners[1]]};
        let v1 = Vertex{ position: [self.corners[2], self.corners[1]]};
        let v2 = Vertex{ position: [self.corners[2], self.corners[3]]};
        let v3 = Vertex{ position: [self.corners[0], self.corners[3]]};
        let shape = vec![v0, v1, v3];
        let vb = glium::VertexBuffer::new(display, &shape).unwrap();

        frame.draw(&vb, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
        let shape = vec![v2, v1, v3];
        let vb = glium::VertexBuffer::new(display, &shape).unwrap();
        frame.draw(&vb, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();

    }
}

implement_vertex!(Vertex, position);

fn keyboard_input(event: &glutin::event::WindowEvent<'_>, debounce: &mut bool) -> (f32, f32) {
    let input = match *event {
        glutin::event::WindowEvent::KeyboardInput { input, .. } => input,
        _ => return (0., 0.),
    };
    let pressed = input.state == glutin::event::ElementState::Pressed;
    if pressed && !*debounce{
        *debounce = true;
    }else if pressed && *debounce{
        return (0.0, 0.0);
    }else{
        *debounce = false;
        return (0.0, 0.0);
    }
    let key = match input.virtual_keycode {
        Some(key) => key,
        None => return (0.0, 0.0),
    };
    match key {
        glutin::event::VirtualKeyCode::Up => (0.0,-0.1),
        glutin::event::VirtualKeyCode::Down => (0.0, 0.1),
        glutin::event::VirtualKeyCode::Left => (-0.1, 0.0),
        glutin::event::VirtualKeyCode::Right => (0.1, 0.0),
        glutin::event::VirtualKeyCode::Return => (1.0, 1.0),
        _ => (0.0, 0.0),
    }
}


pub fn launch_window(){
    let events_loop = glutin::event_loop::EventLoop::new();

    let wb = glutin::window::WindowBuilder::new().with_inner_size(
        glium::glutin::dpi::LogicalSize::new(1024.0, 768.0)
    ).with_title("DSS - Chilton").with_resizable(false);

    let cb = glutin::ContextBuilder::new();

    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let image = image::open("derp2.png").unwrap().to_rgb8();
    let dim = image.dimensions();
    let img = glium::texture::RawImage2d::from_raw_rgb_reversed(&image.into_raw(), dim);
    let tex: glium::texture::Texture2d = glium::texture::Texture2d::new(&display, img).unwrap();
    let rect = Rectangle{ corners: [-0.5, -0.5, 0.5, 0.5], texture_key: "asdf" };
    let mut hm = HashMap::new();
    hm.insert("asdf", tex);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;


    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut debounce = false;

    // add listen handler for window close request
    events_loop.run(move |ev, _, control_flow|{
        let mut target = display.draw();
        target.clear_color(0.01, 0.01, 0.01, 1.0);
        // target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
        // for r in &mut rv{
        rect.draw(&mut target, &display, &program, &hm);
        // }
        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::KeyboardInput { .. } => {
                    let input = keyboard_input(&event, &mut debounce);
                    if input != (0.,0.) {
                        println!("{} {}", input.0, input.1);
                    }
                }

                _ => return,
            },
            _ => (),
        }
    });

}