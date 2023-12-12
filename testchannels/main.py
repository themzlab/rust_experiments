#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import time
import testchannels

testchannels.send_value_py(4325);
testchannels.send_value_py(0);

time.sleep(1.0)

result = testchannels.receive_value_py();
print(result)
result = testchannels.receive_value_py();
print(result)
result = testchannels.receive_value_py();
print(result)
testchannels.send_value_py(-1);
result = testchannels.receive_value_py();
print(result)

g = time.time()
h = 0
for i in range(10_000):
    testchannels.send_value_py(i)
    h+=testchannels.receive_value_py()
ending = time.time()
print(h)
print((ending-g)/10)

