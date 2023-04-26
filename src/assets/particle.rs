#[allow(dead_code)]
use crate::engine::*;
use piston::{RenderArgs,Key};
use opengl_graphics::GlGraphics;

pub struct Particle{
    pos: [f64;2],
    velocity: [f64;2],
    accelleration: [f64;2],
    angle: f64,
    angular_velocity: f64,
    angular_accelleration: f64,
    mass: f64,
    radius: f64,
}

impl Object for Particle{
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, settings:&UpdateParams){
        use graphics::*;
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let rocket = ellipse::circle(
            args.window_size[0]/2.0-25.0*settings.scale+self.pos[0]*5.0*settings.scale,
            args.window_size[1]-(self.pos[1]*5.0+50.0)*settings.scale,self.radius*settings.scale
        );
        
        gl.draw(args.viewport(), |c, thingy| {ellipse(BLACK, rocket, c.transform, thingy);});
    }
    fn scale(&self, args: &RenderArgs) -> f64{}
    fn update(&mut self, settings:&UpdateParams){

    }
    fn reset(&mut self){}
    fn take_input(&mut self, key:Key){}
}