//! Look up OSTN02 adjustments for transforming ETRS89 Eastings and Northings
//! to OSGB36 Eastings and Northings
//! 
extern crate phf;
include!("ostn02.rs");

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
