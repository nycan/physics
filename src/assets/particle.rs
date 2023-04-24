#[allow(dead_code)]
use crate::engine::*;
use piston::{RenderArgs,Key};
use opengl_graphics::GlGraphics;

pub struct Particle{
    pos: [f64;2],
    velocity: [f64;2],
    accelleration: [f64;2],
    mass: f64,
}

impl Object for Particle{
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, settings:&UpdateParams){}
    fn scale(&self, args: &RenderArgs) -> f64{}
    fn update(&mut self, settings:&UpdateParams){}
    fn reset(&mut self){}
    fn take_input(&mut self, key:Key){}
}