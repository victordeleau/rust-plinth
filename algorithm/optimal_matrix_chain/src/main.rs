use std::vec::Vec;
use std::io::{self, BufRead};


fn optimal_matrix_chain(input_dim: Vec<usize>) {
    /*
    Dynamic programming implementation to find
    the optimal matrix chain product parenthesization.
    */

    let nb_matrix: usize = input_dim.len();

    if nb_matrix == 0 {
        println!("()");
        return ();
    }

    let mut m: Vec<Vec<u64>> = vec![vec![0; nb_matrix]; nb_matrix];
    let mut s: Vec<Vec<usize>> = vec![vec![0; nb_matrix]; nb_matrix-1];
    
    // solve each sub-problem in a top-down fashion
    let mut j: usize = 0;
    let mut q: u64 = 0;
    for l in 2..nb_matrix {
        for i in 1..(nb_matrix-l+1) {
            j = i + l - 1;
            m[i][j] = std::u64::MAX;
            for k in i..j {
                q = m[i][k]+m[k+1][j]+((input_dim[i-1]*input_dim[k]*input_dim[j]) as u64);
                if q < m[i][j] {
                    m[i][j] = q;
                    s[i][j] = k;
                }
            }
        }
    }

    fn build_optimal(s: &Vec<Vec<usize>>, i: usize, j: usize) {
        /*
        Build the optimal solution from subproblems solutions recursively
        */

        if i == j { print!(" A{} ", i-1); }
        else {
            print!("( ");
            build_optimal(s, i, s[i][j]);
            build_optimal(s, s[i][j] + 1, j);
            print!(") ");
        }
    }

    build_optimal(&s, 1, nb_matrix-1);
    println!();
}


fn get_matrix_dim(matrix_dim: &mut Vec<usize>) {
    /*  
    Add integers to Vec from stdin
    */

    let stdin = io::stdin();
    
    loop {
        match stdin.lock().lines().next().unwrap() {
            Ok(_i) => {
                match _i.trim().parse() {
                    Ok(_i) => matrix_dim.push( _i ),
                    Err(_error) => break
                };
            }
            Err(_error) => break
        };
    }
}


fn main() {

    println!("Optimal matrix chain product !\n");
    println!("Please enter your dimensions.");

    let mut matrix_dim: Vec<usize> = Vec::new();
    get_matrix_dim(&mut matrix_dim);

    // must yield ( ( ( A0 A1 ) ( A2 A3 ) ) ( A4 A5 ) )
    // let matrix_dim: Vec<usize> = vec![5, 10, 6, 30, 4, 12, 16];
    
    print!("Matrices of dimensions ");
    for i in 0..matrix_dim.len() { print!("{} ", matrix_dim[i]); }
    
    println!("have an optimal product chain parenthesization :");
    optimal_matrix_chain(matrix_dim);
}
