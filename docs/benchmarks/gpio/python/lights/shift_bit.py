#!/usr/bin/env python3

import sys
import time

# Settings

WARMUP = int(sys.argv[1].replace("_", ""))
BENCHMARK = int(sys.argv[2].replace("_", ""))
BIT = int(sys.argv[3] if len(sys.argv) > 3 else "1")

DATA_PIN = int(sys.argv[4] if len(sys.argv) > 4 else "17")
CLOCK_PIN = int(sys.argv[5] if len(sys.argv) > 5 else "22")
LATCH_PING = int(sys.argv[6] if len(sys.argv) > 6 else "27")

# Setup

import lights

register = lights.TPIC6C596(lights.RPiPins())

# Warmup

if BIT == 0:
    warmup_start = time.process_time_ns()
    for _ in range(WARMUP):
        register.shift_low()
    warmup_finish = time.process_time_ns()
else:
    warmup_start = time.process_time_ns()
    for _ in range(WARMUP):
        register.shift_high()
    warmup_finish = time.process_time_ns()

# Benchmark

if BIT == 0:
    benchmark_start = time.process_time_ns()
    for _ in range(BENCHMARK):
        register.shift_low()
    benchmark_finish = time.process_time_ns()
else:
    benchmark_start = time.process_time_ns()
    for _ in range(BENCHMARK):
        register.shift_high()
    benchmark_finish = time.process_time_ns()


print("warmup:", (warmup_finish - warmup_start))
print("benchmark:", (benchmark_finish - benchmark_start))
