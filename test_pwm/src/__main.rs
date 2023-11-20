use rppal::pwm::{Channel, Pwm};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let channel = Channel::Pwm0; // Replace with the desired channel
    let duty_cycle_values = [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0];

    let pwm = Pwm::with_frequency(channel, 50.0)?; // Set PWM frequency to 50 Hz
    pwm.enable();

    loop {
        for duty_cycle in &duty_cycle_values {
            pwm.set_duty_cycle(*duty_cycle)?;
            println!("Set PWM duty cycle to {}%", duty_cycle);
            sleep(Duration::from_millis(10)); // Sleep for 10ms
        }
    }
}
