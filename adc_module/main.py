#!/usr/bin/env python3
import time
from adc_module import AdcModule
m = AdcModule("my_test", 35000, 101.0)
m.begin_reading()
try:
    m.begin_thread()
except AttributeError:
    print("---------------------------------begin_thread is not implemented")
m.test()
time.sleep(20)
m.test()
