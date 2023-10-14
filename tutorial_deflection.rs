// Tutorial: Deflection
// Destroy the enemy ship. Its position is given by the "target" function and velocity by the
// "target_velocity" function.
//
// Hint: p = p₀ + v₀t + ½at² (the third equation of kinematics)
//
// p.s. You can change your username by clicking on it at the top of the page.
use oort_api::prelude::*;

const BULLET_SPEED: f64 = 950.0; // m/s

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }
    
    pub fn tick(&mut self) {
        let dot_prod = target().dot(target_velocity());
        let time = 
        (
            2.0*dot_prod + (
            (2.0*dot_prod).powf(2.0) 
            - 
            (4.0 * 
            (BULLET_SPEED.powf(2.0) - target_velocity().length().powf(2.0)) * 
                -(target().length()).powf(2.0)
            )
                ).sqrt()
        ) / 
        (2.0*(BULLET_SPEED.powf(2.0) - target_velocity().length().powf(2.0)));

        // Calculate the aim vector
        let aim = (target() / time ) + target_velocity();

        turn(
           10.0 * angle_diff(
                heading(), 
                    aim.angle()
           )

        );
        fire(0);
    }
}
