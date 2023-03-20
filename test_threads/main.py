#!/usr/bin/env python3
import time
from test_threads import TestThreads
m = TestThreads("my_test")
m.begin_thread()
m.test()
time.sleep(10)
m.test()
