[![Build Status](https://travis-ci.org/urschrei/OSTN02_PHF.png?branch=master)](https://travis-ci.org/urschrei/OSTN02_PHF) [![](https://img.shields.io/crates/v/lonlat_bng.svg)](https://crates.io/crates/OSTN02_PHF) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](license.txt)  

# Description
A Rust Crate which provides fast lookup of OSTN02 adjustments, for the conversion of ETRS89 grid coordinates to OSGB36.  

The crate provides base shifts. In order to obtain the actual shifts, divide each shift by `1000.`, then subtract the minimum Easting, Northing, and Height shift. All calculations should be carried out using double-precision floating point.

Minimum Easting shift = `86.275`  
Minimum Northing shift = `-81.603`  
Minimum height shift = `43.982`  

Base shifts for `651, 313`: `(16500, 3359, 270)`  
Actual shifts: `(102.775, -78.244, 44.252)`  

The FFI function **does not** require the calculation above; it returns the actual shifts.

# Rust Crate Example
``` rust
// The key is the combined hex-transformed (03x) kilometer-grid reference of the ETRS89 Northings and Eastings coordinates
use ostn02_phf::ostn02_lookup;
// Caister Tower Eastings and Northings: 651307.003, 313255.686
let e_grid = (651307.003 / 1000.) as i32;
let n_grid = (313255.686 / 1000.) as i32;
let key = format!("{:03x}{:03x}", n_grid, e_grid);
// key is 13928b
let result = ostn02_lookup(&*key).unwrap();
// result should be (16500, 3359, 270)
assert_eq!(result, (16500, 3359, 270));
// remember that the actual adjustment for a coordinate is a bilinear transform, using a square
// see ostn02_shifts in https://github.com/urschrei/lonlat_bng/blob/master/src/ostn02/mod.rs
```

# FFI Example
``` python
# using Python
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


prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
lib = ctypes.cdll.LoadLibrary(prefix + "ostn02_phf" + extension)

lib.get_shifts_ffi.argtypes = (GridRefs,)
lib.get_shifts_ffi.restype = Shifts

tup = GridRefs(651, 313)

print lib.get_shifts_ffi(tup)
```

# Building the Shared Library
- Ensure that [Rust](https://www.rust-lang.org/downloads.html) is installed
- Clone this repo
- In the repo root, run `cargo build --release`
- The dylib or DLL will be available as `target/release/libostn02_phf.{dylib, dll}`
- If you need to build a `.so` for Linux:
    1. `ar -x target/release/liblonlat_bng.a`
    2. `gcc -shared *.o -o target/release/libostn02_phf.so`

# License
[MIT](LICENSE)  

This software makes use of OSTN02 data, which is © Crown copyright, Ordnance Survey and the Ministry of Defence (MOD) 2002. All rights reserved. Provided under the BSD 2-clause [license](OSTN02_license.txt).
