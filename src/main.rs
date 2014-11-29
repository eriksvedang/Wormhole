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
    pos: (f64, f64),
    speed: (f64, f64)
}

impl Entity {
    fn new(x: f64, y: f64) -> Entity {
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
    // let tex = Path::new("./Link.png");
    // let tex = Rc::new(Texture::from_path(&tex).unwrap());
    
    let linkTexture = Texture::from_path(&Path::new("./Link.png")).unwrap();

    let window = RefCell::new(window);
    let ref mut gl = Gl::new(opengl);

    let mut game = Game {
        player: Entity::new(0.0, 100.0)
    };
    
    for e in Events::new(&window) {
        use event::{ PressEvent, ReleaseEvent, RenderEvent, ResizeEvent, UpdateEvent };
        use input::keyboard::Key;
        
        scene.event(&e);

        e.resize(|w,h| {
            width = w;
            height = h;
            println!("new w = {}, new h = {}", w, h);
        });

        e.update(|args| {
            use std::num::FloatMath;
            //println!("Update {}", args.dt);
            let (x, y) = game.player.pos;
            let (dx, dy) = game.player.speed;
            if dx != 0.0 || dy != 0.0 {
                let angle = dy.atan2(dx);
                let speed = 300.0;
                let dt = args.dt as f64;
                game.player.pos = (x + angle.cos() * speed * dt, y + angle.sin() * speed * dt);
                let (newX, newY) = game.player.pos;
                game.player.pos = (wrap(newX, width as f64), wrap(newY, height as f64))
            }            
        });

        e.render(|args| {

            gl.viewport(0, 0, args.width as i32, args.height as i32);

            let c = Context::abs(args.width as f64, args.height as f64);
            c.rgb(1.0, 0.9, 1.0).draw(gl);

            let (x, y) = game.player.pos;
            draw_entity(&game.player, &c, &linkTexture, gl, (width, height));
            //scene.child_mut(link_sprite_id).unwrap().set_position(x as f64, y as f64);
            //scene.draw(&c, gl);   
        });

        
        e.press(|key| {
            //println!("Key = {}", key);
            let (dx, dy) = game.player.speed;
            match key {
                input::Keyboard( Key::A ) => {
                    set_speed_entity(&mut game.player, (-1.0, dy))
                }
                input::Keyboard( Key::D ) => {
                    set_speed_entity(&mut game.player, (1.0, dy))
                }
                input::Keyboard( Key::W ) => {
                    set_speed_entity(&mut game.player, (dx, -1.0))
                }
                input::Keyboard( Key::S ) => {
                    set_speed_entity(&mut game.player, (dx, 1.0))
                }
                _ => {
                    println! ("No key match");
                }
            }
        });

        e.release(|key| {
            let (dx, dy) = game.player.speed;
            match(key) {
                input::Keyboard( Key::A ) => {
                    if(dx < 0.0) {
                        set_speed_entity(&mut game.player, (0.0, dy))
                    }                    
                }
                input::Keyboard( Key::D ) => {
                    if(dx > 0.0) {
                        set_speed_entity(&mut game.player, (0.0, dy))
                    }
                }
                input::Keyboard( Key::W ) => {
                    if(dy < 0.0) {
                        set_speed_entity(&mut game.player, (dx, 0.0))
                    }
                }
                input::Keyboard( Key::S ) => {
                    if(dy > 0.0) {
                        set_speed_entity(&mut game.player, (dx, 0.0))
                    }
                }
                _ => {
                    println! ("No key match");
                }
            }
        });
    }

    fn draw_entity( e: &Entity, c: &Context, image: &Texture, gl: &mut Gl, size: (u32,u32)) {
        let (width, height) = size;
        let width = width as f64;
        let height = height as f64;
        let (x,y) = e.pos;
        
        draw_image_at(e.pos, c, image, gl);

        draw_image_at((x + width, y), c, image, gl);
        draw_image_at((x - width, y), c, image, gl);
        draw_image_at((x, y + height), c, image, gl);
        draw_image_at((x, y - height), c, image, gl);
        //draw_image_at((wrap(x, width), wrap(y, height)), c, image, gl);


          //  draw_image_at(e.pos, c, image, gl);
          //  draw_image_at(e.pos, c, image, gl);


    }
    
    fn wrap(val: f64, size: f64) -> f64
    {
        let x = if(val > 0.0){
            0.0
        }else{
            size
        };
        val % size + x


    }

    fn draw_image_at(pos: (f64,f64), c: &Context, image: &Texture, gl: &mut Gl) {
        let (x,y) = pos;
        let (_, h) = image.get_size();
        let h = h as f64;
        c   .image(image)
            .trans(x + 3.0, y + h - 0.3 * h)
            .scale(1.0, 0.3)
            .rgb(0.5, 0.5, 0.5)
            .draw(gl);
            
        c   .image(image)
            .trans(x, y)
            .draw(gl);


    }


    fn set_speed_entity(e: &mut Entity, dir: (f64, f64)) {
        match dir {
            (dx,dy) => { e.speed = (dx, dy); }
        }        
    }
}
