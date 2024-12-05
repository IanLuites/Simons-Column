from .bindings import Rust
import ctypes

RUST_LIB = Rust.load()

RUST_LIB.controller_connect.argtypes = [Rust.USIZE]
RUST_LIB.controller_connect.restype = Rust.POINTER

RUST_LIB.controller_write.argtypes = [Rust.POINTER, Rust.U64]

RUST_LIB.controller_on.argtypes = [Rust.POINTER]
RUST_LIB.controller_off.argtypes = [Rust.POINTER]

RUST_LIB.controller_free.argtypes = [Rust.POINTER]

RUST_LIB.controller_test.argtypes = [
    Rust.POINTER,
    ctypes.c_uint64,
]

RUST_LIB.controller_loop.argtypes = [
    Rust.POINTER,
    ctypes.c_uint8,
    ctypes.c_uint8,
    ctypes.c_uint8,
]


class Controller:
    def __init__(self, chain=3) -> None:
        self.ptr = RUST_LIB.controller_connect(chain)

        if not self.ptr:
            raise MemoryError("Failed to create Rust Controller")

    def __del__(self):
        if hasattr(self, "ptr") and self.ptr:
            RUST_LIB.controller_free(self.ptr)
            self.ptr = None

    def on(self):
        RUST_LIB.controller_on(self.ptr)

    def off(self):
        RUST_LIB.controller_off(self.ptr)

    def write(self, data):
        RUST_LIB.controller_write(self.ptr, data)

    def loop(self, on, off, loops):
        RUST_LIB.controller_loop(self.ptr, on, off, loops)

    def test(self, loops):
        RUST_LIB.controller_test(self.ptr, loops)
