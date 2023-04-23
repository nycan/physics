pub const GRAVITY:f64 = 6.674e-11_f64;
pub const EARTH_GRAVITY:f64 = 398584628000000.0;
pub const LIGHT_SPEED:f64 = 299792458.0;
pub const EARTH_SEA_RADIUS:f64 = 6378137.0;
pub const ROOM_TEMP:f64 = 288.15;

pub fn gravity(mass:f64, distance:f64){
    GRAVITY*mass/distance.powi(2)
}

pub fn earth_gravity(radius:f64) -> f64 {
    EARTH_GRAVITY/radius.powi(2)
}

pub fn earth_pressure(radius:f64) -> f64 {
    101325.0 * (1.0-earth_gravity(radius)/289510.047).powf(3.50057557)
}

pub fn air_density(radius:f64, temp:f64) -> f64 {
    earth_pressure(radius)/(287.05*temp)
}