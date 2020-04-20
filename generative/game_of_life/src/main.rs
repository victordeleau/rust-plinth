use rand::Rng;
use std::{thread, time};
use std::vec::Vec;

use ansi_escapes::*; // specific characters

const GRID_SIZE: (usize, usize) = (52, 52);
const PROBABILITY: f32 = 0.4;
const FPS: f32 = 10.0;


fn print_grid(grid_size: (usize, usize), grid: &Vec<Vec<char>>){
    /*
    print the grid in the terminal
    input
        grid_size: (usize, usize)
        grid: &Vec<Vec<char>>
    output
        None
    */

    print!("      ");
    for _ in  0..(grid_size.0+2)*2 {print!("\u{2588}");}
    println!("");

    for i in  0..grid_size.1 {
        print!("      \u{2588}\u{2588}");
        for j in  0..grid_size.0 {
            print!("{}{}", grid[i][j], grid[i][j]);
        }
        println!("\u{2588}\u{2588}");
    }
    print!("      ");
    for _ in  0..(grid_size.0+2)*2 {print!("\u{2588}");}
    println!("\n");
}


fn update_grid(grid_size: (usize, usize), current_grid: &Vec<Vec<char>>, new_grid: &mut Vec<Vec<char>>) {
    /*
    update grid state
    input
        grid_size: (usize, usize)
        current_grid: &Vec<Vec<char>>
        new_grid: &mut Vec<Vec<char>>
    output
        None
    */

    for i in  0..grid_size.1 {
        for j in  0..grid_size.0 {

            let x = nb_alive_neighbor(&current_grid, (i, j));

            if current_grid[i][j] == '\u{2588}' { // is alive
                if x < 2 || x > 3 { // cell die
                    new_grid[i][j] = ' ';
                }
            } else { // cell is dead
                if x == 3 { // cell becomes alive
                    new_grid[i][j] = '\u{2588}';
                }
            }
        }
    }
}


fn nb_alive_neighbor(grid: &Vec<Vec<char>>, cell: (usize, usize)) -> u8 {
    /*
    Takes a cell index as input and return its number of alive neighbor.
    input
        grid: &[[char; 32]; 32]
        cell: (usize, usize)
    output
        u8
    */

    let mut nb_alive_neighbor: u8 = 0;

    // check top
    if cell.0 != 0 { 

        if grid[cell.0-1][cell.1] == '\u{2588}' { nb_alive_neighbor += 1; }

        // check top right
        if cell.1 != grid[0].len()-1 { 
            if grid[cell.0-1][cell.1+1] == '\u{2588}' {
                nb_alive_neighbor += 1;
            }
        }

        // check top left
        if cell.1 != 0 { 
            if grid[cell.0-1][cell.1-1] == '\u{2588}' {
                nb_alive_neighbor += 1;
            }
        }
    }
    
    // check bottom
    if cell.0 != grid.len()-1 { 

        if grid[cell.0+1][cell.1] == '\u{2588}' {
            nb_alive_neighbor += 1;
        }

        // check bottom left
        if cell.1 != 0 { 
            if grid[cell.0+1][cell.1-1] == '\u{2588}' {
                nb_alive_neighbor += 1;
            }
        }
        
        // check bottom right
        if cell.1 != grid[0].len()-1 { 
            if grid[cell.0+1][cell.1+1] == '\u{2588}' {
                nb_alive_neighbor += 1;
            }
        }
    }    

    // check left
    if cell.1 != 0 { 
        if grid[cell.0][cell.1-1] == '\u{2588}' {
            nb_alive_neighbor += 1;
        }
    }
    
    // check right
    if cell.1 != grid[0].len()-1 { 
        if grid[cell.0][cell.1+1] == '\u{2588}' {
            nb_alive_neighbor += 1;
        } 
    }

    return nb_alive_neighbor;
}


fn initialize_grid(probability: f32) -> Vec<Vec<char>> {
    /*
    Initialize a new grid of cells according to probability.
    input
        probability: f32
    output
        Vec<Vec<i64>>
    */

    let mut rng = rand::thread_rng();

    (0..GRID_SIZE.0).map(|_| {
        (0..GRID_SIZE.1).map(|_| {

            // cell is alive
            if rng.gen::<f32>() < probability { '\u{2588}' } 

            // cell is dead
            else { ' ' } 
            
        }).collect()
    }).collect()
}


////////////////////////////////////////////////////////////////////////////////
// main 
////////////////////////////////////////////////////////////////////////////////

fn main(){

    println!("\n\n                        Welcome to the game of life !\n");
    
    let mut current_grid = initialize_grid(PROBABILITY);

    let mut new_grid: Vec<Vec<char>>=vec![vec![' '; GRID_SIZE.1]; GRID_SIZE.0];
    
    let delay = time::Duration::from_millis((1.0/FPS * 1000.0) as u64);

    loop {

        thread::sleep(delay);

        print_grid(GRID_SIZE, &current_grid);

        update_grid(GRID_SIZE, &current_grid, &mut new_grid);
        
        current_grid = new_grid.to_owned();

        // erase terminal
        for _ in 0..GRID_SIZE.0+3 { print!("{}{}", CursorUp(1), EraseLine); }
    }
}