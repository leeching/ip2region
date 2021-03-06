extern crate ip2region;

use ip2region::*;

use std::env;
use std::time::Instant;

// cargo run --release ../../../data/ip2region.db

static IPS: &'static [&'_ str] = &[
    "117.136.105.202",
    "47.95.47.253",
    "127.0.0.1",
    "10.0.0.1",
    "1.1.1.1",
];

fn main() {
    lazy();

    overview();
}

fn lazy() {
    for ip in IPS {
        let start = Instant::now();
        let res = memory_search(ip);
        let end = start.elapsed().subsec_micros();
        println!("lazy__ {:06} microseconds: {:?}", end, res);
    }
}

fn overview() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let db_path = &args[0];

    let mut ip2 = Ip2Region::new(db_path).unwrap();
    let ip2o = ip2.to_owned().unwrap();

    for ip in IPS {
        let start = Instant::now();
        let res = ip2o.memory_search(ip);
        let end = start.elapsed().subsec_micros();
        println!("memory {:06} microseconds: {:?}", end, res);

        let start = Instant::now();
        let res = ip2.binary_search(ip);
        let end = start.elapsed().subsec_micros();
        println!("binary {:06} microseconds: {:?}", end, res);

        let start = Instant::now();
        let res = ip2.btree_search(ip);
        let end = start.elapsed().subsec_micros();
        println!("btree  {:06} microseconds: {:?}", end, res);

        println!();
    }
}
