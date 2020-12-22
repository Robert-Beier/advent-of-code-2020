mod part_one;
mod part_two;

use lib::read_input;

fn main() {
    let input = read_input();
    let grid = part_one::parse_grid(&input, 0);
    part_one::part_one(&grid);
    let grid = part_two::parse_grid(&input, 0, 0);
    part_two::part_two(&grid);
}
