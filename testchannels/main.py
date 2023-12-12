#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import time
import testchannels

testchannels.send_value_py((1.0, 4325));
testchannels.send_value_py((3.0, 0));

time.sleep(1.0)

result = testchannels.receive_value_py();
print(result)
result = testchannels.receive_value_py();
print(result)
result = testchannels.receive_value_py();
print(result)
testchannels.send_value_py((65.0, -1));
result = testchannels.receive_value_py();
print(result)

g = time.time()
h = 0
for i in range(10_000):
    testchannels.send_value_py((i / 10.0, i))
    myfloat, my_int =testchannels.receive_value_py()
    h += my_int
ending = time.time()
print(h)
print((ending-g)/10)

