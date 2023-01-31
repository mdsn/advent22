use std::collections::HashSet;
use std::io::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Dir {
    U,
    D,
    L,
    R,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Blizzard {
    pos: (i32, i32),
    dir: Dir,
}

#[derive(Debug)]
struct TimeSlice {
    blizzards: HashSet<Blizzard>,
}

impl TimeSlice {
    fn blizzard_at(&self, x: i32, y: i32) -> Option<&Blizzard> {
        self.blizzards
            .iter()
            .find(|Blizzard { pos: (x0, y0), .. }| x == *x0 && y == *y0)
    }
}

#[derive(Debug)]
struct Time {
    width: i32,
    height: i32,
    slices: Vec<TimeSlice>,
}

impl Time {
    fn tick(&mut self) {
        let blizzards = self
            .slices
            .last()
            .unwrap()
            .blizzards
            .iter()
            .map(|b| {
                let (x0, y0) = b.pos;
                Blizzard {
                    pos: match b.dir {
                        Dir::U => (x0, if y0 == 0 { self.height - 1 } else { y0 - 1 }),
                        Dir::D => (x0, (y0 + 1) % self.height),
                        Dir::L => (if x0 == 0 { self.width - 1 } else { x0 - 1 }, y0),
                        Dir::R => ((x0 + 1) % self.width, y0),
                    },
                    dir: b.dir,
                }
            })
            .collect::<HashSet<_>>();
        self.slices.push(TimeSlice { blizzards });
    }

    #[allow(dead_code)]
    fn print_t(&self, t: usize) {
        assert!(t < self.slices.len());
        let slice = &self.slices[t];
        // header
        print!("#.");
        for _ in 0..self.width {
            print!("#");
        }
        println!();
        // rows
        for y in 0..self.height {
            print!("#");
            for x in 0..self.width {
                if let Some(Blizzard { dir, .. }) = slice.blizzard_at(x, y) {
                    print!(
                        "{}",
                        match dir {
                            Dir::U => '^',
                            Dir::D => 'v',
                            Dir::L => '<',
                            Dir::R => '>',
                        }
                    );
                } else {
                    print!(".");
                }
            }
            println!("#");
        }
        // footer
        for _ in 0..self.width {
            print!("#");
        }
        println!(".#");
    }

    #[allow(dead_code)]
    fn print_last(&self) {
        self.print_t(self.slices.len() - 1);
    }
}

fn main() {
    let mut lines = std::io::stdin()
        .lock()
        .lines()
        .skip(1)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    lines.pop();

    let width = (lines[0].len() - 2) as i32;
    let height = lines.len() as i32;

    let mut blizzards = HashSet::new();
    for (y, row) in lines.iter().enumerate() {
        for (x, c) in row.chars().skip(1).enumerate() {
            let bliz = Blizzard {
                pos: (x as i32, y as i32),
                dir: match c {
                    '>' => Dir::R,
                    '^' => Dir::U,
                    '<' => Dir::L,
                    'v' => Dir::D,
                    _ => continue,
                },
            };
            blizzards.insert(bliz);
        }
    }

    let mut time = Time {
        width,
        height,
        slices: vec![TimeSlice { blizzards }],
    };

    let mut seen = HashSet::new();
    let part1 = dfs(&mut time, START, 0, &mut seen);
    dbg!(part1);
}

const START: (i32, i32) = (0, -1);

fn dfs(time: &mut Time, pos: (i32, i32), t: usize, seen: &mut HashSet<(i32, i32, usize)>) -> usize {
    let (x, y) = pos;
    seen.insert((x, y, t));

    assert!(time.slices[t].blizzard_at(x, y).is_none());

    // hard stop at t = 500 not to blow the stack
    if t == 500 {
        return usize::MAX;
    }

    // are we on target yet?
    if x == time.width - 1 && y == time.height - 1 {
        return t + 1;
    }

    // if we don't have the next time slice yet, tick
    if t == time.slices.len() - 1 {
        time.tick();
    }

    // call dfs on each available neighbor at t + 1
    let next_slice = &time.slices[t + 1];
    // available neighbors, two cases
    // - if pos == START: if (0, 0, t+1) is available, take it, otherwise take (0, -1, t+1).
    let neighbors = if pos == START {
        if next_slice.blizzard_at(0, 0).is_none() {
            vec![(0, 0)]
        } else {
            vec![(0, -1)]
        }
    // - otherwise, 4-directional neighbors at t+1 plus current position at t+1
    } else {
        let directions: Vec<(i32, i32)> = [(-1, 0), (0, -1), (0, 1), (1, 0), (0, 0)]
            .into_iter()
            .filter_map(|(dx, dy)| {
                let (x0, y0) = (x + dx, y + dy);
                if x0 < 0
                    || x0 >= time.width
                    || y0 < 0
                    || y0 >= time.height
                    || seen.contains(&(x0, y0, t + 1))
                    || next_slice.blizzard_at(x0, y0).is_some()
                {
                    None
                } else {
                    Some((x0, y0))
                }
            })
            .collect();
        // no available moves, bail
        if directions.is_empty() {
            return usize::MAX;
        } else {
            directions
        }
    };

    neighbors
        .into_iter()
        .map(|(x0, y0)| dfs(time, (x0, y0), t + 1, seen))
        .min()
        .unwrap()
}

// That's not the right answer; your answer is too low. Curiously, it's the right answer for
// someone else; you might be logged in to the wrong account or just unlucky. (You guessed 238.)
