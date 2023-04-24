#[allow(dead_code)]
use crate::engine::*;
use piston::{RenderArgs,Key};
use opengl_graphics::GlGraphics;

const SURFACE_TEMP: f64 = 288.15;
const SURFACE: f64 = 6378137.0;

pub struct IFO {
    // Changing variables
    pub velocity:[f64;2],
    pub pos:[f64;2],

    // Constants
    pub mass:f64,
    pub drag_coeff:f64,
    pub cross_section:f64,

    // Settings
    pub apoapsis_reached:bool,
}

impl Object for IFO {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, settings:&UpdateParams){
        use graphics::*;

        // const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let ifo = rectangle::square(
            args.window_size[0]/2.0-25.0*settings.scale+self.pos[0]*5.0*settings.scale,
            args.window_size[1]-(self.pos[1]*5.0+50.0)*settings.scale,50.0*settings.scale
        );
        let ground = [0.0, args.window_size[1], args.window_size[0], -20.0*settings.scale];

        gl.draw(args.viewport(), |c, thingy| {

            rectangle(GREEN, ground, c.transform, thingy);
            rectangle(BLACK, ifo, c.transform, thingy);
        });
    }

    fn scale(&self, args: &RenderArgs) -> f64 {
        ((args.window_size[1]-75.0)/(5.0*self.pos[1])).abs()
        .min(((args.window_size[0]/2.0-50.0)/(5.0*self.pos[0].abs())).abs())
    }

    fn update(&mut self, settings:&UpdateParams){
        if self.pos[1] < 0.0 {
            return;
        }

        if (self.pos[1] > self.pos[1]+settings.time_delta*self.velocity[1]) && !self.apoapsis_reached{
            println!("IFO reached apoapsis at height: {}", self.pos[1]);
            self.apoapsis_reached=true;
        }

        let temp = SURFACE_TEMP - 0.0065*(self.pos[1]+SURFACE-EARTH_SEA_RADIUS);
        let density = air_density(self.pos[1]+SURFACE, temp);
        self.pos[0] += settings.time_delta*self.velocity[0];
        self.pos[1] += settings.time_delta*self.velocity[1];
        self.velocity[1] += settings.time_delta * (
            if settings.enable_gravity{-earth_gravity(self.pos[1]+SURFACE)} else {0.0}
            -0.5*density*self.velocity[1]*self.velocity[1]*self.drag_coeff*self.cross_section/self.mass
            *(if settings.enable_drag {1.0} else {0.0})
        );
        self.velocity[0] += settings.time_delta * (
            -0.5*density*self.velocity[0]*self.velocity[0]*self.drag_coeff*self.cross_section/self.mass
            *(if settings.enable_drag {1.0} else {0.0})
        );
    }

    fn reset(&mut self){
        self.velocity=[0.0,0.0];
        self.pos=[0.0,0.0]
    }

    fn take_input(&mut self, key:Key){
        match key{
            _ => {},
        }
    }
}