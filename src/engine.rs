extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;
const OPENGL : OpenGL = OpenGL::V3_2;

pub const GRAVITY:f64 = 6.674e-11_f64;
pub const EARTH_GRAVITY:f64 = 398584628000000.0;
pub const LIGHT_SPEED:f64 = 299792458.0;
pub const EARTH_SEA_RADIUS:f64 = 6378137.0;
pub const ROOM_TEMP:f64 = 288.15;

//formulas
pub fn gravity(mass:f64, distance:f64) -> f64{GRAVITY*mass/distance.powi(2)}
pub fn earth_gravity(radius:f64) -> f64 {EARTH_GRAVITY/radius.powi(2)}
pub fn earth_pressure(radius:f64) -> f64 {101325.0 * (1.0-earth_gravity(radius)/289510.047).powf(3.50057557)}
pub fn air_density(radius:f64, temp:f64) -> f64 {earth_pressure(radius)/(287.05*temp)}

type order1_dx = fn(f64,f64)->f64;
type order2_dx = fn(f64,f64,f64)->f64;

//y' = f(x,y), returns y
pub fn rk4_order1(dx1: order1_dx, x:f64, y:f64, step: f64) -> f64{
    let k1 = dx1(x,y);
    let k2 = dx1(x+step/2.0, y+step*k1/2.0);
    let k3 = dx1(x+step/2.0, y+step*k2/2.0);
    let k4 = dx1(x+step, y+step*k3);
    y+step/6.0*(k1+2.0*k2+2.0*k3+k4)
}

//y'' = f(x,y,y'), returns [y,y']
pub fn rk4_order2(dx2: order2_dx,  x:f64, y:f64, dy:f64, step:f64) -> [f64;2]{
    let k1 = dy;
    let l1 = dx2(x,y,dy);
    let k2 = dy+0.5*l1;
    let l2 = dx2(x+0.5*step, y+0.5*k1, dy+0.5*l1);
    let k3 = dy+0.5*l2;
    let l3 = dx2(x+0.5*step, y+0.5*k2, dy+0.5*l2);
    let k4 = dy+l3;
    let l4 = dx2(x+step, y+k3, dy+l3);
    [y+step/6.0*(k1+2.0*k2+2.0*k3+k4), dy+step/6.0*(l1+2.0*l2+2.0*l3+l4)]
}

pub struct UpdateParams{
    pub time:f64,
    pub time_delta:f64,
    pub paused:bool,
    pub enable_gravity:bool,
    pub enable_drag:bool,
    pub scale: f64,
}

pub trait Object{
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, settings:&UpdateParams);
    fn scale(&self, args: &RenderArgs) -> f64;
    fn update(&mut self, settings:&UpdateParams);
    fn reset(&mut self);
    fn take_input(&mut self, key:Key);
}

pub struct Engine{
    window: Window,
    gl: opengl_graphics::GlGraphics,
    objects: Vec<Box<dyn Object>>,
    settings: UpdateParams,
}

impl Engine{
    pub fn new() -> Engine{
        Engine{
            window: WindowSettings::new("vroooom", [640, 480])
            .graphics_api(OPENGL).exit_on_esc(true).build().unwrap(),
            gl: GlGraphics::new(OPENGL),
            objects:Vec::new(),
            settings:UpdateParams{
                time:0.0,
                time_delta:0.01,
                paused:false,
                enable_gravity:true,
                enable_drag:true,
                scale:1.0,
            }
        }
    }

    fn reset(&mut self){
        self.settings.time = 0.0;
        for object in self.objects.iter_mut(){
            object.reset();
        }
    }

    fn update(&mut self){
        for object in self.objects.iter_mut(){
            object.update(&self.settings);
        }
        self.settings.time += self.settings.time_delta;
    }

    fn render(&mut self, args: &RenderArgs){
        self.settings.scale = 1.0;
        for object in self.objects.iter_mut(){
            self.settings.scale = self.settings.scale.min(object.scale(args));
        }
        self.gl.draw(args.viewport(), |_c, thingy| {
            graphics::clear([1.0,1.0,1.0,1.0], thingy);
        });
        for object in self.objects.iter_mut(){
            object.render(&mut self.gl, args, &self.settings);
        }
    }

    pub fn add_object<T: Object + 'static>(&mut self, object:T){
        self.objects.push(Box::new(object));
    }

    pub fn run(&mut self){
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.render(&args);
            }

            if let Some(args) = e.update_args() {
                self.settings.time_delta = if self.settings.paused {0.0} else {args.dt};
                self.update();
            }

            if let Some(Button::Keyboard(key)) = e.press_args(){
                match key{
                    Key::R => self.reset(),
                    Key::Space => self.settings.paused = !self.settings.paused,
                    Key::G => self.settings.enable_gravity = !self.settings.enable_gravity,
                    Key::D => self.settings.enable_drag = !self.settings.enable_drag,
                    _ => {}
                }
                for object in self.objects.iter_mut(){
                    object.take_input(key);
                }
            }
        }
    }
}