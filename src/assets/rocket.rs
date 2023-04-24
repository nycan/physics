#[allow(dead_code)]
use crate::engine::*;
use piston::{RenderArgs,Key};
use opengl_graphics::GlGraphics;

const SURFACE_TEMP: f64 = 288.15;
const SURFACE: f64 = 6378137.0;

pub struct Rocket {
    // Changing variables
    pub mass:f64,
    pub velocity:[f64;2],
    pub pos:[f64;2],

    // Constants
    pub init_mass:f64,
    pub init_pos:[f64;2],
    pub exhaust_velocity:[f64;2],
    pub drag_coeff:f64,
    pub cross_section:f64,
    pub mass_flow_rate:f64,

    // Settings
    pub enable_thrust:bool,
    pub thrust_time:f64,
    pub apoapsis_reached:bool,
}

impl Object for Rocket{
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, settings:&UpdateParams){
        use graphics::*;

        //const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let rocket = rectangle::square(
            args.window_size[0]/2.0-25.0*settings.scale+self.pos[0]*5.0*settings.scale,
            args.window_size[1]-(self.pos[1]*5.0+50.0)*settings.scale,50.0*settings.scale
        );
        let flame = rectangle::square(
            args.window_size[0]/2.0-15.0*settings.scale+self.pos[0]*5.0*settings.scale,
            args.window_size[1]-self.pos[1]*5.0*settings.scale, 30.0*settings.scale
        );
        let ground = [0.0, args.window_size[1], args.window_size[0], -20.0*settings.scale];
        
        gl.draw(args.viewport(), |c, thingy| {

            rectangle(GREEN, ground, c.transform, thingy);
            rectangle(BLACK, rocket, c.transform, thingy);
            if (settings.time<=self.thrust_time) && (self.enable_thrust) {
                rectangle(ORANGE, flame, c.transform, thingy);
            }
        });
    }

    fn scale(&self, args: &RenderArgs) -> f64{
        ((args.window_size[1]-75.0)/(5.0*self.pos[1])).abs()
        .min(((args.window_size[0]/2.0-50.0)/(5.0*self.pos[0].abs())).abs())
    }

    fn update(&mut self, settings:&UpdateParams){ // DE Source: https://web.mit.edu/16.unified/www/FALL/systems/Lab_Notes/traj.pdf
        if self.pos[1] < 0.0 {
            return;
        }

        if (self.pos[1] > self.pos[1]+settings.time_delta*self.velocity[1]) && !self.apoapsis_reached{
            println!("rocket reached apoapsis at height: {}", self.pos[1]);
            self.apoapsis_reached=true;
        }

        let temp = SURFACE_TEMP - 0.0065*(self.pos[1]+SURFACE-EARTH_SEA_RADIUS);
        let density = air_density(self.pos[1]+SURFACE, temp);
        self.pos[0] += settings.time_delta*self.velocity[0];
        self.pos[1] += settings.time_delta*self.velocity[1];
        self.velocity[0] += settings.time_delta * (
            -0.5*density*self.velocity[0]*self.velocity[0]*self.drag_coeff*self.cross_section/self.mass
            *(if settings.enable_drag {1.0} else {0.0})
        );
        self.velocity[1] += settings.time_delta * (
            if settings.enable_gravity{-earth_gravity(self.pos[1]+SURFACE)} else {0.0}
            -0.5*density*self.velocity[1]*self.velocity[1]*self.drag_coeff*self.cross_section/self.mass
            *(if settings.enable_drag {1.0} else {0.0})
        );
        if (settings.time<=self.thrust_time) && (self.enable_thrust){
            self.velocity[0] += settings.time_delta * (self.mass_flow_rate*self.exhaust_velocity[0]/self.mass);
            self.velocity[1] += settings.time_delta * (self.mass_flow_rate*self.exhaust_velocity[1]/self.mass);
            self.mass -= settings.time_delta * self.mass_flow_rate;
        }
    }

    fn reset(&mut self){
        self.mass = self.init_mass;
        self.velocity = [0.0, 0.0];
        self.pos = self.init_pos;
    }

    fn take_input(&mut self, key:Key){

        match key{
            Key::Left => self.exhaust_velocity[0] -= 100.0,
            Key::Right => self.exhaust_velocity[0] += 100.0,
            Key::T => self.enable_thrust = !self.enable_thrust,
            _ => {},
        }
    }
}