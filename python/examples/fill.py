#!/usr/bin/env python3

# Fix import path for examples.
if True:
    import sys
    import os

    sys.path.insert(1, os.path.join(sys.path[0], ".."))

# Import lights lib
import lights
import time

# Connect to lights
column = lights.connect_to_emulator(lights=24)

# Create a new pattern (all lights off)
pattern = column.pattern()

# Write the pattern to the lights
column.set(pattern)

# Turn lights on (might have already been on)
column.on()

# Start running lights
for x in range(24):
    last = x
    for y in range(x, 24):
        pattern[last] = False
        pattern[y] = True
        last = y

        column.set(pattern)
        time.sleep(0.01)

    for y in range(24 - x):
        pattern[last] = False
        pattern[23 - y] = True
        last = 23 - y

        column.set(pattern)
        time.sleep(0.01)
