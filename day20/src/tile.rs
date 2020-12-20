pub struct Tile {
    id: u16,
    edge_top: u16,
    edge_top_flip: u16,
    edge_right: u16,
    edge_right_flip: u16,
    edge_bottom: u16,
    edge_bottom_flip: u16,
    edge_left: u16,
    edge_left_flip: u16,
}

#[cfg(test)]
mod tile_tests {
    use crate::tile::Tile;

    #[test]
    fn new_should_create_edges_from_image() {
        let image = "\
            ..##.#..#.\n\
            ##..#.....\n\
            #...##..#.\n\
            ####.#...#\n\
            ##.##.###.\n\
            ##...#.###\n\
            .#.#.#..##\n\
            ..#....#..\n\
            ###...#.#.\n\
            ..###..###";
        let tile = Tile::new(image, 7);
        assert_eq!(tile.id, 7);
        assert_eq!(tile.edge_top, 0b0011010010);
        assert_eq!(tile.edge_top_flip, 0b0100101100);
        assert_eq!(tile.edge_right, 0b0001011001);
        assert_eq!(tile.edge_right_flip, 0b1001101000);
        assert_eq!(tile.edge_bottom, 0b1110011100);
        assert_eq!(tile.edge_bottom_flip, 0b0011100111);
        assert_eq!(tile.edge_left, 0b0100111110);
        assert_eq!(tile.edge_left_flip, 0b0111110010);
    }
}

impl Tile {
    fn get_edge(edge: &str) -> u16 {
        u16::from_str_radix(&*edge.replace('.', "0").replace('#', "1"), 2).unwrap()
    }

    pub fn new(image: &str, id: u16) -> Self {
        let edge_top = Self::get_edge(image.lines().next().unwrap());
        let edge_top_flip = Self::get_edge(
            &*image
                .lines()
                .next()
                .map(|l| l.chars().rev().collect::<String>())
                .unwrap(),
        );
        let edge_right = Self::get_edge(
            &*image
                .lines()
                .map(|l| l.chars().last().unwrap())
                .collect::<String>(),
        );
        let edge_right_flip = Self::get_edge(
            &*image
                .lines()
                .rev()
                .map(|l| l.chars().last().unwrap())
                .collect::<String>(),
        );
        let edge_bottom = Self::get_edge(
            &*image
                .lines()
                .last()
                .unwrap()
                .chars()
                .rev()
                .collect::<String>(),
        );
        let edge_bottom_flip = Self::get_edge(&*image.lines().last().unwrap());
        let edge_left = Self::get_edge(
            &*image
                .lines()
                .rev()
                .map(|l| l.chars().next().unwrap())
                .collect::<String>(),
        );
        let edge_left_flip = Self::get_edge(
            &*image
                .lines()
                .map(|l| l.chars().next().unwrap())
                .collect::<String>(),
        );
        Self {
            id,
            edge_top,
            edge_top_flip,
            edge_right,
            edge_right_flip,
            edge_bottom,
            edge_bottom_flip,
            edge_left,
            edge_left_flip,
        }
    }
}
