extern crate phf_codegen;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

const GENERATED_FILE: &'static str = "src/ostn02.rs";


fn main() {
    let keys = vec!["13928b", "13928c", "13a28b", "13a28c"];
    let values: Vec<(_, _, _)> = vec![(16500, 3359, 270),
                                          (16538, 3357, 254),
                                          (16508, 3387, 258),
                                          (16547, 3376, 242)];
    // let ostn02 = keys.drain(..).zip(values.drain(..)).collect::<HashMap<_, (_, _, _)>>();




    let ostn02: Vec<_> = keys.iter().zip(values.iter())
        // .map(|&(k, v)| (k, v))
        .collect();

    let mut outfile = BufWriter::new(File::create(GENERATED_FILE).unwrap());

    write!(outfile, "static OSTN02: phf::Map<&'static str, (i32, i32, i32)> = ").unwrap();    
    
    let mut map = phf_codegen::Map::<&str>::new();

    for &(ref key, val) in &ostn02 {
        map.entry(key, &format!("{:?}", val));
    }

    map.build(&mut outfile).unwrap();

    writeln!(outfile, ";").unwrap();
}