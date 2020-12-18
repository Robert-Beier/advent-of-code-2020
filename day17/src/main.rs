use lib::{read_input, solve};
use std::collections::HashSet;

type Cube = (i32, i32, i32);
type Grid = HashSet<Cube>;

#[test]
fn get_next_state_should_return_active_when_previously_inactive_and_3_active_neighbors() {
    let cube: Cube = (0, 0, 0);
    let previous_state = false;
    let mut grid: Grid = HashSet::new();
    grid.insert((0, 0, 1));
    grid.insert((0, 1, 0));
    grid.insert((0, 1, 1));
    assert_eq!(get_next_state(cube, previous_state, &grid), true);
}

#[test]
fn get_next_state_should_return_inactive_when_previously_inactive_and_2_active_neighbors() {
    let cube: Cube = (0, 0, 0);
    let previous_state = false;
    let mut grid: Grid = HashSet::new();
    grid.insert((0, 0, 1));
    grid.insert((0, 1, 0));
    assert_eq!(get_next_state(cube, previous_state, &grid), false);
}

#[test]
fn get_next_state_should_return_inactive_when_previously_inactive_and_less_than_2_active_neighbors()
{
    let cube: Cube = (0, 0, 0);
    let previous_state = false;
    let mut grid: Grid = HashSet::new();
    grid.insert((0, 0, 1));
    assert_eq!(get_next_state(cube, previous_state, &grid), false);
}

#[test]
fn get_next_state_should_return_inactive_when_previously_inactive_and_more_than_3_active_neighbors()
{
    let cube: Cube = (0, 0, 0);
    let previous_state = false;
    let mut grid: Grid = HashSet::new();
    grid.insert((0, 0, 1));
    grid.insert((0, 1, 0));
    grid.insert((0, 1, 1));
    grid.insert((1, 0, 0));
    assert_eq!(get_next_state(cube, previous_state, &grid), false);
}

#[test]
fn get_next_state_should_return_active_when_previously_active_and_3_active_neighbors() {
    let cube: Cube = (0, 0, 0);
    let previous_state = true;
    let mut grid: Grid = HashSet::new();
    grid.insert((0, 0, 1));
    grid.insert((0, 1, 0));
    grid.insert((0, 1, 1));
    assert_eq!(get_next_state(cube, previous_state, &grid), true);
}

#[test]
fn get_next_state_should_return_active_when_previously_active_and_2_active_neighbors() {
    let cube: Cube = (0, 0, 0);
    let previous_state = true;
    let mut grid: Grid = HashSet::new();
    grid.insert((0, 0, 1));
    grid.insert((0, 1, 0));
    assert_eq!(get_next_state(cube, previous_state, &grid), true);
}

#[test]
fn get_next_state_should_return_inactive_when_previously_active_and_less_than_2_active_neighbors() {
    let cube: Cube = (0, 0, 0);
    let previous_state = true;
    let mut grid: Grid = HashSet::new();
    grid.insert((0, 0, 1));
    assert_eq!(get_next_state(cube, previous_state, &grid), false);
}

#[test]
fn get_next_state_should_return_inactive_when_previously_active_and_more_than_3_active_neighbors() {
    let cube: Cube = (0, 0, 0);
    let previous_state = true;
    let mut grid: Grid = HashSet::new();
    grid.insert((0, 0, 1));
    grid.insert((0, 1, 0));
    grid.insert((0, 1, 1));
    grid.insert((1, 0, 0));
    assert_eq!(get_next_state(cube, previous_state, &grid), false);
}

fn get_next_state(cube: Cube, previous_state: bool, grid: &Grid) -> bool {
    let mut active_neighbors = 0u32;
    for x in cube.0 - 1..cube.0 + 2 {
        for y in cube.1 - 1..cube.1 + 2 {
            for z in cube.2 - 1..cube.2 + 2 {
                if x == cube.0 && y == cube.1 && z == cube.2 {
                    continue;
                }
                if grid.contains(&(x, y, z)) {
                    active_neighbors += 1;
                }
                if active_neighbors > 3 {
                    return false;
                }
            }
        }
    }
    if active_neighbors == 3 || (active_neighbors == 2 && previous_state) {
        return true;
    }
    false
}

fn get_potential_cubes(grid: &Grid) -> Grid {
    let mut potential_cubes = HashSet::new();
    for &cube in grid {
        for x in cube.0 - 1..cube.0 + 2 {
            for y in cube.1 - 1..cube.1 + 2 {
                for z in cube.2 - 1..cube.2 + 2 {
                    if x == cube.0 && y == cube.1 && z == cube.2 {
                        continue;
                    }
                    potential_cubes.insert((x, y, z));
                }
            }
        }
    }
    potential_cubes
}

fn get_next_cycle(grid: &Grid) -> Grid {
    let potential_cubes = get_potential_cubes(&grid);
    potential_cubes
        .iter()
        .map(|&c| c)
        .filter(|c| get_next_state(*c, grid.contains(c), &grid))
        .collect()
}

fn parse_grid(input: &String) -> Grid {
    input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '#' => Option::Some((x as i32, y as i32, 0i32)),
                    _ => Option::None,
                })
                .collect::<Vec<Cube>>()
        })
        .flatten()
        .collect()
}

fn part_one(grid: &Grid) {
    solve("Part one", || {
        let mut current_grid = get_next_cycle(&grid);
        for _ in 0..5 {
            current_grid = get_next_cycle(&grid);
        }
        current_grid.len()
    });
}

fn main() {
    let input = read_input();
    let grid = parse_grid(&input);
    part_one(&grid);
}
