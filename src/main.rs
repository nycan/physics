extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;

const GRAVITY:f64 = 398584628000000.0;
const SURFACE:f64 = 6378137.0;
const SURFACE_TEMP:f64 = 288.15;
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
    mass_flow_rate:f64,

    // Settings
    enable_thrust:bool,
    enable_drag:bool,
    enable_gravity:bool,
    thrust_time:i32,
    large_mode: bool,
    paused:bool,
}

impl App{
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 1.0];

        let square = rectangle::square(args.window_size[0]/2.0-25.0, args.window_size[1]-self.height*5.0*(if self.large_mode{0.02}else{1.0})-50.0, 50.0);
        let flame = rectangle::square(args.window_size[0]/2.0-15.0, args.window_size[1]-self.height*5.0*(if self.large_mode{0.02}else{1.0}), 30.0);
        

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            //Draw the rocket
            rectangle(BLACK, square, c.transform, gl);
            if (self.time<=self.thrust_time) && (self.enable_thrust) {
                rectangle(ORANGE, flame, c.transform, gl);
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs){ // DE Source: https://web.mit.edu/16.unified/www/FALL/systems/Lab_Notes/traj.pdf
        if (self.height < 0.0) || (self.paused) {
            return;
        }

        let grav = GRAVITY/(SURFACE+self.height).powi(2);
        let temp = SURFACE_TEMP - 0.0065*(self.height+SURFACE-6378137.0);
        let pressure = 101325.0 * (1.0-grav*self.height/289510.047).powf(3.50057557);
        let density = pressure/(287.05*temp);
        self.height += 0.01*self.velocity;
        self.velocity += 0.01 * (
            -grav*(if self.enable_gravity {1.0} else { 0.0 })
            -0.5*density*self.velocity*self.velocity*self.drag_coeff*self.cross_section/self.mass*(if self.enable_drag {1.0} else {0.0})
        );

        if (self.time<=self.thrust_time) && (self.enable_thrust){
            self.velocity += 0.01 * (self.mass_flow_rate*self.exhaust_velocity/self.mass);
            self.mass -= 0.01 * self.mass_flow_rate;
        }

        self.time += 1;
    }

    fn pause(&mut self){
        self.paused = !self.paused;
    }

    fn reset(&mut self){
        self.mass = 0.2;
        self.velocity = 0.0;
        self.height = 0.0;
        self.time = 0;
    }

    fn toggle_thrust(&mut self){
        self.enable_thrust = !self.enable_thrust;
    }

    fn toggle_drag(&mut self){
        self.enable_drag = !self.enable_drag;
    }

    fn toggle_gravity(&mut self){
        self.enable_gravity = !self.enable_gravity;
    }

    fn toggle_large_mode(&mut self){
        self.large_mode = !self.large_mode;
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
        mass_flow_rate: 0.01,
        time: 0,
        enable_thrust: true,
        enable_drag: true,
        enable_gravity: true,
        thrust_time: 150,
        paused: false,
        large_mode: false,
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
                Key::Space => app.pause(),
                Key::R => app.reset(),
                Key::T => app.toggle_thrust(),
                Key::D => app.toggle_drag(),
                Key::G => app.toggle_gravity(),
                Key::L => app.toggle_large_mode(),
                _ => {}
            }
        }
    }
}
