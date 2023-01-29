extern crate rand;

use std::fmt;

use good_lp::{
    constraint, default_solver, variable, Constraint, ProblemVariables, Solution, SolverModel,
};
use itertools::join;
use ndarray::{arr1, arr2};
use rand::{Fill, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn pretty<T: fmt::Display, const X: usize, const Y: usize>(arr: &[[T; X]; Y], prec: usize) {
    for inner in arr.iter() {
        println!(
            "{}",
            join(inner.iter().map(|el: &T| format!("{:.prec$}", el)), ", ")
        );
    }
}

fn main() {
    // trait: rand::SeedableRng
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    // const M: usize = 1000;
    // const N: usize = 1000;
    const M: usize = 5;
    const N: usize = 5;

    let mut _sol = [0f64; N];
    let mut _arr = [[0f64; M]; N];
    let mut _del = [0f64; N];

    // trait: rand::Rng
    // match rng.try_fill(&mut sol[..]) {
    // 	Ok(_) => (),
    // 	Err(error) => panic!("Could not allocate {:?}", error),
    // };

    // trait: rand::Fill
    match _sol.try_fill(&mut rng) {
        Ok(_) => (),
        Err(error) => panic!("Could not allocate {:?}", error),
    };

    for row in _arr.iter_mut() {
        match row.try_fill(&mut rng) {
            Ok(_) => (),
            Err(error) => panic!("Could not allocate {:?}", error),
        };
    }

    match _del.try_fill(&mut rng) {
        Ok(_) => (),
        Err(error) => panic!("Could not allocate {:?}", error),
    };

    if N < 10 && M < 10 {
        pretty(&_arr, 5);
    }

    let soln = arr1(&_sol);
    let coeffs = arr2(&_arr);
    let delta = arr1(&_del);
    let limits = &coeffs.dot(&soln) + &delta;
    println!("Expected: {limits:?}");

    let mut probvars = ProblemVariables::new();
    let vars = probvars.add_vector(variable().min(0).max(5), N);
    let mut constraints: Vec<Constraint> = Vec::with_capacity(M);
    for i in 0..M {
        let lhs = (0..N)
            .map(|j| coeffs[(i, j)] * vars[i])
            .reduce(|v1, v2| v1 + v2)
            .unwrap();
        constraints.push(constraint!(lhs <= limits[(i)]));
    }

    let objective = (0..N).map(|i| 1 * vars[i]).reduce(|i, j| i + j).unwrap();

    let mut solver_problem = probvars.maximise(objective).using(default_solver);
    // trait: good_lp::SolverModel
    for _ in 0..M {
        solver_problem = solver_problem.with(constraints.pop().expect(""));
    }
    let solution = solver_problem.solve().unwrap();

    // trait: good_lp::Solution
    for i in 0..N {
        println!("{:?} => {}", vars[i], solution.value(vars[i]));
    }
}
