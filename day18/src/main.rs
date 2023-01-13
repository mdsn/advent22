use std::collections::{HashMap, HashSet, VecDeque};
use std::io;
use std::io::prelude::*;

type Pos = [u8; 3];

fn adjacent_positions(pos: Pos) -> Vec<Pos> {
    let mut neighbors = vec![];
    let faces = [
        [1i8, 0, 0],
        [0, 1, 0],
        [0, 0, 1],
        [-1, 0, 0],
        [0, -1, 0],
        [0, 0, -1],
    ];
    let [x, y, z] = pos;
    for [x0, y0, z0] in faces
        .iter()
        .map(|[dx, dy, dz]| [x as i8 - dx, y as i8 - dy, z as i8 - dz])
        .filter(|[x0, y0, z0]| *x0 >= 0 && *y0 >= 0 && *z0 >= 0)
    {
        neighbors.push([x0 as u8, y0 as u8, z0 as u8]);
    }
    neighbors
}

fn neighbors(grid: &HashSet<Pos>, pos: Pos) -> Vec<Pos> {
    let mut neighbors = vec![];
    // if grid.contains(&pos) {
    for neighbor in adjacent_positions(pos)
        .into_iter()
        .filter(|[x0, y0, z0]| grid.contains(&[*x0, *y0, *z0]))
    {
        neighbors.push(neighbor);
    }
    // }

    neighbors
}

fn main() {
    let mut grid = HashSet::new();

    for line in io::stdin().lock().lines().map(Result::unwrap) {
        let cube: Pos = line
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        grid.insert(cube);
    }

    let mut counters = HashMap::new();
    for cube in &grid {
        let open_faces = 6 - neighbors(&grid, *cube).len();
        counters.insert(cube, open_faces);
    }

    let total: usize = counters.values().sum();
    dbg!(total);

    // idea: 3D flood fill.
    // list all adjacents. take one, flood fill with bfs, if it hits boundaries it's outside.
    // flood fill:
    //  take one from adjacent, mark it as visited, queue all neighbors not in grid. Repeat for
    //  each neighbor. If we hit boundaries, there is an open path from original adjacent to
    //  outside; mark all visited as "outside". Note that all visited are connected to it so are
    //  also open to the outside, so none of them are trapped.
    //
    //  if a flood fill runs out of neighbors without hitting boundaries, we have found a trapped
    //  pocket of air. Mark these nodes as "trapped".
    //
    //  if there are still nodes in `adjacent` not in `visited` or `trapped`, take one and run
    //  flood fill again.
    //
    //  once there are no more nodes in `adjacent` that aren't `outside` or `trapped`, we have
    //  effectively checked all the empty spaces around the grid. Take the intersection between the
    //  `trapped` nodes and the original `adjacent` nodes to get the cubes touching the trapped
    //  faces.
    let adjacent: HashSet<Pos> = HashSet::from_iter(grid.iter().flat_map(|q| {
        adjacent_positions(*q)
            .into_iter()
            .filter(|adj| !grid.contains(adj))
    }));
    let mut outside = HashSet::new();
    let mut all_visited = HashSet::new();
    let mut queue = VecDeque::new();

    loop {
        let mut visited = HashSet::new();
        // if adjacent - all_visited is empty, there are no more adjacents to check, break.
        if adjacent.difference(&all_visited).next().is_none() {
            break;
        }

        // take arbitrary cube from adjacent that is not `all_visited`
        let cube = adjacent.iter().find(|q| !all_visited.contains(*q)).unwrap();

        // put it in the queue
        queue.push_back(*cube);
        while !queue.is_empty() {
            let qb = queue.pop_front().unwrap();

            visited.insert(qb);
            all_visited.insert(qb);

            // if it touches the boundaries at 0 or 20
            if qb.iter().any(|xyz| *xyz == 0 || *xyz >= 20) {
                // mark visited as `outside`, clear visited
                visited.drain().for_each(|q| {
                    assert!(!outside.contains(&q));
                    outside.insert(q);
                });
                queue.clear();
                break;
            }

            // take every non-visited+non-all_visited neighbor that is not in the grid, queue them
            let ns = adjacent_positions(qb);
            for n in ns {
                if !queue.contains(&n) // why is this even necessary :S
                    && !visited.contains(&n)
                    && !all_visited.contains(&n)
                    && !grid.contains(&n)
                {
                    queue.push_back(n);
                }
            }
        }
    }

    // once we broke from the loop, adjacent - outside are the adjacent cubes that are trapped
    let trapped: HashSet<&Pos> = adjacent.difference(&outside).collect();

    // count faces of `trapped` that touch cubes in `grid`, subtract from `total`, report. The end
    counters.clear();
    for cube in trapped {
        let closed_faces = neighbors(&grid, *cube).len();
        counters.insert(cube, closed_faces);
    }

    let closed: usize = counters.values().sum();
    dbg!(total - closed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        let grid = HashSet::from([[0, 0, 0], [1, 0, 0]]);
        let p0 = [0, 0, 0];
        let ns = neighbors(&grid, p0);
        dbg!(ns);
    }
}
