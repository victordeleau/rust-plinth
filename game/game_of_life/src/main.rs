// for testing stuff

use rand::Rng;
use std::{thread, time};

use ansi_escapes::*;

const ESC: char = 27u8 as char;
const BACKSPACE: char = 8u8 as char;

// print the grid in the terminal
fn print_grid(){

    ;

}

fn erase_grid(){

    ;
}


// check the number of alive neighbor
fn nb_alive_neighbor(grid: &[[char; 32]; 32], cell: (usize, usize)) -> u8 {

    let mut nb_alive_neighbor: u8 = 0;

    if cell.0 != 0 { // check top

        if grid[cell.0-1][cell.1] == '\u{2588}' { nb_alive_neighbor += 1; }

        if cell.1 != grid[0].len()-1 { // check top right
            if grid[cell.0-1][cell.1+1] == '\u{2588}' { nb_alive_neighbor += 1; }
        }

        if cell.1 != 0 { // check top left
            if grid[cell.0-1][cell.1-1] == '\u{2588}' { nb_alive_neighbor += 1; }
        }
    }
    
    if cell.0 != grid.len()-1 { // check bottom

        if grid[cell.0+1][cell.1] == '\u{2588}' { nb_alive_neighbor += 1; }

        if cell.1 != 0 { // check bottom left
            if grid[cell.0+1][cell.1-1] == '\u{2588}' { nb_alive_neighbor += 1; }
        }
        
        if cell.1 != grid[0].len()-1 { // check bottom right
            if grid[cell.0+1][cell.1+1] == '\u{2588}' { nb_alive_neighbor += 1; }
        }
    }    

    if cell.1 != 0 { // check left
        if grid[cell.0][cell.1-1] == '\u{2588}' { nb_alive_neighbor += 1; }
    }
    
    if cell.1 != grid[0].len()-1 { // check right
        if grid[cell.0][cell.1+1] == '\u{2588}' { nb_alive_neighbor += 1; } 
    }

    return nb_alive_neighbor;
}


fn main(){

    println!("\n\n                        Welcome to the game of life !");

    const GRID_SIZE: (usize, usize) = (32, 32);
    const LIFE_PROBABILITY: f32 = 0.4;
    const FPS: f32 = 2.0;

    let mut current_grid = [[' '; GRID_SIZE.0]; GRID_SIZE.1];
    let mut new_grid = [[' '; GRID_SIZE.0]; GRID_SIZE.1];

    // initialize random number generator
    let mut rng = rand::thread_rng();

    // initialize grid
    for i in  0..GRID_SIZE.1 {
        for j in  0..GRID_SIZE.0 {
            let draw: f32 = rng.gen();
            if draw < LIFE_PROBABILITY {
                // cell is alive
                current_grid[i][j] = '\u{2588}'; 
            } else {
                // cell is dead
                current_grid[i][j] = ' ';
            }
        }
    }

    ///////////////////////////////////////////////////////////////////
    // main game lopp /////////////////////////////////////////////////
    
    //let delay = time::Duration::from_millis((1000.0/FPS) as u64);
    let delay = time::Duration::from_millis(100);

    while true {

        thread::sleep(delay);
        print!("    ");
        for i in  0..(GRID_SIZE.0+2)*2 {print!("\u{2588}");}
        println!("");

        for i in  0..GRID_SIZE.1 {
            print!("    \u{2588}\u{2588}");
            for j in  0..GRID_SIZE.0 {
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
                print!("{}", current_grid[i][j]);
                print!("{}", current_grid[i][j]);
            }
            println!("\u{2588}\u{2588}");
        }
        print!("    ");
        for i in  0..(GRID_SIZE.0+2)*2 {print!("\u{2588}");}
        println!("\n");

        current_grid = new_grid;

        for i in 0..GRID_SIZE.0+3 { print!("{}{}", CursorUp(1), EraseLine); }
    }
}