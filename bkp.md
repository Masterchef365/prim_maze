Think about your optimizing like a computer might
Each performance opt choice has a time-cost component, and a benefit component
Try to maximize benefit within your time cost
Turns out that's the 0/1 knapsack problem, for which a solution is np-complete... But a greedy approach can be good iirc

Weight is on-demand
```rust
struct Node {
    dist: u32,
    pos: (u32, u32),
}

fn make_problem(width: usize, height: usize) -> Vec<Node> {
    let mut nodes = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            nodes.push(Node {
                dist: u32::MAX,
                pos: (x as _, y as _),
            });
        }
    }
}
```

* See if it's still in Q
* See which neighbors it has which are in Q
* 

Remove lowest,
look at it's neighbors and assign new distances to them

How distances are assigned:
If it's not already u32::MAX, you can assign it. Do so randomly!

```rust
#[derive(Copy, Clone)]
struct Node {
    dist: u32,
    in_tree: bool,
    prev: Option<(u32, u32)>,
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

let width = 10;
let height = 10;
let mut nodes = vec![Node::new(); width * height];
let mut rng = rand::thread_rng();
let mut output = Vec::new();

loop {
    let mut current = None;
    let mut best_dist = u32::MAX;
    for y in 0..height {
        for x in 0..width {
            let pos = (x, y);
            let node = &nodes[pos];
            let dist = node.dist;
            if dist <= best_dist && !node.in_tree {
                best_dist = dist;
                current = Some(pos);
            }
        }
    }

    let current = match current {
        Some(c) => c,
        None => break nodes,
    };
    nodes[current].in_tree = true;

    
    for pos in neighborhood(current) {
        if let Some(node) = &mut nodes[pos] {
            if !node.in_tree && node.dist == u32::MAX {
                let candidate = rand.gen_range(0, u32::MAX-1);
                if candidate < node.dist {
                    node.dist = candidate;
                    node.prev = current;
                }
            }
        }
    }
}


```

