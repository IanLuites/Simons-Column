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
x = -1
step = 1
while True:
    if x >= 0:
        pattern[x] = False

    x += step
    pattern[x] = True

    if x <= 0:
        step = 1
    elif x >= 23:
        step = -1

    column.set(pattern)
    time.sleep(0.025)
