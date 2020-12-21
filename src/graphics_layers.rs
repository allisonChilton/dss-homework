extern crate image;
extern crate cgmath;
use crate::data_loader;

use cgmath::{Matrix4, Vector2};
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
use data_loader::TitleContainer;
use std::borrow::BorrowMut;


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

#[derive(Clone)]
pub(crate) struct Rectangle {
    pub(crate) center: [f32; 2],
    pub(crate) size: [f32; 2],
    pub(crate) texture_key: &'static str,
}

impl Rectangle{

    fn draw(&self, frame: &mut glium::Frame, indices: &glium::IndexBuffer<u16>, display: &glium::Display,
            program: &glium::Program, tex_cache: &HashMap<&str, Texture2d>, perspective: [[f32; 4]; 4]){

        let vb = glium::VertexBuffer::empty_dynamic(display, 4).unwrap();
        let rect_size = Vector2 {
            x: self.size[0],
            y: self.size[1],
        };

        let mut rect_position = Vector2 {
            x: self.center[0],
            y: self.center[1],
        };

        let left = rect_position.x - rect_size.x / 2.0;
        let right = rect_position.x + rect_size.x / 2.0;
        let top = rect_position.y + rect_size.y / 2.0;
        let bottom = rect_position.y - rect_size.y / 2.0;


        let v0 = Vertex{ position: [left, top]};
        let v1 = Vertex{ position: [right, top]};
        let v2 = Vertex{ position: [left, bottom]};
        let v3 = Vertex{ position: [right, bottom]};
        let shape = vec![v0, v1, v2, v3];
        vb.write(&shape);

        let texture = tex_cache.get(self.texture_key).unwrap();

        let uniforms = uniform!{
            projection: perspective,
            tex: texture
        };
        frame.draw(&vb, indices, program, &uniforms, &Default::default()).unwrap();

    }
}

implement_vertex!(Vertex, position);

fn keyboard_input(event: &glutin::event::WindowEvent<'_>, debounce: &mut bool) -> (i32, i32) {
    let input = match *event {
        glutin::event::WindowEvent::KeyboardInput { input, .. } => input,
        _ => return (0, 0),
    };
    let pressed = input.state == glutin::event::ElementState::Pressed;
    if pressed && !*debounce{
        *debounce = true;
    }else if pressed && *debounce{
        return (0, 0);
    }else{
        *debounce = false;
        return (0, 0);
    }
    let key = match input.virtual_keycode {
        Some(key) => key,
        None => return (0, 0),
    };
    match key {
        glutin::event::VirtualKeyCode::Up => (0,-1),
        glutin::event::VirtualKeyCode::Down => (0, 1),
        glutin::event::VirtualKeyCode::Left => (-1, 0),
        glutin::event::VirtualKeyCode::Right => (1, 0),
        glutin::event::VirtualKeyCode::Return => (1, 1),
        _ => (0, 0),
    }
}

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;
const num_rows: u32 = 4;
const num_cols: u32 = 6;
const padding: u32 = 16;
const center_dist_horz: u32 = SCREEN_WIDTH / num_cols;
const center_dist_vert: u32 = SCREEN_HEIGHT / num_rows;
const tile_size: f32 = (SCREEN_WIDTH / num_cols - (padding * 2)) as f32;


const VERTEX_SHADER_SRC: &str = r#"
     #version 140
    // Input parameter from the Vertex struct.
    in vec2 position;
    // Uniform parameter passed in from the frame.draw() call.
    uniform mat4 projection;
    // Output texture coordinates that gets passed into the fragment shader.
    out vec2 v_tex_coords;
    void main() {
        // In order to return the texture coordinate for a specific
        // vertex we have to know what vertex is currently being passed in.
        // We do this through gl_VertexID which increments with every vertex passed in.
        // We can figure out the rectangle specific index from the vertex id by modding it
        // by 4. Example: if a vertex has id 16, then it is the first vertex of the fourth
        // rectangle being drawn. 16 % 4 == 0 which correctly returns the first index.
        if (gl_VertexID % 4 == 0) { // First vertex
            v_tex_coords = vec2(0.0, 1.0);
        } else if (gl_VertexID % 4 == 1) { // Second vertex
            v_tex_coords = vec2(1.0, 1.0);
        } else if (gl_VertexID % 4 == 2) { // Third vertex
            v_tex_coords = vec2(0.0, 0.0);
        } else { // Fourth vertex
            v_tex_coords = vec2(1.0, 0.0);
        }
        gl_Position = projection * vec4(position, 0.0, 1.0);
    }
    "#;

const FRAGMENT_SHADER_SRC: &str = r#"
        #version 140
    // Input texture coordinates passed from the vertex shader.
    in vec2 v_tex_coords;
    // Outputs the color for the specific fragment.
    out vec4 color;
    // Uniform parameter passed in from the frame.draw() call.
    uniform sampler2D tex;
    void main() {
        // Applies a texture to the rectangle.
        color = texture(tex, v_tex_coords);
    }
    "#;

struct Menu{
    selected_tile: [i32; 2],
    tiles: Vec<Vec<Rectangle>>,
    texture_map: HashMap<&'static str, Texture2d>,
    selection_square: Rectangle
}

impl Menu{
    fn new()->Menu{
        let mut active_rows: Vec<Vec<Rectangle>> = Vec::new();
        for row_num in 1..num_rows{
            let mut active_cols: Vec<Rectangle> = Vec::new();
            for col_num in 1..num_cols{
                active_cols.push(Rectangle{
                    center: [(col_num * center_dist_horz) as f32, (row_num * center_dist_vert) as f32],
                    size: [tile_size, tile_size],
                    texture_key: "asdf"
                })
            }
            active_rows.push(active_cols);
        }

        let hm = HashMap::new();
        let ctile = active_rows.get(0).unwrap().get(0).unwrap().center.clone();
        Menu{ selected_tile: [0; 2], tiles: active_rows, texture_map: hm, selection_square: Rectangle {
            center: ctile,
            size: [Menu::selection_backdrop_size(); 2],
            texture_key: "whitesquare"
        } }
    }

    fn add_texture(&mut self, path: &str, key: &'static str, display: &glium::Display){
        let image = image::open(path).unwrap().to_rgb8();
        let dim = image.dimensions();
        let img = glium::texture::RawImage2d::from_raw_rgb(image.into_raw(), dim);
        let tex: glium::texture::Texture2d = glium::texture::Texture2d::new(display, img).unwrap();
        self.texture_map.insert(key, tex);
    }

    fn change_selected_tile(&mut self, change: (i32, i32)){
        let (x, y) = change;
        let (lx, ly) = (self.selected_tile[0] as usize, self.selected_tile[1] as usize);
        match self{
            Menu{ref mut tiles, ..} => match tiles[lx][ly]{
                Rectangle{ref mut size, .. } => {
                    size[0] = tile_size;
                    size[1] = tile_size;
                }
            }
        }

        self.selected_tile[0] += y;
        self.selected_tile[1] += x;
        self.selected_tile[0] = self.selected_tile[0].max(0).min(num_rows as i32 - 2);
        self.selected_tile[1] = self.selected_tile[1].max(0).min(num_cols as i32 - 2);
        println!("{} {}",self.selected_tile[0], self.selected_tile[1]);
        let (nx, ny) = (self.selected_tile[0] as usize, self.selected_tile[1] as usize);
        let sel_center = self.selection_square.borrow_mut().center.as_mut();
        match self{
            Menu{ref mut tiles, ..} => match tiles[nx][ny]{
                Rectangle{ref mut size, center, .. } => {
                    size[0] = Menu::selected_tile_size();
                    size[1] = Menu::selected_tile_size();
                    sel_center[0] = center[0];
                    sel_center[1] = center[1];
                }
            }
        }
    }

    fn selection_backdrop_size()->f32{
        return ((tile_size as u32) + (padding * 2) + 8) as f32;
    }

    fn selected_tile_size()->f32{
        return (tile_size as u32 + (padding * 2)) as f32;
    }

    fn draw(&mut self, frame: &mut glium::Frame, indices: &glium::IndexBuffer<u16>, display: &glium::Display,
            program: &glium::Program, perspective: [[f32; 4]; 4]) {

        let tile = self.selection_square.borrow_mut();
        tile.draw(frame, indices, display, program, &self.texture_map, perspective);
        for row in &self.tiles{
            for tile in row {
                tile.draw(frame, indices, display, program, &self.texture_map, perspective);
            }
        }

    }
}

pub fn launch_window(title_data: Vec<TitleContainer>){
    let events_loop = glutin::event_loop::EventLoop::new();

    let wb = glutin::window::WindowBuilder::new().with_inner_size(
        glium::glutin::dpi::LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    ).with_title("DSS - Chilton").with_resizable(false);

    let cb = glutin::ContextBuilder::new();

    let display = glium::Display::new(wb, cb, &events_loop).unwrap();




    let program = glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();


    let perspective = {
        let matrix: Matrix4<f32> = cgmath::ortho(
            0.0,
            SCREEN_WIDTH as f32,
            SCREEN_HEIGHT as f32,
            0.0,
            -1.0,
            1.0
        );
        Into::<[[f32; 4]; 4]>::into(matrix)
    };

    // 0      1
    // +------+
    // |    / |
    // |  /   |
    // |/     |
    // +------+
    // 2      3
    let ib_data: Vec<u16> = vec![0, 1, 2, 1, 3, 2];
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &ib_data
    ).unwrap();

    let mut debounce = false;


    let mut menu = Menu::new();
    menu.add_texture("derp2.png","asdf", &display);
    menu.add_texture("whitesquare.png","whitesquare", &display);

    // add listen handler for window close request
    events_loop.run(move |ev, _, control_flow|{
        let mut target = display.draw();
        target.clear_color(0.01, 0.01, 0.01, 1.0);
        menu.draw(&mut target, &indices, &display, &program,  perspective);
        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(166_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::KeyboardInput { .. } => {
                    let input = keyboard_input(&event, &mut debounce);
                    if input != (0,0) {
                        println!("{} {}", input.0, input.1);
                        menu.change_selected_tile(input);
                    }
                }

                _ => return,
            },
            _ => (),
        }
    });

}