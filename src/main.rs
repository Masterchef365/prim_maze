use rand::Rng;
use klystron::Vertex;
use anyhow::Result;

mod display;

fn main() -> Result<()> {
    let width = 50;
    let height = 100;

    let start = std::time::Instant::now();
    let nodes = maze(width, height);
    println!("Maze time: {}s", start.elapsed().as_secs_f32());

    let start = std::time::Instant::now();
    let indices: Vec<u16> = line_indices(&nodes).into_iter().map(|i| i as u16).collect();
    let vertices = vertex_mesh_dist(width, height, &nodes);
    println!("Mesh time: {}s", start.elapsed().as_secs_f32());

    dbg!(vertices.len());
    dbg!(indices.len());
    display::visualize((vertices, indices), false)
}

fn vertex_mesh_dist(width: usize, height: usize, nodes: &[Node]) -> Vec<Vertex> {
    let mut vertices = Vec::with_capacity(width * height);
    for y in 0..height {
        let y = y as f32 / height as f32;
        for x in 0..width {
            let x = x as f32 / width as f32;
            let dist = nodes[vertices.len()].dist as f32 / u32::MAX as f32;
            vertices.push(Vertex {
                pos: [x, y, 0.],
                color: [x, y, dist],
            });
        }
    }
    vertices
}


#[derive(Copy, Clone)]
struct Node {
    dist: u32,
    in_tree: bool,
    prev: Option<usize>,
}

impl Node {
    pub const fn new() -> Self {
        Self {
            dist: u32::MAX,
            in_tree: false,
            prev: None,
        }
    }
}

fn maze(width: usize, height: usize) -> Vec<Node> {
    let mut nodes = vec![Node::new(); width * height];
    let mut rng = rand::thread_rng();

    loop {
        let mut current = None;
        let mut best_dist = u32::MAX;
        for (pos, node) in nodes.iter().enumerate() {
            let dist = node.dist;
            if dist <= best_dist && !node.in_tree {
                best_dist = dist;
                current = Some(pos);
            }
        }

        let current = match current {
            Some(c) => c,
            None => break nodes,
        };
        nodes[current].in_tree = true;

        for pos in neighborhood(current, width, height) {
            let node = &mut nodes[pos];
            if !node.in_tree && node.dist == u32::MAX {
                let candidate = rng.gen_range(0..u32::MAX - 1);
                if candidate < node.dist {
                    node.dist = candidate;
                    node.prev = Some(current);
                }
            }
        }
    }
}

fn line_indices(nodes: &[Node]) -> Vec<usize> {
    let mut visited = std::collections::HashSet::new();
    let mut indices = Vec::new();
    for idx in 0..nodes.len() {
        let mut current = idx;
        indices.push(current);
        while let Some(prev) = nodes[current].prev {
            indices.push(prev);
            indices.push(prev);
            if visited.contains(&prev) {
                break;
            } else {
                visited.insert(prev);
            }
            current = prev;
        }
        indices.push(current);
    }
    indices
}

fn neighborhood(idx: usize, width: usize, height: usize) -> impl Iterator<Item = usize> {
    debug_assert!(width > 0 && height > 0);
    debug_assert!(idx < width * height);
    std::iter::empty()
        .chain((idx > width).then(|| idx - width))
        .chain((idx > 0 && (idx - 1) / width == idx / width).then(|| idx - 1))
        .chain((idx + width < width * height).then(|| idx + width))
        .chain((idx + 1 < width * height && (idx + 1) / width == idx / width).then(|| idx + 1))
}
