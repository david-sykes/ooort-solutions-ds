// Tutorial: Radar
// Destroy the enemy ships. Use your radar to find them.
// Hint: Press 'g' in-game to show where your radar is looking.
// Hint: Press 'n' to single-step.
// Hint: Use the set_radar_heading() function to keep your radar pointed at a
// target, or to search for a new one.
//
// Join the Discord at https://discord.gg/vYyu9EhkKH for Oort discussion and
// tournament results.
use oort_api::prelude::*;

const BULLET_SPEED: f64 = 1000.0; // m/s


pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    pub fn shoot_at_moving_target(&mut self){
        
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

    pub fn tick(&mut self) {
        set_radar_heading(radar_heading() + radar_width()/10.0);
        if let Some(contact) = scan() {
            let ship_vector = contact.position - position();
            let ship_heading = ship_vector.angle();
            set_radar_heading(ship_heading);
            turn(
            200.0 * angle_diff(
                heading(), 
                ship_heading
                )
            );
            // accelerate(0.1 * (contact.position - position()));
            self.shoot_at_moving_target();
        }
    }
}
