#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import time
import threading

import testchannels

def safe_send(_values) -> bool:
    if len(_values) == 2:
        _values = (_values[0], _values[1], (4.5, 6.7, 1.0))

    
    try:
        testchannels.send_value_py(_values);
    except ValueError:
        return False
    except OverflowError:
        return False
    except TypeError:
        return False
    return True

print(safe_send((50000, 4325)));

safe_send((50000, 0));

time.sleep(1.0)


result = testchannels.receive_value_py();
print(result)
result = testchannels.receive_value_py();
print(result)
result = testchannels.receive_value_py();
print(result)
safe_send((50000, 255));
result = testchannels.receive_value_py();
print(result)

g = time.time()
h = 0
for i in range(255):
    safe_send((int(i * 10.0), i))
    myfloat, my_int, _mylist =testchannels.receive_value_py()
    h += my_int
print(_mylist)

ending = time.time()
print(h)
print((ending-g)/10)
time.sleep(1.0)
print("now going to send bad data")
time.sleep(1.0)


success = safe_send((500000, "hi"))
if success:
    myfloat, my_int, _mylist =testchannels.receive_value_py()
    print(f"{myfloat}\t{my_int}{_mylist}")
print(success)

print(f"is module started {testchannels.is_module_started()}")
# initialized here as required
testchannels.initialize_module()
print(f"is module started {testchannels.is_module_started()}")
print(f"exit reqeust status {testchannels.get_exit_request_status()}")
print("go there ----")
success = safe_send((100000, 3))
time.sleep(1.0)
print("go there ---- again")
print(testchannels.get_shared_bool())
print(testchannels.get_shared_bool())
time.sleep(1.0)
# initialized here as an error on purpose - to test the behavior
testchannels.initialize_module()

testchannels.set_shared_bool(True)
time.sleep(1.0)
print(testchannels.get_shared_bool())
testchannels.set_shared_bool(False)
time.sleep(2.0)
print(f"exit request status is {testchannels.get_exit_request_status()}")
print(f"error code is {testchannels.get_error_code()}")
testchannels.clear_error_code()
print(f"error code is {testchannels.get_error_code()}")

success = safe_send((100000, 3))
if success:
    testchannels.set_exit_request(True)

print(f"exit request status is {testchannels.get_exit_request_status()}")
print("just told the thread to exit, waiting 3 seconds now")
time.sleep(3.0)
print(testchannels.get_shared_bool())
print(f"module is started {testchannels.is_module_started()}")
print(f"exit request status {testchannels.get_exit_request_status()}")
print(f"module is started and running {testchannels.is_module_started_and_active()}")