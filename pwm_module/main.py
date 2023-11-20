#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import time
import threading

import pwm_module

# Define 100 duty cycle values (as percentages, e.g., 0.5 for 50%)
duty_cycles = [0.1, 0.2, 0.3, .4, .5, .6, .7, .9, .95]  # Populate this list with your values
duty_cycles2 = [0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.502884615384615, 0.50676923076923, 0.511653846153846, 0.517538461538461, 0.524423076923076, 0.532307692307692, 0.541192307692307, 0.551076923076922, 0.561961538461538, 0.573846153846153, 0.586730769230769, 0.600615384615384, 0.6055, 0.600615384615384, 0.586730769230769, 0.573846153846153, 0.561961538461538, 0.551076923076923, 0.541192307692308, 0.532307692307692, 0.524423076923077, 0.517538461538462, 0.511653846153846, 0.506769230769231, 0.502884615384616]

frequency=20000.0
# Start the PWM control loop
def main():
    pwm_module.start_pwm(duty_cycles2, sleep_ms=5, modulation_hz=frequency)
    print("this prints ~immediately because Rust spawns a new thread")

def slow():
    pwm_module.start_pwm([0.50, 0.10, 0.90], sleep_ms=2000, modulation_hz=frequency)
    print("this prints ~immediately because Rust spawns a new thread")

# _main = threading.Thread(target=main, daemon=True)
# _main.start()
print("Starting for 20 seconds")
main()
time.sleep(20)
print("about to stop the pwm")
pwm_module.stop_pwm()
print("stay stopped for 5 seconds")
time.sleep(5.0)
# _main = threading.Thread(target=slow).start()
print("starting a slower modulation")
slow()

time.sleep(10)
pwm_module.stop_pwm()

print("end")
