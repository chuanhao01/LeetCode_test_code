use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let fl: Vec<u64> = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|num| num.parse().unwrap())
        .collect();
    let t = fl[0];
    let b = fl[1];
    let bs: Vec<u64> = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|num| num.parse().unwrap())
        .collect();
    let mut versions: HashMap<String, u64> = HashMap::new();
    for _ in 0..t {
        let nxl = lines.next().unwrap();
        let mut pv_iter = nxl.split(' ');
        let package_name = pv_iter.next().unwrap().to_owned();
        let package_version: u64 = pv_iter.next().unwrap().parse().unwrap();
        versions.insert(package_name, package_version);
    }
    for store_versions in 0..b {
        let mut mismatch = 0u64;
        for _ in 0..bs[store_versions as usize] {
            let nxl = lines.next().unwrap();
            let mut pv_iter = nxl.split(' ');
            let package_name = pv_iter.next().unwrap();
            let package_version: u64 = pv_iter.next().unwrap().parse().unwrap();
            mismatch += versions.get(package_name).unwrap() - package_version;
        }
        println!("{}", mismatch);
    }
}
