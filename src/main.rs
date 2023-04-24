#[allow(dead_code)]
mod engine;
use engine::*;

use assets::rocket::Rocket;
use assets::ifo::IFO;

mod assets;

fn main() {
    let mut engine = Engine::new();

    let rocket = Rocket {
        mass: 0.2,
        init_mass: 0.2,
        velocity: [0.0, 0.0],
        pos: [0.0, 0.0],
        init_pos: [0.0, 0.0],
        exhaust_velocity: [0.0,650.0],
        drag_coeff: 0.1,
        cross_section: 0.01,
        mass_flow_rate: 0.01,
        enable_thrust: true,
        thrust_time: 4.5,
        apoapsis_reached: false,
    };
    engine.add_object(rocket);

    let rocket2 = Rocket {
        mass: 0.2,
        init_mass: 0.2,
        velocity: [0.0, 0.0],
        pos: [100.0, 0.0],
        init_pos: [100.0, 0.0],
        exhaust_velocity: [0.0,650.0],
        drag_coeff: 0.1,
        cross_section: 0.01,
        mass_flow_rate: 0.01,
        enable_thrust: true,
        thrust_time: 4.5,
        apoapsis_reached: false,
    };
    engine.add_object(rocket2);

    let ifo:IFO = IFO {
        mass: 1.0,
        velocity: [100.0,100.0],
        init_velocity: [100.0,100.0],
        pos: [-100.0,0.0],
        init_pos: [-100.0,0.0],
        drag_coeff: 1.0,
        cross_section: 0.01,
        apoapsis_reached: false,
    };
    engine.add_object(ifo);

    engine.run();
}
