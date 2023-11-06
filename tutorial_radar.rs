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
// 
    pub fn adjust_torque_to_target_angle(&mut self, aim_vector: Vec2){
        debug!("angular velocity: {}", angular_velocity());
        let required_rotation = angle_diff(heading(), aim_vector.angle());
        debug!("Required rotation: {}", required_rotation);
        let required_slow_down_torque = (-1.0 * angular_velocity().powf(2.0)) / (2.0 * required_rotation);
        debug!("Required slow down torque: {}", required_slow_down_torque);


        if required_slow_down_torque.abs() > max_angular_acceleration() - 0.5 {
                debug!("Torque set to: {}", required_slow_down_torque);
                torque(required_slow_down_torque);
                }
        else {
                debug!("Torque set to: {}", required_rotation.signum() * max_angular_acceleration());
                torque(required_rotation.signum() * max_angular_acceleration());
        }
        
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


        // Add aim line
        draw_line(position(), position() + aim, 0xff0000);

        // Calculate angle between heading and the aim vector
        self.adjust_torque_to_target_angle(aim);
        fire(0);
    }

    pub fn tick(&mut self) {
        set_radar_width(2.0*PI / 100.0);
        set_radar_heading(radar_heading() - radar_width());
        if let Some(contact) = scan() {
            let ship_vector = contact.position - position();
            let ship_heading = ship_vector.angle();
            set_radar_heading(ship_heading);
            self.shoot_at_moving_target();
        }
    }
}
