import sys, ctypes
from ctypes import c_uint32, c_double, Structure



class GridRefs(Structure):
    _fields_ = [("eastings", c_uint32),
                ("northings", c_uint32)]
    def __str__(self):
        return "({},{})".format(self.eastings, self.northings)



class Shifts(Structure):
    _fields_ = [("x_shift", c_double),
                ("y_shift", c_double),
                ("z_shift", c_double)]
    def __str__(self):
        return "({}, {}, {})".format(self.x_shift, self.y_shift, self.z_shift)



prefix = {"win32": ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
lib = ctypes.cdll.LoadLibrary("../target/release/" + prefix + "ostn02_phf" + extension)
lib.get_shifts_ffi.argtypes = (GridRefs,)
lib.get_shifts_ffi.restype = Shifts
tup = GridRefs(651, 313)
# Should return (102.775, -78.244, 44.252)
print lib.get_shifts_ffi(tup)
