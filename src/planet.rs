use rand;
use rand::Rng;
use glam::Vec2;
use crate::physics::*;
use crate::*;
use pixels::Pixels;

#[derive(Copy, Clone, PartialEq)]
pub struct Planet<'a> {
    pub name: &'a str,
    pub pos: Vec2,
    pub radius: f32,
    pub vel: Vec2,
    pub mass: f32,
    pub color: PlanetColor,
    pub accel: Vec2
}

impl<'a> Planet<'a> {
    pub fn new(name: &'a str, pos: Vec2, radius: f32, vel: Vec2, mass: f32, color: PlanetColor) -> Planet<'a> {
       Planet {name, pos, radius, vel, mass, color, accel: Vec2::new(1.0, 1.0)} 
    }

    pub fn create_satellite( sun: & Planet, name: &'a str, radius: f32, mass: f32, color: PlanetColor) -> Planet<'a> {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(25.0*radius..50.0*radius);
        let mut x: f32 = 200.0;
        if r > 2.5*radius {
            let y = sun.pos.x + r;
        } else {
            let y = sun.pos.x - r;
        }
        let y = (r.powi(2)-x.powi(2)).sqrt();
        let pos = Vec2::new(x, y);

        let mut new_planet = Self::new(name, pos, radius, Vec2::new(0.0, 0.0), mass, color);
        let new_velocity = calc_init_orbital_velocity(&new_planet, &sun);
        new_planet.vel = new_velocity;
        println!("{} will escape orbit: {}", new_planet.name, check_escape_velocity(&new_planet, &sun));
        new_planet
    }

    pub fn render (&self, px: &mut Pixels) {
        for y in ((self.pos.y-self.radius) as usize)..((self.pos.y+self.radius) as usize) {
            for x in ((self.pos.x-self.radius) as usize)..((self.pos.x+self.radius) as usize) {
                let circle_check = (x as f32 - self.pos.x).powf(2.0) + (y as f32 - self.pos.y).powf(2.0);
                if (circle_check < self.radius.powf(2.0)) || (circle_check == self.radius.powf(2.0)) {
                    let index = y * WIDTH * 4 + x * 4 as usize;
                    if index >= (WIDTH * HEIGHT * 4) {
                        continue;
                    }
                    if x < 0 || x > WIDTH {
                        continue;
                    } 
                    if y < 0 || y > HEIGHT {
                        continue
                    }
                    let r = self.color.r as u8;
                    let g = self.color.g as u8;
                    let b = self.color.b as u8;
                    px.frame_mut()[index..index+4].copy_from_slice(vec![r, g, b, 255u8].as_slice());
                }
            }
        }
    }

    pub fn render_force (&self, px: &mut Pixels) {
        //in a space thats around the bounding box of a planet x5
        //check if pixels fall on line of vector
        //how do i get the gradient
        for y in ((self.pos.y-self.radius*5.0) as usize)..((self.pos.y+self.radius*5.0) as usize) {
            for x in ((self.pos.x-self.radius*5.0) as usize)..((self.pos.y+self.radius*5.0) as usize) {
                let thickness = 0.1;
                let norm = self.vel.normalize_or_zero();
                let gradient = norm.y / norm.x;
                let line_check = gradient * x as f32;
                if line_check <= thickness {
                 let index = y * WIDTH * 4 + x * 4 as usize;
                    if index >= (WIDTH * HEIGHT * 4) {
                        continue;
                    }
                    if x < 0 || x > WIDTH {
                        continue;
                    } 
                    if y < 0 || y > HEIGHT {
                        continue
                    }
                    let r = self.color.r as u8;
                    let g = self.color.g as u8;
                    let b = self.color.b as u8;
                    px.frame_mut()[index..index+4].copy_from_slice(vec![r, g, b, 255u8].as_slice());

                }
            }
        }
    }

    pub fn render_force2 (&self, px: &mut Pixels) {   //render acceleration, can be changed to
        //render velocity
        let scale = 50.2;
        
        let start_x = self.pos.x;
        let start_y = self.pos.y;
        let end_x = (start_x + self.accel.x * scale) as i32;
        let end_y = (start_y + self.accel.y * scale) as i32;

        let dx = (end_x - start_x as i32).abs();
        let dy = (end_y - start_y as i32).abs();

        let sx: i32 = if (start_x as i32) < end_x { 1 } else { -1 };
        let sy: i32 = if (start_y as i32) < end_y { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = self.pos.x as i32;
        let mut y = self.pos.y as i32;

        loop {
            if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
                let index = (y * WIDTH as i32  + x ) as usize * 4;
                    let r = self.color.r as u8;
                    let g = self.color.g as u8;
                    let b = self.color.b as u8;
                    px.frame_mut()[index..index+4].copy_from_slice(vec![r, g, b, 255u8].as_slice());


            }
            if x == end_x && y == end_y {
                break;
            }
            let e2 = err * 2i32;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn update (&mut self, accel: Vec2) {
        // self.vel += Vec2::new(0.01, 0.01);  //accel
        self.accel = accel;
        self.vel += accel * delta_t;
        self.pos += self.vel * delta_t;
        // println!("{}",self.vel);
    }
}


#[derive(Copy, Clone, PartialEq)]
pub struct PlanetColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl PlanetColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> PlanetColor
    {
        PlanetColor {r, g, b, a}
    }

    pub fn white() -> PlanetColor {
        Self::new(255, 255, 255, 255)
    }
    
    pub fn black() -> PlanetColor {
        Self::new(0,0,0,255)
    }

    pub fn red() -> PlanetColor {
        Self::new(255, 0, 0, 255)
    }

    pub fn green() -> PlanetColor {
        Self::new(0, 255, 0, 255)
    }

    pub fn blue() -> PlanetColor {
        Self::new(0, 0, 255, 255)
    }

}

#[derive(Copy, Clone, PartialEq)]
pub struct PlanetTrail {
    pos: Vec2,
    radius: f32,
    color: PlanetColor
}

impl PlanetTrail {
    pub fn new(pos: Vec2, radius: f32, color: PlanetColor) -> PlanetTrail {
        PlanetTrail {pos, radius, color}
    }

    pub fn update(&mut self) {
        self.radius -= 1.0 * delta_t ;
        self.color.a -= (10.0 * delta_t) as u8;
    }

    pub fn render(&self, px: &mut Pixels) {
        for y in ((self.pos.y-self.radius) as usize)..((self.pos.y+self.radius) as usize) {
                    for x in ((self.pos.x-self.radius) as usize)..((self.pos.x+self.radius) as usize) {
                        let circle_check = (x as f32 - self.pos.x).powf(2.0) + (y as f32 - self.pos.y).powf(2.0);
                        if (circle_check < self.radius.powf(2.0)) || (circle_check == self.radius.powf(2.0)) {
                            let index = y * WIDTH * 4 + x * 4 as usize;
                            if index >= (WIDTH * HEIGHT * 4) {
                                continue;
                            }
                            if x < 0 || x > WIDTH {
                                continue;
                            } 
                            if y < 0 || y > HEIGHT {
                                continue;
                            }
                            let r = self.color.r as u8;
                            let g = self.color.g as u8;
                            let b = self.color.b as u8;
                            px.frame_mut()[index..index+4].copy_from_slice(vec![r, g, b, 255u8].as_slice());
                        }
                    }
                }
           }


}
