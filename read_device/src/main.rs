use std::io::{self, Read, SeekFrom, Seek};
use std::time::{Duration, Instant};
use std::fs::OpenOptions;

// Rest of your code to read and parse the value


fn main() -> io::Result<()> {
    // Path to the ADC sysfs entry
    // let adc_path =
    //     "/sys/devices/platform/soc/fe804000.i2c/i2c-1/1-0029/iio:device0/in_intensity_ir_raw";

    let device_path: &str = "/sys/devices/platform/soc/fe804000.i2c/i2c-1/1-0029/iio:device0/";
    // let device_path: &str = "/home/pi/";

    // let property_name: &str = "in_illuminance_input";
    let property_name: &str = "in_intensity_both_raw";

    // Open the file for reading
    let mut options: OpenOptions = OpenOptions::new();
    let mut file = options.read(true).open(format!("{device_path}{property_name}")).unwrap();

    // Number of samples to read per second
    let samples_per_second: i32 = 1;

    // Calculate the delay between samples in microseconds
    let sample_delay: Duration = Duration::from_micros(1_000_000 / samples_per_second as u64);

    let mut adc_value_str: String = String::new();
    let mut count: i32 = 0;
    
    loop {
        
        let start: Instant = Instant::now();
        file.read_to_string(&mut adc_value_str)?;
        
        // Parse the value to a u32
        if let Ok(adc_value) = adc_value_str.trim().parse::<u32>() {
            // Print the  value
            println!("ADC Value: {} count {}", adc_value, count);
            adc_value_str = "".to_string();
        } else {
            eprintln!("Failed to parse ADC value: {} count{}", adc_value_str, count);
        }
        file.seek(SeekFrom::Start(0))?;
        let elapsed: Duration = start.elapsed();

        count +=1;
        if let Some(sleep_duration) = sample_delay.checked_sub(elapsed) {
            // only sleep if there is a positive time duration after subtracting elapsed time
            // from the desired interval
            std::thread::sleep(sleep_duration);
        }
    }
}
