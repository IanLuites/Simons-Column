# Python

## Quick Start

```python
# Import lights lib
import lights
import time

# Connect to lights
column = lights.connect(lights = 24)

# Create a new pattern (all lights off)
pattern = column.pattern()

# Turn light 4 and 14 on
pattern[3] = True
pattern[13] = True

# Write the pattern to the lights
column.set(pattern)

# Turn lights on (might have already been on)
column.on()

# Sleep for 0.5 a second
time.sleep(0.5)

# Shift light pattern to the left
pattern.shift_left()
column.set(pattern)

time.sleep(0.5)

# Shift light pattern back to the right
pattern.shift_right()
column.set(pattern)
```
