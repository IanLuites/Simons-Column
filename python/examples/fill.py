#!/usr/bin/env python3

# Fix import path for examples.
if True:
    import sys
    import os

    sys.path.insert(1, os.path.join(sys.path[0], ".."))

# Import lights lib
import lights
import time

LIGHTS = 24

# Connect to lights
column = lights.connect_to_emulator(lights=LIGHTS)

# Create a new pattern (all lights off)
pattern = column.pattern()

# Write the pattern to the lights
column.set(pattern)

# Turn lights on (might have already been on)
column.on()

# Start running lights
for x in range(LIGHTS):
    last = x
    for y in range(x, LIGHTS):
        pattern[last] = False
        pattern[y] = True
        last = y

        column.set(pattern)
        time.sleep(0.01)

    for y in range(LIGHTS - x):
        pattern[last] = False
        last = (LIGHTS - 1) - y
        pattern[last] = True

        column.set(pattern)
        time.sleep(0.01)
