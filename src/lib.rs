extern crate phf;
include!("ostn02.rs");

#[test]
fn it_works() {
}
pub fn ostn02_lookup(key: &str) -> Option<(i32, i32, i32)>  {
    if key.is_empty() { return None; }
    OSTN02.get(&*key).cloned()  
}
