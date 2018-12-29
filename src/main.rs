extern crate rand;

mod vector2;
mod gridpoint;
mod gridmodel;
mod pathfinding;

use std::f64::INFINITY;
use rand::Rng;

use gridpoint::GridPoint;
use gridmodel::GridModel;
use pathfinding::Pathfinding;

fn main() {
    test_generate_random_grid_with_longest_path_indefinitely();
}

pub fn test_generate_random_grid_with_solution() {
    // This test randomly generates grids until one with a solution is found
    let mut random_grid = String::new();
    let chars = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p'];
    let mut iterations = 0;
    let mut solution_found = false;
    while !solution_found {
        iterations += 1;
        random_grid = String::new();
        for i in 0..10000 {
            random_grid.push(chars[rand::thread_rng().gen_range(0, chars.len())]);
        }

        let mut grid = GridModel::from_string(random_grid.to_string(), 100, 100);
        let start = *grid.get_point(20, 20);
        let goal = *grid.get_point(79, 79);
        let path = Pathfinding::a_star(start, goal, grid, false);
        if path.len() > 0 {
            solution_found = true;
            println!("Random grid with solution ({},{} -> {},{}) found in {} iterations:", start.x, start.y, goal.x, goal.y, iterations);
            grid = GridModel::from_string(random_grid.to_string(), 100, 100);
            print_route(grid, path);
        }
    }
}

pub fn test_generate_random_grid_with_longest_path() {
    // This test randomly generates grids and preserves the one with the longest path
    let mut random_grid = String::new();
    let chars = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p'];
    let mut longest = INFINITY;
    let mut longest_path = String::new();
    let iterations = 1000000;
    for iteration in 1..iterations {
        random_grid = String::new();
        for i in 0..10000 {
            random_grid.push(chars[rand::thread_rng().gen_range(0, chars.len())]);
        }

        let mut grid = GridModel::from_string(random_grid.to_string(), 100, 100);
        let start = *grid.get_point(20, 20);
        let goal = *grid.get_point(79, 79);
        let path = Pathfinding::a_star(start, goal, grid, false);
        if path.len() > 0 && (path.len() as f64) > longest {
            longest = path.len() as f64;
            longest_path = random_grid;
        }
    }
    let mut grid = GridModel::from_string(longest_path.to_string(), 100, 100);
    let start = *grid.get_point(20, 20);
    let goal = *grid.get_point(79, 79);
    let path = Pathfinding::a_star(start, goal, grid, false);
    println!("Random grid with solution ({},{} -> {},{}) with longest path of {} steps ({} iterations):", start.x, start.y, goal.x, goal.y, path.len(), iterations);
    grid = GridModel::from_string(longest_path.to_string(), 100, 100);
    print_route(grid, path);
}

pub fn test_generate_random_grid_with_longest_path_indefinitely() {
    let mut generations = 0;
    // This test randomly generates grids and preserves the one with the longest path, until forced to stop
    let mut random_grid = String::new();
    let chars = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p'];
    let mut longest = 0;
    let mut longest_path = String::new();
    while true {
        generations += 1;
        random_grid = String::new();
        for i in 0..10000 {
            random_grid.push(chars[rand::thread_rng().gen_range(0, chars.len())]);
        }

        let path_str = random_grid.to_string();

        let mut grid = GridModel::from_string(path_str.clone(), 100, 100);
        let start = *grid.get_point(20, 20);
        let goal = *grid.get_point(79, 79);
        let path = Pathfinding::a_star(start, goal, grid, false);
        if path.len() > 0 && path.len() > longest {
            longest = path.len();
            longest_path = random_grid;
            //Print for posterity
            grid = GridModel::from_string(path_str, 100, 100);
            print_route(grid, path);
        }
        if generations % 100 == 0 {
            println!("Generations elapsed: {}", generations);
        }
    }
}

pub fn test_row_positioning() {
    let position_testing_path = concat!(
        "aaaaaaaaaaaaaaaa",
        "bbbbbbbbbbbbbbbb",
        "cccccccccccccccc",
        "dddddddddddddddd",
        "eeeeeeeeeeeeeeee",
        "ffffffffffffffff",
        "gggggggggggggggg",
        "hhhhhhhhhhhhhhhh",
        "iiiiiiiiiiiiiiii",
        "jjjjjjjjjjjjjjjj",
        "kkkkkkkkkkkkkkkk",
        "llllllllllllllll",
        "mmmmmmmmmmmmmmmm",
        "nnnnnnnnnnnnnnnn",
        "oooooooooooooooo",
        "pppppppppppppppp");
    let mut grid = GridModel::from_string(position_testing_path.to_string(), 16, 16);
    let mut start = *grid.get_point(1, 1);
    let mut goal = *grid.get_point(9, 7);
    let path = Pathfinding::a_star(start, goal, grid, false);
    grid = GridModel::from_string(position_testing_path.to_string(), 16, 16);
    print_route(grid, path);
}

pub fn test_small_snake_path() {
    let small_snake_path = concat!(
        "aaaaaaaaaaaaaaaa",
        "aeeeeeeeeeeeeeca",
        "aaaaaaaaaaaaaaca",
        "aeeeeeeeeeeecaca",
        "abaaaaaaaaaacaca",
        "abaeeeeeeecacaca",
        "ababaaaaaacacaca",
        "ababaeeecacacaca",
        "abababaacacacaca",
        "abababaddacacaca",
        "abababaaaacacaca",
        "abababdddddacaca",
        "ababaaaaaaaacaca",
        "ababdddddddddaca",
        "abaaaaaaaaaaaaca",
        "abddddddddddddda",
        "aaaaaaaaaaaaaaaa");
    let mut grid = GridModel::from_string(small_snake_path.to_string(), 16, 17);
    let mut start = *grid.get_point(1, 1);
    let mut goal = *grid.get_point(7, 9);
    let path = Pathfinding::a_star(start, goal, grid, false);
    grid = GridModel::from_string(small_snake_path.to_string(), 16, 17);
    print_route(grid, path);
}

pub fn print_route(grid:GridModel, path: Vec<GridPoint>) {
    //let mut steps = vec![];
    /* for y in 0..grid.height {
        for x in 0..grid.width {
            print!("{}", grid.get_point(x, y).key);
        }
        println!("");
    }
    println!(""); */
    for y in 0..grid.height {
        for x in 0..grid.width {
            let mut display = grid.get_point(x, y).key;
            for point in &path {
                if point.x as usize == x && point.y as usize == y {
                    display = ' ';
                    //steps.push(point.key);
                    break;
                }
            }
            print!("{}", display);
        }
        println!("");
    }
    print!("Path ({}):", path.len());
    let mut steps = path.clone();
    steps.reverse();
    for step in steps {
        print!("{}", step.key);
    }
    println!("");
    print!("Full route:");
    for y in 0..grid.height {
        for x in 0..grid.width {
            print!("{}", grid.get_point(x, y).key);
        }
    }
    println!("");
}