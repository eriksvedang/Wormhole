#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
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

struct Entity {
    pos: (f32, f32),
    speed: (f32, f32)
}

impl Entity {
    fn new(x: f32, y: f32) -> Entity {
        Entity {
            pos: (x, y),
            speed: (0.0, 0.0)
        }
    }
}

struct Game {
    player: Entity
}

fn main() {
    let (mut width, mut height) = (640, 480);
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
    let tex = Path::new("./Link.png");
    let tex = Rc::new(Texture::from_path(&tex).unwrap());
    let mut link_sprite = Sprite::from_texture(tex.clone());
    link_sprite.set_position(0.0 as f64, 0.0 as f64);
    
    let link_sprite_id = scene.add_child(link_sprite);

    let window = RefCell::new(window);
    let ref mut gl = Gl::new(opengl);

    let mut game = Game {
        player: Entity::new(0.0, 100.0)
    };
    
    for e in Events::new(&window) {
        use event::{ PressEvent, ReleaseEvent, RenderEvent, ResizeEvent, UpdateEvent };

        scene.event(&e);

        e.resize(|w,h| {
            width = w;
            height = h;
            println!("new w = {}, new h = {}", w, h);
        });

        e.update(|args| {
            //println!("Update {}", args.dt);
            let (x, y) = game.player.pos;
            let (dx, dy) = game.player.speed;
            let dt = args.dt as f32;
            game.player.pos = (x + dx * dt, y + dy * dt);
        });

        e.render(|args| {
            use graphics::*;

            gl.viewport(0, 0, args.width as i32, args.height as i32);

            let c = Context::abs(args.width as f64, args.height as f64);
            c.rgb(1.0, 0.9, 1.0).draw(gl);

            let (x, y) = game.player.pos;
            scene.child_mut(link_sprite_id).unwrap().set_position(x as f64, y as f64);

            scene.draw(&c, gl);
        });
        
        e.press(|key| {
            //println!("Key = {}", key);
            match(key) {
                input::Keyboard( input::keyboard::Key::A ) => {
                    set_speed_entity(&mut game.player, (-300.0, 0.0))
                }
                input::Keyboard( input::keyboard::Key::D ) => {
                    set_speed_entity(&mut game.player, (300.0, 0.0))
                }
                _ => {
                    println! ("No key match");
                }
            }
        });

        e.release(|key| {
            match(key) {
                input::Keyboard( input::keyboard::Key::A ) => {
                    set_speed_entity(&mut game.player, (0.0, 0.0))
                }
                input::Keyboard( input::keyboard::Key::D ) => {
                    set_speed_entity(&mut game.player, (0.0, 0.0))
                }
                _ => {
                    println! ("No key match");
                }
            }
        });
    }

    fn set_speed_entity(e: &mut Entity, dir: (f32, f32)) {
        match dir {
            (dx,dy) => { e.speed = (dx, dy); }
        }        
    }
}
