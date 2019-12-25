#![feature(test)]

extern crate test;

use std::fs;

use test::Bencher;
use tf_demo_parser::{Demo, DemoParser, Stream};

fn bench_file(input_file: &str, b: &mut Bencher) {
    let file = fs::read(input_file).expect("Unable to read file");
    let demo = Demo::new(file);
    let stream: Stream = demo.get_stream();

    b.iter(|| {
        let (_, state) = DemoParser::new(stream.clone()).parse().unwrap();
        test::black_box(state);
    })
}

#[bench]
fn bench_gully(b: &mut Bencher) {
    bench_file("data/gully.dem", b);
}

#[bench]
fn bench_comp(b: &mut Bencher) {
    bench_file("data/comp.dem", b);
}
