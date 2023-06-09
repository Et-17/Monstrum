use crate::random::Generator;

use super::Node;

/// This structure stores the mazes that are generated by this module. It also
/// has width and height information directly accessible to
/// - speed up the program
/// - make the struct easier to use
/// - avoid weird issues over owning the nodes array
pub struct Maze {
    pub nodes: Vec<Vec<Node>>,
    pub width: usize,
    pub height: usize,
}

impl Maze {
    /// Creates a new maze from the given nodes. Automatically calculates the
    /// size of the given maze: x by the column count, and y by the node count
    /// in the first column.
    pub fn new(nodes: &Vec<Vec<Node>>) -> Maze {
        Maze {
            // I'm pretty sure that the way to do this is to take a ref and then
            // own it so I don't copy the argument on accident or copy it onto
            // the stack when the executor didn't want to
            width: nodes.len(),
            height: nodes[0].len(),
            nodes: nodes.to_owned(),
        }
    }

    /// Creates a recursive backtracking maze of the given size.
    pub fn backtrack(width: usize, height: usize, generator: &mut Generator) -> Maze {
        let mut maze = vec![vec![Node::empty(); height]; width];
        let mut open_cells = vec![(0usize, 0usize)];

        while open_cells.len() > 0 {
            // This won't panic because we just verified len > 0
            let selected = *open_cells.last().unwrap();
            let neighbors = Self::open_neighbors(&maze, selected);
            // println!(
            //     "selected {:?} - {:} - {:}",
            //     selected,
            //     open_cells.len(),
            //     neighbors.bits().count_ones()
            // );
            let selected_neighbor = Self::select_neighbor(neighbors, generator);
            if selected_neighbor.is_none() {
                open_cells.pop();
                continue;
            }
            Self::bore_connection(selected, selected_neighbor.unwrap(), &mut maze);
            match selected_neighbor.unwrap() {
                Node::north => open_cells.push((selected.0, selected.1 - 1)),
                Node::east => open_cells.push((selected.0 + 1, selected.1)),
                Node::south => open_cells.push((selected.0, selected.1 + 1)),
                Node::west => open_cells.push((selected.0 - 1, selected.1)),
                _ => panic!(),
            }
        }

        Self::new(&maze)
    }

    /// Finds what directions have unmarked cells
    fn open_neighbors(nodes: &Vec<Vec<Node>>, cell: (usize, usize)) -> Node {
        let mut directions = Node::empty();
        if cell.1 != 0 && !nodes[cell.0][cell.1 - 1].contains(Node::marked) {
            directions.insert(Node::north);
        }
        if cell.0 != nodes.len() - 1 && !nodes[cell.0 + 1][cell.1].contains(Node::marked) {
            directions.insert(Node::east);
        }
        if cell.1 != nodes[0].len() - 1 && !nodes[cell.0][cell.1 + 1].contains(Node::marked) {
            directions.insert(Node::south);
        }
        if cell.0 != 0 && !nodes[cell.0 - 1][cell.1].contains(Node::marked) {
            directions.insert(Node::west);
        }
        return directions;
    }

    /// Select an open neighbor
    fn select_neighbor(neighbors: Node, generator: &mut Generator) -> Option<Node> {
        let mask = neighbors.bits();
        let open_neighbor_count = neighbors.bits().count_ones();
        if open_neighbor_count == 0 {
            return None;
        }
        let mut left = generator.next() % open_neighbor_count as u64;
        for i in 0..4 {
            if mask & (1 << i) >= 1 {
                if left == 0 {
                    return Node::from_bits(1 << i);
                }
                left -= 1;
            }
        }
        None
    }

    /// Bore connection
    fn bore_connection(first: (usize, usize), direction: Node, nodes: &mut Vec<Vec<Node>>) {
        // omg this function nests so hard but idk how i should refactor it
        match direction {
            Node::north => {
                nodes[first.0][first.1] |= Node::north;
                nodes[first.0][first.1 - 1] |= Node::marked | Node::south;
            }
            Node::east => {
                nodes[first.0][first.1] |= Node::east;
                nodes[first.0 + 1][first.1] |= Node::marked | Node::west;
            }
            Node::south => {
                nodes[first.0][first.1] |= Node::south;
                nodes[first.0][first.1 + 1] |= Node::marked | Node::north;
            }
            Node::west => {
                nodes[first.0][first.1] |= Node::west;
                nodes[first.0 - 1][first.1] |= Node::marked | Node::east;
            }
            _ => panic!(),
        }
    }
}
