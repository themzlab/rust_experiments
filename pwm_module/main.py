#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import time
import threading
import multiprocessing

import pwm_module

# Define 100 duty cycle values (as percentages, e.g., 0.5 for 50%)
duty_cycles = [0.1, 0.2, 0.3, .4, .5, .6, .7, .9, .95]  # Populate this list with your values

# Start the PWM control loop
def main():
    print("Starting")
    pwm_module.start_pwm(duty_cycles, sleep_ms=10)
    print("this prints because Rust spawns a new thread")

def slow():
    print("Starting slow pwm")
    pwm_module.start_pwm([0.50, 0.10, 0.90], 2000)
    print("this prints because Rust spawns a new thread")

# _main = threading.Thread(target=main, daemon=True)
# _main.start()
main()
time.sleep(10)
print("about to stop the pwm")
pwm_module.stop_pwm()
print("stay stopped for 5 seconds")
time.sleep(5.0)
# _main = threading.Thread(target=slow).start()
slow()

time.sleep(10)
pwm_module.stop_pwm()

print("end")
