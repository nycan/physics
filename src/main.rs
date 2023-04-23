extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;

mod formulas;
use formulas::*;

const SURFACE_TEMP: f64 = ROOM_TEMP;
const SURFACE: f64 = EARTH_SEA_RADIUS;

pub struct App {
    gl: opengl_graphics::GlGraphics, // OpenGL drawing backend.
    // Changing variables
    mass:f64,
    velocity:[f64;2],
    pos:[f64;2],
    time:i32,

    // Constants
    exhaust_velocity:[f64;2],
    drag_coeff:f64,
    cross_section:f64,
    mass_flow_rate:f64,

    // Settings
    enable_thrust:bool,
    enable_drag:bool,
    enable_gravity:bool,
    thrust_time:i32,
    paused:bool,
}

impl App{
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

    fn update(&mut self, _args: &UpdateArgs){ // DE Source: https://web.mit.edu/16.unified/www/FALL/systems/Lab_Notes/traj.pdf
        if (self.pos[1] < 0.0) || (self.paused) {
            return;
        }

        let temp = SURFACE_TEMP - 0.0065*(self.pos[1]+SURFACE-EARTH_SEA_RADIUS);
        let density = air_density(self.pos[1]+SURFACE, temp);
        self.pos[0] += 0.01*self.velocity[0];
        self.pos[1] += 0.01*self.velocity[1];
        self.velocity[0] += 0.01 * (
            -0.5*density*self.velocity[0]*self.velocity[0]*self.drag_coeff*self.cross_section/self.mass
            *(if self.enable_drag {1.0} else {0.0})
        );
        self.velocity[1] += 0.01 * (
            -earth_gravity(self.pos[1]+SURFACE)*(if self.enable_gravity {1.0} else { 0.0 })
            -0.5*density*self.velocity[1]*self.velocity[1]*self.drag_coeff*self.cross_section/self.mass
            *(if self.enable_drag {1.0} else {0.0})
        );
        if (self.time<=self.thrust_time) && (self.enable_thrust){
            self.velocity[0] += 0.01 * (self.mass_flow_rate*self.exhaust_velocity[0]/self.mass);
            self.velocity[1] += 0.01 * (self.mass_flow_rate*self.exhaust_velocity[1]/self.mass);
            self.mass -= 0.01 * self.mass_flow_rate;
        }

        self.time += 1;
    }

    fn reset(&mut self){
        self.mass = 0.2;
        self.velocity = [0.0, 0.0];
        self.pos = [0.0, 0.0];
        self.time = 0;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = 
        WindowSettings::new("vroooom", [640, 480])
        .graphics_api(opengl).exit_on_esc(true).build().unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        mass: 0.2,
        velocity: [0.0, 0.0],
        pos: [0.0, 0.0],
        exhaust_velocity: [0.0,650.0],
        drag_coeff: 0.1,
        cross_section: 0.01,
        mass_flow_rate: 0.01,
        time: 0,
        enable_thrust: true,
        enable_drag: true,
        enable_gravity: true,
        thrust_time: 450,
        paused: false,
    };
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(Button::Keyboard(key)) = e.press_args(){
            match key{
                Key::R => app.reset(),
                Key::Space => app.paused = !app.paused,
                Key::Left => app.exhaust_velocity[0] -= 100.0,
                Key::Right => app.exhaust_velocity[0] += 100.0,
                Key::T => app.enable_thrust = !app.enable_thrust,
                Key::D => app.enable_drag = !app.enable_drag,
                Key::G => app.enable_gravity = !app.enable_gravity,
                _ => {}
            }
        }
    }
}
