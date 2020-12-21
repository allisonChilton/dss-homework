extern crate image;
extern crate cgmath;
extern crate glium_text_rusttype as glium_text;
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
use crate::data_loader::Title;


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

    fn get_nw_corner(&self)->(f32, f32){
        let rect_size = Vector2 {
            x: self.size[0],
            y: self.size[1],
        };

        let mut rect_position = Vector2 {
            x: self.center[0],
            y: self.center[1],
        };

        let left = rect_position.x - rect_size.x / 2.0;
        let bottom = rect_position.y - rect_size.y / 2.0;
        (left, bottom)
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
const NUM_ROWS: u32 = 4;
const NUM_COLS: u32 = 6;
const PADDING: u32 = 16;
const CENTER_DIST_HORZ: u32 = SCREEN_WIDTH / NUM_COLS;
const CENTER_DIST_VERT: u32 = SCREEN_HEIGHT / NUM_ROWS;
const TILE_SIZE: f32 = (SCREEN_WIDTH / NUM_COLS - (PADDING * 2)) as f32;


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
    selection_square: Rectangle,
    headings: Vec<&'static str>,
    popup: Rectangle,
    popup_enabled: bool
}

impl Menu{
    fn new()->Menu{
        let mut active_rows: Vec<Vec<Rectangle>> = Vec::new();
        for row_num in 1..NUM_ROWS {
            let mut active_cols: Vec<Rectangle> = Vec::new();
            for col_num in 1..NUM_COLS {
                active_cols.push(Rectangle{
                    center: [(col_num * CENTER_DIST_HORZ) as f32, (row_num * CENTER_DIST_VERT) as f32],
                    size: [TILE_SIZE, TILE_SIZE],
                    texture_key: "asdf"
                })
            }
            active_rows.push(active_cols);
        }
        active_rows[0][0].size = [Menu::selected_tile_size(); 2];

        let hm = HashMap::new();
        let ctile = active_rows.get(0).unwrap().get(0).unwrap().center.clone();
        Menu{ selected_tile: [0; 2],
            tiles: active_rows,
            texture_map: hm,
            selection_square: Rectangle {
                center: ctile,
                size: [Menu::selection_backdrop_size(); 2],
                texture_key: "whitesquare",
            },
            headings: vec!["this is a test", "yet another test", "and again a test"],
            popup: Rectangle{
                center: [SCREEN_WIDTH as f32 / 2., SCREEN_HEIGHT as f32 / 2.],
                size: [SCREEN_WIDTH as f32 * 0.5, SCREEN_HEIGHT as f32 * 0.9],
                texture_key: "popup"
            },
            popup_enabled: false
        }
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
        if x == 1 && y == 1{
            self.popup_enabled = true;
            return;
        }else{
            self.popup_enabled = false;
        }
        let (lx, ly) = (self.selected_tile[0] as usize, self.selected_tile[1] as usize);
        match self{
            Menu{ref mut tiles, ..} => match tiles[lx][ly]{
                Rectangle{ref mut size, .. } => {
                    size[0] = TILE_SIZE;
                    size[1] = TILE_SIZE;
                }
            }
        }

        self.selected_tile[0] += y;
        self.selected_tile[1] += x;
        self.selected_tile[0] = self.selected_tile[0].max(0).min(NUM_ROWS as i32 - 2);
        self.selected_tile[1] = self.selected_tile[1].max(0).min(NUM_COLS as i32 - 2);
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
        return ((TILE_SIZE as f32) + (PADDING as f32 * 1.25) + 4.) as f32;
    }

    fn selected_tile_size()->f32{
        return (TILE_SIZE as f32 + (PADDING as f32 * 1.25)) as f32;
    }

    fn x_y_to_mat(x: f32, y: f32) -> (f32, f32) {
        let xrise = 1. - (-1.);
        let xrun = (SCREEN_WIDTH as f32);
        let xslope = xrise / xrun;
        let xb = 1. - (SCREEN_WIDTH as f32 * xslope);
        let retx = xslope * x + xb;

        let yrise = 1. - (-1.);
        let yrun = (SCREEN_HEIGHT as f32);
        let yslope = yrise / yrun;
        let yb = 1. - (SCREEN_HEIGHT as f32 * yslope);
        let rety = -(yslope * y + yb);
        (retx , rety)
    }

    fn write_popup(&self, title: &Title, frame: &mut glium::Frame, text_system: &glium_text::TextSystem, font: &glium_text::FontTexture){
        let tstr = title.to_string();
        let strings = tstr.lines();
        let (xoffset, yoffset) = self.popup.get_nw_corner();

        for (idx, s) in strings.enumerate(){
            let text = glium_text::TextDisplay::new(text_system, font, s);
            let (x, y) = Menu::x_y_to_mat(xoffset + 32., yoffset + 64. + (32. * idx as f32));
            const tsize: f32 = 0.05;
            let loc_mat: [[f32; 4]; 4] = cgmath::Matrix4::new(
                tsize, 0.0, 0.0, 0.0,
                0.0, tsize, 0.0, 0.0,
                0.0, 0.0, tsize, 0.0,
                x, y, 0.0, 1.0f32,
            ).into();
            glium_text::draw(&text, text_system, frame, loc_mat, (1.0, 1.0, 1.0, 1.0)).unwrap();
        }
    }

    fn draw(&mut self, frame: &mut glium::Frame, indices: &glium::IndexBuffer<u16>, display: &glium::Display,
            program: &glium::Program, perspective: [[f32; 4]; 4], text_system: &glium_text::TextSystem, font: &glium_text::FontTexture) {

        if self.headings.len() >= 3{
            for (idx, heading) in self.headings.iter().enumerate(){
                let text = glium_text::TextDisplay::new(text_system, font, heading);
                let yoffset = self.tiles[idx][0].center[1] - TILE_SIZE as f32 / 2. - (PADDING as f32);
                let xoffset = self.tiles[idx][0].center[0] - TILE_SIZE as f32 / 2.;

                let (x, y) = Menu::x_y_to_mat(xoffset, yoffset);
                const tsize: f32 = 0.05;
                let loc_mat: [[f32; 4]; 4] = cgmath::Matrix4::new(
                    tsize, 0.0, 0.0, 0.0,
                    0.0, tsize, 0.0, 0.0,
                    0.0, 0.0, tsize, 0.0,
                    x, y, 0.0, 1.0f32,
                ).into();

                glium_text::draw(&text, text_system, frame, loc_mat, (1.0, 1.0, 1.0, 1.0)).unwrap();
            }
        }

        let tile = self.selection_square.borrow_mut();
        tile.draw(frame, indices, display, program, &self.texture_map, perspective);
        for row in &self.tiles{
            for tile in row {
                tile.draw(frame, indices, display, program, &self.texture_map, perspective);
            }
        }

        if self.popup_enabled{
            self.popup.draw(frame, indices, display, program, &self.texture_map, perspective);
            let title = Title{
                id: "asdf".to_string(),
                name: "Hello World".to_string(),
                release_date: "1984".to_string(),
                rating: "PG-13".to_string(),
                image_url: "".to_string()
            };
            self.write_popup(&title, frame, text_system, font);
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

    let text_system = glium_text::TextSystem::new(&display);

    let font = glium_text::FontTexture::new(&display, &include_bytes!("OpenSans-Bold.ttf")[..], 70, glium_text::FontTexture::ascii_character_list()).unwrap();



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
    menu.add_texture("popup.png","popup", &display);

    let mut redraw = true;

    // add listen handler for window close request
    events_loop.run(move |ev, _, control_flow|{
        if redraw {
            let mut target = display.draw();
            target.clear_color(0.01, 0.01, 0.01, 1.0);
            menu.draw(&mut target, &indices, &display, &program, perspective, &text_system, &font);
            target.finish().unwrap();
            redraw = false;
        }

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
                    redraw = true;
                }

                _ => return,
            },
            _ => (),
        }
    });

}