import time
from adc_module import AdcModule
m = AdcModule("my_test")
m.test()
time.sleep(5)
m.test()
