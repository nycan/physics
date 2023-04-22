extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

// Test comment


pub struct App {
    gl: opengl_graphics::GlGraphics, // OpenGL drawing backend.
    // Changing variables
    mass:f64,
    velocity:f64,
    height:f64,
    time:i32,

    // Constants
    exhaust_velocity:f64,
    drag_coeff:f64,
    cross_section:f64,
    G:f64,
    mass_flow_rate:f64,
    air_density:f64,

    // Settings
    enable_thrust:bool,
    enable_drag:bool,
    enable_gravity:bool,
}

impl App{
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(args.window_size[0]/2.0-25.0, args.window_size[1]-self.height*5.0-50.0, 50.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            //Draw the rocket
            rectangle(BLACK, square, c.transform, gl);
        });
    }
    fn update(&mut self, args: &UpdateArgs){ // DE Source: https://web.mit.edu/16.unified/www/FALL/systems/Lab_Notes/traj.pdf
        
        if self.height < 0.0 {
            return;
        }

        self.height += 0.01*self.velocity;
        self.velocity += 0.01 * (
            -self.G*(if self.enable_gravity {1.0} else { 0.0 })
            -0.5*self.air_density*self.velocity*self.velocity*self.drag_coeff*self.cross_section/self.mass*(if self.enable_drag {1.0} else {0.0})
        );

        if (self.time<=100) && (self.enable_thrust){
            self.velocity += 0.01 * (self.mass_flow_rate*self.exhaust_velocity/self.mass);
            self.mass -= 0.01 * self.mass_flow_rate;
        }

        self.time += 1;
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
        velocity: 0.0,
        height: 0.0,
        exhaust_velocity: 650.0,
        drag_coeff: 0.1,
        cross_section: 0.01,
        G: 9.8,
        mass_flow_rate: 0.01,
        air_density: 1.3,
        time: 0,
        enable_thrust: true,
        enable_drag: true,
        enable_gravity: true,
    };
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
