#![feature(portable_simd)]
#![feature(test)]

mod heuristic_solver;

extern crate test;

// a dummy function to test the diagonal case when solving
fn main() {
    // call the heuristic solver with two files
    let minimized = heuristic_solver::two_file_solve(
        "1000diagonal.txt".to_string(),
        "1000diagonal_1.txt".to_string(),
        (0usize..1000).collect(),
    );

    // for each tag, print the tag
    for i in minimized {
        print!("{} ", i);
    }
    println!();
}

// tell cargo to use this when executing "cargo bench"
#[cfg(test)]
mod bench {
    use crate::heuristic_solver::two_file_solve;
    use test::Bencher;
    #[bench]
    fn bench_two_file(b: &mut Bencher) {
        let f1: String = "1000diagonal.txt".to_string();
        let f2: String = "1000diagonal_1.txt".to_string();
        let desc: Vec<usize> = (0usize..1000).collect();

        b.iter(|| {
            two_file_solve(f1.clone(), f2.clone(), desc.clone());
        });
    }
}
