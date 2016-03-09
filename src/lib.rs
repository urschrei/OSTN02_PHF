#![doc(html_root_url = "https://urschrei.github.io/ostn02_phf/")]
//! Look up OSTN02 adjustments for transforming ETRS89 Eastings and Northings
//! to OSGB36 Eastings and Northings

const MIN_X_SHIFT: f64 = 86.275;
const MIN_Y_SHIFT: f64 = -81.603;
const MIN_Z_SHIFT: f64 = 43.982;

use std::f64;
const NAN: f64 = f64::NAN;

extern crate phf;
include!("ostn02.rs");

extern crate libc;
use libc::{c_double, int16_t};

/// Return a 3-tuple of adjustments which convert ETRS89 Eastings and Northings
/// to OSGB36 Eastings, Northings, and Orthometric height
fn get_shifts(tup: (i16, i16)) -> (f64, f64, f64) {
    // look up the shifts, or return NAN
    let key = format!("{:03x}{:03x}", tup.1, tup.0);
    match ostn02_lookup(&*key) {
        Some(res) => {
            (res.0 as f64 / 1000. + MIN_X_SHIFT,
             res.1 as f64 / 1000. + MIN_Y_SHIFT,
             res.2 as f64 / 1000. + MIN_Z_SHIFT)
        }
        None => (NAN, NAN, NAN),
    }
}

#[repr(C)]
/// Incoming ETRS89 kilometer-grid references
pub struct GridRefs {
    pub easting: int16_t,
    pub northing: int16_t,
}

#[repr(C)]
/// Outgoing OSTN02 Easting, Northing, and height adjustments
pub struct Adjustment {
    pub x_shift: c_double,
    pub y_shift: c_double,
    pub z_shift: c_double,
}

// From and Into traits for GridRefs
impl From<(i16, i16)> for GridRefs {
    fn from(gr: (i16, i16)) -> GridRefs {
        GridRefs {
            easting: gr.0,
            northing: gr.1,
        }
    }
}

impl From<GridRefs> for (i16, i16) {
    fn from(gr: GridRefs) -> (i16, i16) {
        (gr.easting, gr.northing)
    }
}

// From and Into traits for Adjustment
impl From<(f64, f64, f64)> for Adjustment {
    fn from(adj: (f64, f64, f64)) -> Adjustment {
        Adjustment {
            x_shift: adj.0,
            y_shift: adj.1,
            z_shift: adj.2,
        }
    }
}

impl From<Adjustment> for (f64, f64, f64) {
    fn from(adj: Adjustment) -> (f64, f64, f64) {
        (adj.x_shift, adj.y_shift, adj.z_shift)
    }
}

/// FFI function returning a 3-tuple of Easting, Northing, and height adjustments, for use in transforming
/// ETRS89 Eastings and Northings to OSGB36 Eastings, Northings.  
/// The argument is a Struct containing kilometer-grid references of the ETRS89 Northings and Eastings you wish to convert
/// 
/// # Examples
/// 
/// ```python
/// # Python example using ctypes
/// import sys, ctypes
/// from ctypes import c_uint32, c_double, Structure
/// 
/// 
/// class GridRefs(Structure):
///     _fields_ = [("eastings", c_uint32),
///                 ("northings", c_uint32)]
/// 
///     def __str__(self):
///         return "({},{})".format(self.eastings, self.northings)
/// 
/// 
/// class Shifts(Structure):
///     _fields_ = [("x_shift", c_double),
///                 ("y_shift", c_double),
///                 ("z_shift", c_double)]
/// 
///     def __str__(self):
///         return "({}, {}, {})".format(self.x_shift, self.y_shift, self.z_shift)
/// 
/// 
/// prefix = {'win32': ''}.get(sys.platform, 'lib')
/// extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
/// lib = ctypes.cdll.LoadLibrary(prefix + "ostn02_phf" + extension)
/// 
/// lib.get_shifts_ffi.argtypes = (GridRefs,)
/// lib.get_shifts_ffi.restype = Shifts
/// 
/// tup = GridRefs(651, 313)
/// 
/// # Should return (102.775, -78.244, 44.252)
/// print lib.get_shifts_ffi(tup)
/// ```
#[no_mangle]
pub extern "C" fn get_shifts_ffi(gr: GridRefs) -> Adjustment {
    get_shifts(gr.into()).into()
}

/// Return a 3-tuple of Easting, Northing, and height adjustments, for use in transforming
/// ETRS89 Eastings and Northings to OSGB36 Eastings, Northings.  
/// The key is the combined hex-transformed (03x) kilometer-grid reference of the Northings and Eastings:
/// 
/// # Examples
/// 
/// ```
/// use ostn02_phf::ostn02_lookup;
/// 
/// // Caister Tower Eastings and Northings: 651307.003, 313255.686
/// let e_grid = (651307.003 / 1000.) as i32;
/// let n_grid = (313255.686 / 1000.) as i32;
/// let key = format!("{:03x}{:03x}", n_grid, e_grid);
/// // key is 13928b
/// let result = ostn02_lookup(&*key).unwrap();
/// // result should be (16500, 3359, 270)
/// assert_eq!(result, (16500, 3359, 270));
/// // remember that the actual adjustment for a coordinate is a bilinear transform, using a square
/// // see ostn02_shifts in https://github.com/urschrei/lonlat_bng/blob/master/src/ostn02/mod.rs
/// ```
pub fn ostn02_lookup(key: &str) -> Option<(i16, i16, i16)> {
    if key.is_empty() {
        return None;
    }
    OSTN02.get(&*key).cloned()
}

#[test]
fn test_internal_ffi() {
    assert_eq!((102.775, -78.244, 44.252), get_shifts((651, 313)));
}

#[test]
fn test_ffi() {
    let gr = GridRefs {easting: 651, northing: 313};
    assert_eq!((102.775, -78.244, 44.252), get_shifts_ffi(gr).into());
}
