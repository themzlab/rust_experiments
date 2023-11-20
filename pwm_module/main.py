import pwm_module

# Define 100 duty cycle values (as percentages, e.g., 0.5 for 50%)
duty_cycles = [0.1, 0.2, 0.3, .4, .5, .6, .7, .9, .95]  # Populate this list with your values

# Start the PWM control loop
pwm_module.start_pwm(duty_cycles)
