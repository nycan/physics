extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;

#[allow(dead_code)]
mod formulas;
use formulas::*;

const SURFACE_TEMP: f64 = ROOM_TEMP;
const SURFACE: f64 = EARTH_SEA_RADIUS;

pub struct Rocket {
    gl: opengl_graphics::GlGraphics, // OpenGL drawing backend.
    // Changing variables
    mass:f64,
    velocity:[f64;2],
    pos:[f64;2],
    time:f64,

    // Constants
    exhaust_velocity:[f64;2],
    drag_coeff:f64,
    cross_section:f64,
    mass_flow_rate:f64,

    // Settings
    enable_thrust:bool,
    enable_drag:bool,
    enable_gravity:bool,
    thrust_time:f64,
    paused:bool,
    time_delta:f64,
}

impl Rocket{
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let scale = 1.0_f64
            .min(((args.window_size[1]-75.0)/(5.0*self.pos[1])).abs())
            .min(((args.window_size[0]/2.0-50.0)/(5.0*self.pos[0].abs())).abs());
        let rocket = rectangle::square(
            args.window_size[0]/2.0-25.0*scale+self.pos[0]*5.0*scale,
            args.window_size[1]-(self.pos[1]*5.0+50.0)*scale,50.0*scale
        );
        let flame = rectangle::square(
            args.window_size[0]/2.0-15.0*scale+self.pos[0]*5.0*scale,
            args.window_size[1]-self.pos[1]*5.0*scale, 30.0*scale
        );
        let ground = [0.0, args.window_size[1], args.window_size[0], -20.0*scale];
        
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            rectangle(GREEN, ground, c.transform, gl);
            rectangle(BLACK, rocket, c.transform, gl);
            if (self.time<=self.thrust_time) && (self.enable_thrust) {
                rectangle(ORANGE, flame, c.transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs){ // DE Source: https://web.mit.edu/16.unified/www/FALL/systems/Lab_Notes/traj.pdf
        self.time_delta = args.dt;
        println!("{}", self.time_delta);
        
        if (self.pos[1] < 0.0) || (self.paused) {
            return;
        }

        let temp = SURFACE_TEMP - 0.0065*(self.pos[1]+SURFACE-EARTH_SEA_RADIUS);
        let density = air_density(self.pos[1]+SURFACE, temp);
        self.pos[0] += self.time_delta*self.velocity[0];
        self.pos[1] += self.time_delta*self.velocity[1];
        self.velocity[0] += self.time_delta * (
            -0.5*density*self.velocity[0]*self.velocity[0]*self.drag_coeff*self.cross_section/self.mass
            *(if self.enable_drag {1.0} else {0.0})
        );
        self.velocity[1] += self.time_delta * (
            -earth_gravity(self.pos[1]+SURFACE)*(if self.enable_gravity {1.0} else { 0.0 })
            -0.5*density*self.velocity[1]*self.velocity[1]*self.drag_coeff*self.cross_section/self.mass
            *(if self.enable_drag {1.0} else {0.0})
        );
        if (self.time<=self.thrust_time) && (self.enable_thrust){
            self.velocity[0] += self.time_delta * (self.mass_flow_rate*self.exhaust_velocity[0]/self.mass);
            self.velocity[1] += self.time_delta * (self.mass_flow_rate*self.exhaust_velocity[1]/self.mass);
            self.mass -= self.time_delta * self.mass_flow_rate;
        }

        self.time += self.time_delta;
    }

    fn reset(&mut self){
        self.mass = 0.2;
        self.velocity = [0.0, 0.0];
        self.pos = [0.0, 0.0];
        self.time = 0.0;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = 
        WindowSettings::new("vroooom", [640, 480])
        .graphics_api(opengl).exit_on_esc(true).build().unwrap();

    let mut rocket = Rocket {
        gl: GlGraphics::new(opengl),
        mass: 0.2,
        velocity: [0.0, 0.0],
        pos: [0.0, 0.0],
        exhaust_velocity: [0.0,650.0],
        drag_coeff: 0.1,
        cross_section: 0.01,
        mass_flow_rate: 0.01,
        time: 0.0,
        enable_thrust: true,
        enable_drag: true,
        enable_gravity: true,
        thrust_time: 4.5,
        paused: false,
        time_delta: 0.01,
    };
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            rocket.render(&args);
        }

        if let Some(args) = e.update_args() {
            rocket.update(&args);
        }

        if let Some(Button::Keyboard(key)) = e.press_args(){
            match key{
                Key::R => rocket.reset(),
                Key::Space => rocket.paused = !rocket.paused,
                Key::Left => rocket.exhaust_velocity[0] -= 100.0,
                Key::Right => rocket.exhaust_velocity[0] += 100.0,
                Key::T => rocket.enable_thrust = !rocket.enable_thrust,
                Key::D => rocket.enable_drag = !rocket.enable_drag,
                Key::G => rocket.enable_gravity = !rocket.enable_gravity,
                _ => {}
            }
        }
    }
}
