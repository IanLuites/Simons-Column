#!/usr/bin/env python3

import sys
import time

# Settings

WARMUP = int(sys.argv[1].replace("_", ""))
BENCHMARK = int(sys.argv[2].replace("_", ""))
DATA = int(sys.argv[3] if len(sys.argv) > 3 else "43775")

DATA_PIN = int(sys.argv[4] if len(sys.argv) > 4 else "17")
CLOCK_PIN = int(sys.argv[5] if len(sys.argv) > 5 else "22")
LATCH_PING = int(sys.argv[6] if len(sys.argv) > 6 else "27")

# Setup

import RPi.GPIO as GPIO

GPIO.setmode(GPIO.BCM)
for pin in [CLOCK_PIN, DATA_PIN, LATCH_PING]:
    GPIO.setup(pin, GPIO.OUT)
    GPIO.output(pin, GPIO.LOW)

# Warmup

warmup_start = time.process_time_ns()
for _ in range(WARMUP):
    data = DATA
    for _ in range(24):
        GPIO.output(CLOCK_PIN, GPIO.LOW)
        GPIO.output(DATA_PIN, data & 1)
        GPIO.output(CLOCK_PIN, GPIO.HIGH)
        data >>= 1

    GPIO.output(LATCH_PING, GPIO.HIGH)
    GPIO.output(LATCH_PING, GPIO.LOW)
warmup_finish = time.process_time_ns()

# Benchmark

benchmark_start = time.process_time_ns()
for _ in range(BENCHMARK):
    data = DATA
    for _ in range(24):
        GPIO.output(CLOCK_PIN, GPIO.LOW)
        GPIO.output(DATA_PIN, data & 1)
        GPIO.output(CLOCK_PIN, GPIO.HIGH)
        data >>= 1

    GPIO.output(LATCH_PING, GPIO.HIGH)
    GPIO.output(LATCH_PING, GPIO.LOW)
benchmark_finish = time.process_time_ns()

print("warmup:", (warmup_finish - warmup_start))
print("benchmark:", (benchmark_finish - benchmark_start))
