#!/usr/bin/env python3
import time
from adc_module import AdcModule
m = AdcModule("my_test")
m.begin_reading()
m.test()
time.sleep(20)
m.test()
