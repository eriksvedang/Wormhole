
#![feature(globs)]

extern crate shader_version;
extern crate input;
extern crate ai_behavior;
extern crate sprite;
extern crate event;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::cell::RefCell;
use std::rc::Rc;

use event::{ Events, WindowSettings };
use sprite::*;
use ai_behavior::{
    Action,
    Sequence,
    Wait,
    WaitForever,
    While,
};

use graphics::*;

use sdl2_window::Sdl2Window;
use opengl_graphics::{
    Gl,
    Texture,
};

fn main() {
    let (mut width, mut height) = (940, 280);
    let opengl = shader_version::opengl::OpenGL_3_2;
    let window = Sdl2Window::new(
        opengl,
        WindowSettings {
            title: "Wormhole".to_string(),
            size: [width, height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let mut scene = Scene::new();
    let tex = Path::new("./rust-logo.png");
    let tex = Rc::new(Texture::from_path(&tex).unwrap());
    let mut sprite = Sprite::from_texture(tex.clone());
    let mut sprite2 = Sprite::from_texture(tex.clone());
    sprite.set_position(width as f64 / 2.0, height as f64 / 2.0);
    sprite2.set_position(0.0, 0.0);
    
    let id = scene.add_child(sprite);
    let id2 = scene.add_child(sprite2);

    let ref mut gl = Gl::new(opengl);
    let window = RefCell::new(window);
    
    for e in Events::new(&window) {
        use event::{ PressEvent, RenderEvent, ResizeEvent };

        scene.event(&e);

        e.resize(|w,h| {
            width = w;
            height = h;
            println!("new w = {}, new h = {}", w, h);
        });

        e.render(|args| {
            use graphics::*;

            gl.viewport(0, 0, args.width as i32, args.height as i32);

            let c = Context::abs(args.width as f64, args.height as f64);
            c.rgb(1.0, 0.9, 1.0).draw(gl);

            scene.draw(&c, gl);
        });
        
        e.press(|key| {
            println!("Key = {}", key);
        });
    }
}
