extern crate phf_codegen;
extern crate rusqlite;
use rusqlite::Connection;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

const GENERATED_FILE: &'static str = "src/ostn02.rs";

#[derive(Debug)]
struct Shift {
    key: String,
    eastings: f64,
    northings: f64,
    height: f64,
}

fn main() {
    let conn = Connection::open("src/OSTN02.db").unwrap();

    let mut outfile = BufWriter::new(File::create(GENERATED_FILE).unwrap());
    write!(outfile,
           "static OSTN02: phf::Map<&'static str, (i32, i32, i32)> = ")
        .unwrap();

    let mut stmt = conn.prepare("SELECT key, eastings_offset, northings_offset, height_offset \
                                 FROM ostn02")
                       .unwrap();
    let ostn02_iter = stmt.query_map(&[], |row| {
                                  Shift {
                                      key: row.get(0),
                                      eastings: row.get(1),
                                      northings: row.get(2),
                                      height: row.get(3),
                                  }
                              })
                              .unwrap();

    let mut keys = vec![];
    let mut values = vec![];

    for each in ostn02_iter {
        let record = each.unwrap();
        keys.push(record.key);
        values.push((record.eastings,
                     record.northings,
                     record.height));
    }
    let results: Vec<_> = keys.iter().zip(values.iter()).collect();
    let mut map = phf_codegen::Map::<&str>::new();
    for &(ref key, val) in &results {
        map.entry(key, &format!("{:?}", val));
    }
    map.build(&mut outfile).unwrap();
    writeln!(outfile, ";").unwrap();
}
