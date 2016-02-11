//! Look up OSTN02 adjustments for transforming ETRS89 Eastings and Northings
//! to OSGB36 Eastings and Northings
//!

const MIN_X_SHIFT: f64 = 86.275;
const MIN_Y_SHIFT: f64 = -81.603;
const MIN_Z_SHIFT: f64 = 43.982;

extern crate phf;
include!("ostn02.rs");

extern crate libc;
use libc::{c_void, c_double, c_int};

fn get_shifts(tup: (i32, i32)) -> (c_double, c_double, c_double) {
    // look up the shifts, or return 0.0
    let key = format!("{:03x}{:03x}", tup.1, tup.0);
    // some or None, so try! this
    match ostn02_lookup(&*key) {
        Some(res) => {
            (res.0 as c_double / 1000. + MIN_X_SHIFT,
             res.1 as c_double / 1000. + MIN_Y_SHIFT,
             res.2 as c_double / 1000. + MIN_Z_SHIFT)
        }
        None => (9999.0 as c_double, 9999.0 as c_double, 9999.0 as c_double),
    }
}


#[repr(C)]
/// Incoming kilometer-grid references
pub struct GridRefs {
    pub easting: c_int,
    pub northing: c_int,
}

#[repr(C)]
/// Outgoing shifts
pub struct Adjustment {
    pub x_shift: c_double,
    pub y_shift: c_double,
    pub z_shift: c_double,
}

impl From<(i32, i32)> for GridRefs {
    fn from(gridref: (i32, i32)) -> GridRefs {
        GridRefs {
            easting: gridref.0,
            northing: gridref.1,
        }
    }
}

impl From<Adjustment> for (f64, f64, f64) {
    fn from(adj: Adjustment) -> (f64, f64, f64) {
        (adj.x_shift, adj.y_shift, adj.z_shift)
    }
}

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
pub fn ostn02_lookup(key: &str) -> Option<(i32, i32, i32)> {
    if key.is_empty() {
        return None;
    }
    OSTN02.get(&*key).cloned()
}
