// Tutorial: Lead
// Destroy the enemy ship. Its position is given by the "target" function and velocity by the
// "target_velocity" function. Your ship is not able to accelerate in this scenario.
//
// This is where the game becomes challenging! You'll need to lead the target
// by firing towards where the target will be by the time the bullet gets there.
//
// Hint: target() + target_velocity() * t gives the position of the target after t seconds.
//
// You can scale a vector by a number: vec2(a, b) * c == vec2(a * c, b * c)
//
// p.s. You can change your username by clicking on it at the top of the page.
use oort_api::prelude::*;

const BULLET_SPEED: f64 = 950.0; // m/s I had to fiddle this

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    pub fn tick(&mut self) {
        draw_line(position(), target(), 0x00ff00);
        let dp = target() - position();
        let time_to_target = dp.length() / BULLET_SPEED;
        let dot_prod = target().dot(target_velocity());

        // Calculate the time that the target and bullet arrive at same place
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

        debug!("distance to target: {}", dp.length());
        debug!("Time: {}", time);
        debug!("Heading: {}", heading());
        debug!("Aim angle: {}", aim.angle());
        debug!("velocity: {}", target_velocity().length());
        debug!("time to target: {}", dp.length() / BULLET_SPEED);

        // Add aim line
        draw_line(position(), position() + aim, 0xff0000);

        // Calculate angle between heading and the aim vector
        turn(
            10.0 * angle_diff(
                heading(), 
                    aim.angle()
                )
            );
        fire(0);
    }
}
