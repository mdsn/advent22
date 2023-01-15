use std::collections::{HashMap, HashSet};
use std::io::prelude::*;

#[derive(Debug)]
enum Monkey {
    K(u64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

impl Monkey {
    fn is_k(&self) -> bool {
        matches!(self, Monkey::K(_))
    }

    fn needs(&self, name: &str) -> bool {
        match self {
            Monkey::K(_) => false,
            Monkey::Add(a, b) | Monkey::Sub(a, b) | Monkey::Mul(a, b) | Monkey::Div(a, b) => {
                a == name || b == name
            }
        }
    }
}

fn main() {
    let monkeys: HashMap<String, Monkey> = std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let (name, task) = line.split_once(": ").unwrap();
            let op = task
                .parse::<u64>()
                .map(Monkey::K)
                .or_else(|_| {
                    let parts = task.split_whitespace().collect::<Vec<_>>();
                    let (a, b) = (parts[0].to_string(), parts[2].to_string());
                    Ok::<Monkey, ()>(match parts[1] {
                        "+" => Monkey::Add(a, b),
                        "-" => Monkey::Sub(a, b),
                        "*" => Monkey::Mul(a, b),
                        "/" => Monkey::Div(a, b),
                        _ => unreachable!(),
                    })
                })
                .unwrap();
            (name.to_string(), op)
        })
        .collect();

    // root: pppw + sjmn
    // dbpl: 5
    // cczh: sllz + lgvd
    // zczc: 2
    // ptdq: humn - dvpt
    // dvpt: 3
    // lfqf: 4
    // humn: 5
    // ljgn: 2
    // sjmn: drzm * dbpl
    // sllz: 4
    // pppw: cczh / lfqf
    // lgvd: ljgn * ptdq
    // drzm: hmdt - zczc
    // hmdt: 32

    let queue = toposort(&monkeys);
    let mut numbers: HashMap<String, u64> = [].into();
    for name in queue {
        let monkey = &monkeys[&name];
        if let Monkey::K(k) = monkey {
            numbers.insert(name, *k);
        } else {
            let (a, b) = match monkey {
                Monkey::Add(a, b) | Monkey::Sub(a, b) | Monkey::Mul(a, b) | Monkey::Div(a, b) => {
                    (numbers[a], numbers[b])
                }
                _ => unreachable!(),
            };
            let k = match monkey {
                Monkey::Add(_, _) => a + b,
                Monkey::Sub(_, _) => a - b,
                Monkey::Mul(_, _) => a * b,
                Monkey::Div(_, _) => a / b,
                _ => unreachable!(),
            };
            numbers.insert(name, k);
        }
    }
    dbg!(&numbers["root"]);
}

fn toposort(monkeys: &HashMap<String, Monkey>) -> Vec<String> {
    let mut queue = vec![];
    let mut starter: HashSet<&String> = monkeys
        .iter()
        .filter_map(|(name, op)| op.is_k().then_some(name))
        .collect();

    loop {
        let ready = starter.iter().next().cloned().unwrap();
        starter.remove(ready);
        queue.push(ready.clone());

        if ready == "root" {
            break;
        }

        let (name, monkey) = monkeys
            .iter()
            .find(|(_, monkey)| monkey.needs(ready))
            .unwrap();

        assert!(!monkey.is_k());
        match monkey {
            Monkey::Add(a, b) | Monkey::Sub(a, b) | Monkey::Mul(a, b) | Monkey::Div(a, b) => {
                if queue.contains(a) && queue.contains(b) {
                    starter.insert(name);
                }
            }
            Monkey::K(_) => unreachable!(),
        }
    }
    queue
}
