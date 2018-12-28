extern crate rand;

mod vector2;
mod gridpoint;
mod gridmodel;

use std::collections::HashMap;
use std::collections::HashSet;
use std::f64::INFINITY;
use rand::Rng;

use gridpoint::GridPoint;
use gridmodel::GridModel;

fn main() {
    /* let mut grid = GridModel::new(5, 5);
    for x in 0..5 {
        for y in 0..5 {
            print!("{}", grid.get_point(x, y).key);
        }
        println!("");
    } */
    let maze_path = concat!(
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppaaaaaaaaaaaaaaaaaaaappppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppappppppppppppppppppappppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppapaaaaaaaaaaaaaaaapappppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppapappppppppppppppapappppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppapapaaaaaaaaaaaapapappppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppapapappppppppppapapappppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppapapapaaaaaaaapapapappppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppapapapappppppapapapappppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppapapapapaaaapapapapappppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppapapapapaaaapapapapappppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppapppapppaapppapppappppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppapapapapaaaapapapapappppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppapapapapaaaapapapapappppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppapapapappppppapapapappppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppapapapaaaaaaaapapapappppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppapapappppppppppapapappppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppapapaaaaaaaaaaaapapappppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppapappppppppppppppapappppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppapaaaaaaaaaaaaaaaapappppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppappppppppppppppppppappppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppaaaaaaaaaaaaaaaaaaaappppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp",
        "pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp");
    let mut grid = GridModel::from_string(maze_path.to_string(), 100, 100);
    let mut start = *grid.get_point(0, 0);
    let mut goal = *grid.get_point(25, 25);
    let path = Pathfinding::a_star(start, goal, grid);
    grid = GridModel::from_string(maze_path.to_string(), 100, 100);
    Pathfinding::print_route(grid, path);



    /* println!("Path: {}", path.len());
    for point in path {
        println!("{:?}", point);
    } */

    //let mut map: HashMap<GridPoint, f64> = HashMap::new();
    //map.insert(start, 0.0);
    //let mut start2 = GridPoint::new(0, 0, 'p');
    //map.remove(&start2);
    //println!("{:?}", map);


    /* let no_solution_path = concat!(
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
    let mut grid = GridModel::from_string(no_solution_path.to_string(), 16, 16);
    let mut start = *grid.get_point(1, 1);
    let mut goal = *grid.get_point(9, 7);
    let path = Pathfinding::a_star(start, goal, grid);
    grid = GridModel::from_string(no_solution_path.to_string(), 16, 16);
    Pathfinding::print_route(grid, path); */
}


pub struct Pathfinding {

}

impl Pathfinding {
    pub fn reconstruct_path(came_from: HashMap<GridPoint, GridPoint>, mut current: GridPoint) -> Vec<GridPoint> {
        let mut total_path = vec![current];
        while came_from.contains_key(&current) {
            current = *came_from.get(&current).unwrap();
            total_path.push(current);
        }
        total_path
    }

    pub fn a_star(start: GridPoint, goal: GridPoint, grid: GridModel) -> Vec<GridPoint> {
        // TODO right now a_star() consumes the grid; fix that
        let mut closed_set: HashSet<GridPoint> = HashSet::new();

        let mut open_set: HashSet<GridPoint> = HashSet::new();
        open_set.insert(start);

        let mut came_from: HashMap<GridPoint, GridPoint> = HashMap::new();

        let mut g_score: HashMap<GridPoint, f64> = HashMap::new();
        g_score.insert(start, 0.0);

        let mut f_score: HashMap<GridPoint, f64> = HashMap::new();
        f_score.insert(start, Pathfinding::heuristic_cost_estimate(&start, &goal));

        //let mut i = 0;
        while !open_set.is_empty() {
            let current = Pathfinding::get_lowest_fscore(&f_score);

            /* if i > 500 {
                break;
            } else {
            }
                println!("Iteration: {}", i);
                println!("Current:");
                println!("{:?}", current);
                println!("Closed Set: {}", closed_set.len());
                println!("{:?}", closed_set);
                println!("Open Set: {}", open_set.len());
                println!("{:?}", open_set);
                println!("Came From: {}", came_from.len());
                println!("{:?}", came_from);
                println!("F Score: {}", f_score.len());
                println!("{:?}", f_score);
                println!("G Score: {}", g_score.len());
                println!("{:?}", g_score);
                println!(""); */


            if current == goal {
                //println!("Found path somehow");
                return Pathfinding::reconstruct_path(came_from, current)
            }
            open_set.remove(&current);
            f_score.remove(&current);
            closed_set.insert(current);

            for neighbor in grid.get_neighbors(&current) {
                if closed_set.contains(neighbor) {
                    continue
                }

                let mut tentative_g_score = g_score.get(neighbor).unwrap_or(&0.0) + current.manhattan_distance_to(neighbor);

                if !open_set.contains(neighbor) {
                    open_set.insert(*neighbor);
                } else if tentative_g_score >= *g_score.get(neighbor).unwrap() {
                    continue
                }

                came_from.insert(*neighbor, current);
                g_score.insert(*neighbor, tentative_g_score);
                f_score.insert(*neighbor, tentative_g_score + Pathfinding::heuristic_cost_estimate(neighbor, &goal));
            }
            //i += 1;
        }

        if came_from.len() > 0 {
            let closest = Pathfinding::get_closest(&start, &goal, &came_from);

            for pair in &came_from {
                println!("{:#?}", pair);
            }
            return Pathfinding::reconstruct_path(came_from, closest);
        }

        // We should never get here, but if no solution was found we'll return an empty list
        //println!("No path found, rip");
        Vec::new()
    }

    pub fn get_closest(start: &GridPoint, goal: &GridPoint, came_from: &HashMap<GridPoint, GridPoint>) -> GridPoint {
        let mut nearest = *start;
        let mut nearest_distance = nearest.manhattan_distance_to(goal);
        for point in came_from.values() {
            let distance = point.manhattan_distance_to(goal);
            if distance < nearest_distance {
                nearest = *point;
                nearest_distance = distance;
            }
        }

        nearest
    }

    pub fn get_lowest_fscore(scores: &HashMap<GridPoint, f64>) ->  GridPoint {
        // Defaulting to a generic GridPoint, not sure how that plays out
        let mut lowest_node: GridPoint = GridPoint::new(0, 0, 'p');
        let mut lowest_score: f64 = INFINITY;
        for score in scores {
            if score.1 < &lowest_score {
                lowest_node = *score.0;
                lowest_score = *score.1;
            }
        }

        lowest_node
    }

    pub fn heuristic_cost_estimate(current: &GridPoint, goal: &GridPoint) -> f64 {
        current.manhattan_distance_to(goal)
    }

    pub fn print_route(grid:GridModel, path: Vec<GridPoint>) {
        let mut steps = vec![];
        for x in 0..grid.height {
            for y in 0..grid.width {
                let mut display = grid.get_point(x, y).key;
                for point in &path {
                    if point.x as usize == x && point.y as usize == y {
                        display = ' ';
                        steps.push(point.key);
                        break;
                    }
                }
                print!("{}", display);
            }
            println!("");
        }
        print!("Path:");
        for step in steps {
            print!("{}", step);
        }
        println!("");
    }
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
        let path = Pathfinding::a_star(start, goal, grid);
        if path.len() > 0 {
            solution_found = true;
            println!("Random grid with solution ({},{} -> {},{}) found in {} iterations:", start.x, start.y, goal.x, goal.y, iterations);
            grid = GridModel::from_string(random_grid.to_string(), 100, 100);
            Pathfinding::print_route(grid, path);
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
        let path = Pathfinding::a_star(start, goal, grid);
        if path.len() > 0 && (path.len() as f64) < longest {
            longest = path.len() as f64;
            longest_path = random_grid;
        }
    }
    let mut grid = GridModel::from_string(longest_path.to_string(), 100, 100);
    let start = *grid.get_point(20, 20);
    let goal = *grid.get_point(79, 79);
        let path = Pathfinding::a_star(start, goal, grid);
    println!("Random grid with solution ({},{} -> {},{}) with longest path of {} steps ({} iterations):", start.x, start.y, goal.x, goal.y, path.len(), iterations);
    grid = GridModel::from_string(longest_path.to_string(), 100, 100);
    Pathfinding::print_route(grid, path);
}