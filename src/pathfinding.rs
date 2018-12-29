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
        let mut came_from: HashMap<GridPoint, GridPoint> = HashMap::new();
        let mut g_score: HashMap<GridPoint, f64> = HashMap::new();
        let mut f_score: HashMap<GridPoint, f64> = HashMap::new();

        open_set.insert(start);
        g_score.insert(start, 0.0);
        f_score.insert(start, Pathfinding::heuristic_cost_estimate(&start, &goal));

        while !open_set.is_empty() {
            let current = Pathfinding::get_lowest_fscore(&f_score);

            if current == goal {
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
        }

        /* if came_from.len() > 0 {
            let closest = Pathfinding::get_closest(&start, &goal, &came_from);

            for pair in &came_from {
                println!("{:#?}", pair);
            }
            return Pathfinding::reconstruct_path(came_from, closest);
        } */

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
}