#!/usr/bin/env python3

import sys
import time

# Settings

WARMUP = int(sys.argv[1].replace("_", ""))
BENCHMARK = int(sys.argv[2].replace("_", ""))
PIN = int(sys.argv[3] if len(sys.argv) > 3 else "17")

# Setup

import RPi.GPIO as GPIO

GPIO.setmode(GPIO.BCM)
GPIO.setup(PIN, GPIO.OUT)
GPIO.output(PIN, GPIO.LOW)

# Warmup

warmup_start = time.process_time_ns()
for _ in range(WARMUP):
    GPIO.output(PIN, GPIO.HIGH)
    GPIO.output(PIN, GPIO.LOW)
warmup_finish = time.process_time_ns()

# Benchmark

benchmark_start = time.process_time_ns()
for _ in range(BENCHMARK):
    GPIO.output(PIN, GPIO.HIGH)
    GPIO.output(PIN, GPIO.LOW)
benchmark_finish = time.process_time_ns()

print("warmup:", (warmup_finish - warmup_start))
print("benchmark:", (benchmark_finish - benchmark_start))
