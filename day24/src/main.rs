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
    pos: (usize, usize),
    dir: Dir,
}

#[derive(Debug)]
struct TimeSlice {
    blizzards: HashSet<Blizzard>,
}

impl TimeSlice {
    fn blizzard_at(&self, x: usize, y: usize) -> Option<&Blizzard> {
        self.blizzards
            .iter()
            .find(|Blizzard { pos: (x0, y0), .. }| x == *x0 && y == *y0)
    }
}

#[derive(Debug)]
struct Time {
    width: usize,
    height: usize,
    slices: Vec<TimeSlice>,
}

impl Time {
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

    let width = lines[0].len() - 2;
    let height = lines.len();

    let mut blizzards = HashSet::new();
    for (y, row) in lines.iter().enumerate() {
        for (x, c) in row.chars().skip(1).enumerate() {
            let bliz = Blizzard {
                pos: (x, y),
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

    let time = Time {
        width,
        height,
        slices: vec![TimeSlice { blizzards }],
    };

    time.print_last();
}
