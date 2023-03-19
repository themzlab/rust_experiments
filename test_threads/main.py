import time
from test_threads import TestThreads
m = TestThreads("my_test")
m.test()
time.sleep(5)
m.test()