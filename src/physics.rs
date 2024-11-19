use crate::planet::Planet;
use glam::Vec2;

pub fn calc_accel(self_planet: &Planet, planet: &Planet) -> Vec2 {
    let G: f32 = 6.6 * (10.0_f32).powf(-11.0_f32);
    let mut resultant_force = Vec2::new(0.0, 0.0);
    //ma = Gmm/r^2   -> a = Gm/r^2
    let dist = ((planet.pos.x - self_planet.pos.x).powf(2.0)
        + (planet.pos.y - self_planet.pos.y).powf(2.0))
    .sqrt()
    .abs();
    let magnitude = (G * self_planet.mass * planet.mass) / dist.powi(2);
    let force = (planet.pos - self_planet.pos).normalize_or_zero() * magnitude;
    resultant_force += force;
    let accel = force / self_planet.mass;
    accel
}

pub fn calc_init_orbital_velocity(planet: &Planet, sun: &Planet) -> Vec2 {
    // V = ((Gm)/r).sqrt()
    let G: f32 = 6.6 * (10.0_f32).powf(-11.0_f32);
    let dist = ((sun.pos.x - planet.pos.x).powf(2.0) + (sun.pos.y - planet.pos.y).powf(2.0)).sqrt();
    let v = ((G * sun.mass) / dist).sqrt();
    let force = (sun.pos - planet.pos).perp().normalize_or_zero() * v;
    force
}

pub fn check_escape_velocity(planet: &Planet, sun: &Planet) -> bool {
    // checks whether a given planet will escape the orbit of a given sun
    let G: f32 = 6.6 * (10.0_f32).powf(-11.0_f32);
    let dist = ((sun.pos.x - planet.pos.x).powf(2.0) + (sun.pos.y - planet.pos.y).powf(2.0))
        .sqrt()
        .abs();
    let escape_vel = ((2.0 * G * sun.mass) / dist).sqrt();
    if planet.vel.length() > escape_vel {
        true
    } else {
        false
    }
}

pub fn check_collision(self_planet: &Planet, planet: &Planet) -> bool {
    let check = (self_planet.pos.distance(planet.pos) <= self_planet.radius + planet.radius);
    if check == true {
        println!("Collision detected!")
    }
    check
}

pub fn calc_collision(self_planet: &Planet, planet: &Planet) -> Vec2 {
    let old_vel = self_planet.vel;
    let part_a = (2.0 * planet.mass / (self_planet.mass + planet.mass));
    let part_b = ((self_planet.vel - planet.vel).dot(self_planet.pos - planet.pos))
        / (self_planet.pos.distance(planet.pos).powi(2));
    let part_c = self_planet.pos - planet.pos;
    let mut collision_force = part_a * part_b * part_c;
    collision_force = collision_force.reflect(planet.vel);
    collision_force
}

pub fn calc_collision2(self_planet: &Planet, planet: &Planet) -> Vec2 {
    // Vector from the other planet to this one (collision axis)
    let collision_axis = self_planet.pos - planet.pos;
    let distance_squared = collision_axis.length_squared();

    if distance_squared == 0.0 {
        // To handle any edge cases where positions might be identical, avoid division by zero
        return Vec2::ZERO;
    }

    // Normalized direction vector along the collision axis
    let collision_axis_normalized = collision_axis.normalize();

    // Relative velocity in the direction of the collision axis
    let relative_velocity = self_planet.vel - planet.vel;
    let velocity_along_axis = relative_velocity.dot(collision_axis_normalized);

    // Mass ratio factor for elastic collision
    let mass_factor = (2.0 * planet.mass) / (self_planet.mass + planet.mass);

    // The force applied in the direction of the collision axis
    let collision_force = mass_factor * velocity_along_axis * collision_axis_normalized;

    collision_force
}

pub fn get_gradient(vel: Vec2) -> f32 {
    let norm = vel.normalize_or_zero();
    let y_d = norm.y;
    let x_d = norm.x;
    y_d / x_d
}
