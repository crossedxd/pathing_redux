mod vector2;
mod gridpoint;
mod gridmodel;

use std::collections::HashMap;
use std::collections::HashSet;
use std::f64::INFINITY;

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
    let no_solution_path = concat!(
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
    let mut grid = GridModel::from_string(no_solution_path.to_string(), 100, 100);
    let mut start = *grid.get_point(0, 0);
    let mut goal = *grid.get_point(25, 25);
    let path = Pathfinding::a_star(start, goal, grid);
    /* println!("Path: {}", path.len());
    for point in path {
        println!("{:?}", point);
    } */

    grid = GridModel::from_string(no_solution_path.to_string(), 100, 100);
    Pathfinding::print_route(grid, path);

    //let mut map: HashMap<GridPoint, f64> = HashMap::new();
    //map.insert(start, 0.0);
    //let mut start2 = GridPoint::new(0, 0, 'p');
    //map.remove(&start2);
    //println!("{:?}", map);
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

                let mut tentative_g_score = g_score.get(neighbor).unwrap_or(&0.0) + current.distance_to(neighbor);

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

        // We should never get here, but if no solution was found we'll return an empty list
        //println!("No path found, rip");
        Vec::new()
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
        current.distance_to(goal)
    }

    pub fn print_route(grid:GridModel, path: Vec<GridPoint>) {
        println!("Path:");
        for x in 0..grid.height {
            for y in 0..grid.width {
                let mut display = grid.get_point(x, y).key;
                for point in &path {
                    if point.x as usize == x && point.y as usize == y {
                        display = ' ';
                        break;
                    }
                }
                print!("{}", display);
            }
            println!("");
        }
    }
}



