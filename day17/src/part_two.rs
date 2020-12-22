use lib::solve;
use std::collections::HashSet;

type Hypercube = (i32, i32, i32, i32);
type HypercubeGrid = HashSet<Hypercube>;

fn get_next_state(cube: Hypercube, previous_state: bool, grid: &HypercubeGrid) -> bool {
    let mut active_neighbors = 0u32;
    for x in cube.0 - 1..cube.0 + 2 {
        for y in cube.1 - 1..cube.1 + 2 {
            for z in cube.2 - 1..cube.2 + 2 {
                for w in cube.3 - 1..cube.3 + 2 {
                    if x == cube.0 && y == cube.1 && z == cube.2 && w == cube.3 {
                        continue;
                    }
                    if grid.contains(&(x, y, z, w)) {
                        active_neighbors += 1;
                    }
                    if active_neighbors > 3 {
                        return false;
                    }
                }
            }
        }
    }
    if active_neighbors == 3 || (active_neighbors == 2 && previous_state) {
        return true;
    }
    false
}

fn get_potential_cubes(grid: &HypercubeGrid) -> HypercubeGrid {
    let mut potential_cubes = HashSet::new();
    for &cube in grid {
        for x in cube.0 - 1..cube.0 + 2 {
            for y in cube.1 - 1..cube.1 + 2 {
                for z in cube.2 - 1..cube.2 + 2 {
                    for w in cube.3 - 1..cube.3 + 2 {
                        if x == cube.0 && y == cube.1 && z == cube.2 && w == cube.3 {
                            continue;
                        }
                        potential_cubes.insert((x, y, z, w));
                    }
                }
            }
        }
    }
    potential_cubes
}

fn get_next_cycle(grid: &HypercubeGrid) -> HypercubeGrid {
    let potential_cubes = get_potential_cubes(&grid);
    potential_cubes
        .iter()
        .map(|&c| c)
        .filter(|c| get_next_state(*c, grid.contains(c), &grid))
        .collect()
}

pub fn parse_grid(input: &String, z_index: i32, w_index: i32) -> HypercubeGrid {
    input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.trim()
                .chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '#' => Option::Some((x as i32, y as i32, z_index, w_index)),
                    _ => Option::None,
                })
                .collect::<Vec<Hypercube>>()
        })
        .flatten()
        .collect()
}

pub fn part_two(grid: &HypercubeGrid) {
    solve("Part two", || {
        let mut current_grid = get_next_cycle(&grid);
        for _ in 0..5 {
            current_grid = get_next_cycle(&current_grid);
        }
        current_grid.len()
    });
}
