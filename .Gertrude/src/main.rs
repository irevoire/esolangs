mod vm;

use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.split(' ');
        let nb_words = line.clone().count();
        let sizes = line.map(|w| w.len());
        let mean: usize = sizes.clone().sum::<usize>() / nb_words;
        let smaller = sizes
            .clone()
            .fold(0, |acc, s| if s < mean { acc + 1 } else { acc });
        let longer = sizes
            .clone()
            .inspect(|a| println!("lala {}", a))
            .fold(0, |acc, s| if s > mean { acc + 1 } else { acc });
        println!("nb {:?}", nb_words);
        println!("mean {:?}", mean);
        println!("longer {:?}", longer);
        println!("smaller {:?}", smaller);
    }
}
