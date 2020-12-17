use std::collections::HashSet;

type Cube = (i32, i32, i32);

#[test]
fn get_next_state_should_return_active_when_previously_inactive_and_3_active_neighbors() {
    let cube: Cube = (0, 0, 0);
    let previous_state = false;
    let mut grid: HashSet<Cube> = HashSet::new();
    grid.insert((0, 0, 1));
    grid.insert((0, 1, 0));
    grid.insert((0, 1, 1));
    assert_eq!(get_next_state(cube, previous_state, &grid), true);
}

#[test]
fn get_next_state_should_return_inactive_when_previously_inactive_and_2_active_neighbors() {
    let cube: Cube = (0, 0, 0);
    let previous_state = false;
    let mut grid: HashSet<Cube> = HashSet::new();
    grid.insert((0, 0, 1));
    grid.insert((0, 1, 0));
    assert_eq!(get_next_state(cube, previous_state, &grid), false);
}

#[test]
fn get_next_state_should_return_inactive_when_previously_inactive_and_less_than_2_active_neighbors()
{
    let cube: Cube = (0, 0, 0);
    let previous_state = false;
    let mut grid: HashSet<Cube> = HashSet::new();
    grid.insert((0, 0, 1));
    assert_eq!(get_next_state(cube, previous_state, &grid), false);
}

#[test]
fn get_next_state_should_return_inactive_when_previously_inactive_and_more_than_3_active_neighbors()
{
    let cube: Cube = (0, 0, 0);
    let previous_state = false;
    let mut grid: HashSet<Cube> = HashSet::new();
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
    let mut grid: HashSet<Cube> = HashSet::new();
    grid.insert((0, 0, 1));
    grid.insert((0, 1, 0));
    grid.insert((0, 1, 1));
    assert_eq!(get_next_state(cube, previous_state, &grid), true);
}

#[test]
fn get_next_state_should_return_active_when_previously_active_and_2_active_neighbors() {
    let cube: Cube = (0, 0, 0);
    let previous_state = true;
    let mut grid: HashSet<Cube> = HashSet::new();
    grid.insert((0, 0, 1));
    grid.insert((0, 1, 0));
    assert_eq!(get_next_state(cube, previous_state, &grid), true);
}

#[test]
fn get_next_state_should_return_inactive_when_previously_active_and_less_than_2_active_neighbors() {
    let cube: Cube = (0, 0, 0);
    let previous_state = true;
    let mut grid: HashSet<Cube> = HashSet::new();
    grid.insert((0, 0, 1));
    assert_eq!(get_next_state(cube, previous_state, &grid), false);
}

#[test]
fn get_next_state_should_return_inactive_when_previously_active_and_more_than_3_active_neighbors() {
    let cube: Cube = (0, 0, 0);
    let previous_state = true;
    let mut grid: HashSet<Cube> = HashSet::new();
    grid.insert((0, 0, 1));
    grid.insert((0, 1, 0));
    grid.insert((0, 1, 1));
    grid.insert((1, 0, 0));
    assert_eq!(get_next_state(cube, previous_state, &grid), false);
}

fn get_next_state(cube: Cube, previous_state: bool, grid: &HashSet<Cube>) -> bool {
    let mut active_neighbors = 0u32;
    for x in cube.0 - 1..cube.0 + 2 {
        for y in cube.1 - 1..cube.1 + 2 {
            for z in cube.2 - 1..cube.2 + 2 {
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

fn main() {
    println!("Hello, world!");
}
