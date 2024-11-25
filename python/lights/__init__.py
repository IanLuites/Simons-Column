"""
Lights helpers.
"""

import sys

if sys.version_info[0] < 3:
    raise Exception("Python 3 is required.")

import socket as sockets
import time
from typing import Final, Literal, Protocol
from abc import abstractmethod


class Pins(Protocol):
    @abstractmethod
    def set_data(self, on: Literal[0, 1]):
        raise NotImplementedError

    @abstractmethod
    def set_clock(self, on: Literal[0, 1]):
        raise NotImplementedError

    @abstractmethod
    def set_latch(self, on: Literal[0, 1]):
        raise NotImplementedError

    @abstractmethod
    def set_control(self, on: Literal[0, 1]):
        raise NotImplementedError

    @abstractmethod
    def get_control(self) -> Literal[0, 1]:
        raise NotImplementedError


PI_SUPPORTED = False
try:
    import RPi.GPIO as GPIO

    PI_SUPPORTED = True

    class RPiPins(Pins):
        def __init__(
            self,
            data_pin: int = 17,
            latch_pin: int = 27,
            clock_pin: int = 22,
            control_pin: int = 23,
            chain: int = 1,
            mode: Literal[10, 11] = GPIO.BCM,
        ):
            """
            Initialize the RPi GPIO pins for controlling the lights.

            Args:
              data_pin (int): The GPIO pin number for data. Default is 17.
              latch_pin (int): The GPIO pin number for latch. Default is 27.
              clock_pin (int): The GPIO pin number for clock. Default is 22.
              control_pin (int): The GPIO pin number for control. Default is 23.
              mode (Literal[10, 11]): The GPIO mode to use. Default is GPIO.BCM.

            """
            self.data_pin = data_pin
            self.latch_pin = latch_pin
            self.clock_pin = clock_pin
            self.control_pin = control_pin
            self.chain = chain

            GPIO.setmode(mode)
            GPIO.setup(self.data_pin, GPIO.OUT)
            GPIO.setup(self.latch_pin, GPIO.OUT)
            GPIO.setup(self.clock_pin, GPIO.OUT)
            GPIO.setup(self.control_pin, GPIO.OUT)

        def set_clock(self, on: Literal[0] | Literal[1]):
            GPIO.output(self.data_pin, on)

        def set_data(self, on: Literal[0] | Literal[1]):
            GPIO.output(self.data_pin, on)

        def set_control(self, on: Literal[0] | Literal[1]):
            GPIO.output(self.control_pin, on)

        def set_latch(self, on: Literal[0] | Literal[1]):
            GPIO.output(self.latch_pin, on)

        def get_control(self) -> Literal[0] | Literal[1]:
            return GPIO.input(self.control_pin)

except ImportError:
    import warnings

    warnings.warn("RPi not supported on this platform.")


class TPIC6C596:
    """
    A class to interface with the TPIC6C596 shift register using GPIO pins.

    Attributes:
      data_pin (int): GPIO pin connected to the data input of the shift register.
      latch_pin (int): GPIO pin connected to the latch input of the shift register.
      clock_pin (int): GPIO pin connected to the clock input of the shift register.
      control_pin (int): GPIO pin connected to the control input of the shift register.
      chain (int): Number of chained shift registers.
    """

    BITS_PER_CHAIN: Final = 8

    def __init__(
        self,
        pins: Pins,
        chain: int = 1,
    ):
        """
        Initialize the GPIO pins for controlling the lights.

        Args:
          chain (int): The number of chained devices. Default is 1.

        """
        self.pins = pins
        self.chain = chain

    def is_on(self) -> bool:
        """
        Check if the lights are currently on.

        Returns:
          bool: True if the lights are on, False otherwise.
        """
        return self.pins.get_control()

    def on(self):
        """
        Turns the lights on by setting the control pin to HIGH.
        """
        self.pins.set_control(1)

    def off(self):
        """
        Turns off the lights by setting the control pin to LOW.
        """
        self.pins.set_control(0)

    def shift_high(self):
        """
        Shifts the bits of the register[s] to the left by one high bit.
        """
        self.shift_bits(1, 1)

    def shift_low(self):
        """
        Shifts the bits of the register[s] to the left by one low bit.
        """
        self.shift_bits(0, 1)

    def shift_bits(self, data: int, count: int):
        """
        Shifts the given number of bits from the data to the shift register[s].

        Args:
          data (int): The data to be shifted out, bit by bit.
          count (int): The number of bits to shift.
        """
        self.pins.set_latch(0)

        for _ in range(count):
            self.pins.set_clock(0)
            self.pins.set_data(data & 1)
            self.pins.set_clock(1)
            data >>= 1

        self.pins.set_latch(1)
        self.pins.set_latch(0)

    def write(self, data: int):
        """
        Writes the given data to the shift register[s].

        Args:
          data (int): The data to be written to the shift register.
        """
        self.shift_bits(data, self.chain * self.BITS_PER_CHAIN)


class Pattern:
    """
    A class to represent a pattern of lights.

    Attributes:
      lights (int): The number of lights in the pattern.
      mask (int): Bitmask for truncating the pattern data.
      data (int): The binary representation of the lights' states.
    """

    def __init__(
        self,
        lights: int,
        state: bool = False,
    ):
        """
        Initialize the lights object.

        Args:
          lights (int): The number of lights.
          state (bool, optional): The initial state of the lights. Defaults to False.
        """

        self.lights = lights
        self.bitmask = (2**self.lights) - 1
        self.data = self.bitmask if state else 0

    def shift_left(self, count=1):
        """
        Shift the light pattern to the left by a specified number of positions.

        This method shifts the bits in `self.data` to the left by `count` positions.
        Bits that overflow on the left are wrapped around to the right side.

        Args:
          count (int, optional): The number of positions to shift the bits to the left. Defaults to 1.
        """
        overflow = self.data >> (self.lights - count)
        self.data = ((self.data << count) | overflow) & self.bitmask

    def shift_right(self, count=1):
        """
        Shift the light pattern to the right by a specified number of positions.

        This method shifts the bits in `self.data` to the right by `count` positions.
        Bits that overflow on the right are wrapped around to the left side.

        Args:
          count (int, optional): The number of positions to shift the bits to the right. Defaults to 1.
        """
        underflow = (self.data & ((2**count) - 1)) << (self.lights - count)
        self.data = (self.data >> count) | underflow

    def __setitem__(self, index, value: int | bool) -> bool:
        if index >= self.lights:
            raise RuntimeError(
                f"Trying to access light #{index + 1} with index {index}, but only {self.lights} lights connected."
            )

        if value:
            self.data |= 1 << index
        else:
            self.data &= ~(1 << index)

    def __getitem__(self, index) -> bool:
        if index >= self.lights:
            raise RuntimeError(
                f"Trying to access light #{index + 1} with index {index}, but only {self.lights} lights connected."
            )
        return self.data & 1 << index != 0

    def __str__(self) -> str:
        return f"Pattern({{0:0{self.lights}b}})".format(self.data)


class Lights:
    """
    A class to control a set of lights using TPIC6C596 shift registers.

    Attributes:
      lights (int): The number of lights to control.
      shift_register (TPIC6C596): The shift register used to control the lights.
    """

    def __init__(
        self,
        shift_register: TPIC6C596,
        lights: int,
    ):
        """
        Initializes the light control with the given shift register and number of lights.
        Args:
          shift_register (TPIC6C596): The shift register used to control the lights.
          lights (int): The number of lights to be controlled.
        Raises:
          RuntimeError: If the number of lights exceeds the maximum capacity of the shift registers.
        """

        max_lights = shift_register.chain * shift_register.BITS_PER_CHAIN
        if lights > max_lights:
            raise RuntimeError(f"""
                               Too many lights to for the amount of TPIC6C596 shift registers.

                               Shift Registers:         {shift_register.chain}
                               Max Lights:              {max_lights}
                               Given amount of lights:  {lights}
                               """)

        self.lights = lights
        self.shift_register = shift_register

    def is_on(self) -> bool:
        """
        Check if the lights are currently on.

        Returns:
          bool: True if the lights are on, False otherwise.
        """
        return self.shift_register.is_on()

    def on(self):
        """
        Turns the lights on by setting the control pin to HIGH.
        """
        self.shift_register.on()

    def off(self):
        """
        Turns off the lights by setting the control pin to LOW.
        """
        self.shift_register.off()

    def set(self, pattern: Pattern):
        """
        Set the light pattern using the provided Pattern object.
        Args:
          pattern (Pattern): An instance of the Pattern class containing the data to be written to the shift register.
        """

        self.shift_register.write(pattern.data)

    def pattern(self, state: bool = False) -> Pattern:
        """
        Create a Pattern object with the current lights and the specified state.
        Args:
          state (bool): The state to set for the pattern. Defaults to False.
        Returns:
          Pattern: A Pattern object initialized with the current lights and the specified state.
        """

        return Pattern(self.lights, state=state)


if PI_SUPPORTED:

    def connect(
        lights: int,
        data_pin: int = 17,
        latch_pin: int = 27,
        clock_pin: int = 22,
        control_pin: int = 23,
        mode: Literal[10, 11] = GPIO.BCM,
        chain: int | None = None,
    ) -> Lights:
        """
        Connects to a series of lights using a shift register.

        Args:
          lights (int): The number of lights to control.
          data_pin (int, optional): The GPIO pin connected to the data input of the shift register. Defaults to 17.
          latch_pin (int, optional): The GPIO pin connected to the latch input of the shift register. Defaults to 27.
          clock_pin (int, optional): The GPIO pin connected to the clock input of the shift register. Defaults to 22.
          control_pin (int, optional): The GPIO pin connected to the control input of the shift register. Defaults to 23.
          mode (Literal[10, 11], optional): The GPIO mode to use. Defaults to GPIO.BCM.
          chain (int | None, optional): The number of chained shift registers. If None, it is calculated as lights // 8. Defaults to None.

        Returns:
          Lights: An instance of the Lights class configured with the specified shift register and number of lights.
        """
        shift_register: TPIC6C596 = TPIC6C596(
            pins=RPiPins(
                data_pin=data_pin,
                latch_pin=latch_pin,
                clock_pin=clock_pin,
                control_pin=control_pin,
                mode=mode,
            ),
            chain=chain or lights // 8,
        )

        return Lights(shift_register=shift_register, lights=lights)


# Check whether unix sockets are supported.
if hasattr(sockets, "AF_UNIX"):

    class Emulator(Pins):
        def __init__(self, socket: str):
            self.socket = sockets.socket(sockets.AF_UNIX, sockets.SOCK_DGRAM)
            self.address = socket
            self.control = 0

        def set_clock(self, on: Literal[0] | Literal[1]):
            self.socket.sendto(bytes([on << 7 | 3]), self.address)

        def set_data(self, on: Literal[0] | Literal[1]):
            self.socket.sendto(bytes([on << 7 | 1]), self.address)

        def set_control(self, on: Literal[0] | Literal[1]):
            self.control = on
            self.socket.sendto(bytes([on << 7 | 2]), self.address)

        def set_latch(self, on: Literal[0] | Literal[1]):
            self.socket.sendto(bytes([on << 7 | 4]), self.address)

        def get_control(self) -> Literal[0] | Literal[1]:
            return self.control

    def connect_to_emulator(
        lights: int,
        socket: str = "/tmp/tpic6c596-emulator.sock",
        chain: int | None = None,
    ) -> Lights:
        shift_register: TPIC6C596 = TPIC6C596(
            pins=Emulator(socket=socket),
            chain=chain or lights // 8,
        )

        return Lights(shift_register=shift_register, lights=lights)
