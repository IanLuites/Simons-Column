import ctypes
import importlib.resources
import sys
import threading


class RustPointer(ctypes.Structure):
    _fields_ = []


class Rust:
    """Rust bindings."""

    POINTER = ctypes.POINTER(RustPointer)
    U64 = ctypes.c_uint64
    USIZE = ctypes.c_uint64 if sys.maxsize > 2**32 else ctypes.c_uint32

    _lib = None
    _lock = threading.Lock()

    @classmethod
    def load(cls):
        with cls._lock:
            if cls._lib is None:
                if sys.platform.startswith("linux"):
                    lib_name = "liblights.so"
                elif sys.platform == "darwin":
                    lib_name = "liblights.dylib"
                elif sys.platform.startswith("win"):
                    lib_name = "liblights.dll"
                else:
                    raise OSError("Unsupported platform")

                with importlib.resources.path(
                    "simons_column.lib", lib_name
                ) as lib_path:
                    lib = ctypes.CDLL(str(lib_path))

                    cls._lib = lib

        return cls._lib
