#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import time
import testchannels

def safe_send(_values) -> bool:
    try:
        testchannels.send_value_py(_values);
    except ValueError:
        return False
    except OverflowError:
        return False
    except TypeError:
        return False
    return True

print(safe_send((1.0, 4325)));

safe_send((3.0, 0));

time.sleep(1.0)


result = testchannels.receive_value_py();
print(result)
result = testchannels.receive_value_py();
print(result)
result = testchannels.receive_value_py();
print(result)
safe_send((65.0, 255));
result = testchannels.receive_value_py();
print(result)

g = time.time()
h = 0
for i in range(255):
    safe_send((i / 10.0, i))
    myfloat, my_int =testchannels.receive_value_py()
    h += my_int
ending = time.time()
print(h)
print((ending-g)/10)
time.sleep(1.0)
print("now going to send bad data")
time.sleep(1.0)


success = safe_send((65.0, "hi"))
if success:
    myfloat, my_int =testchannels.receive_value_py()
    print(f"{myfloat}\t{my_int}")
print(success)
