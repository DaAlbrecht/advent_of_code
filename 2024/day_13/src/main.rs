use anyhow::Result;

use good_lp::{constraint, default_solver, variables, Solution, SolverModel};

type Machine = ((f64, f64), (f64, f64), (f64, f64));

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_01: {}", part_01(input.as_str()));
    println!("part_02: {}", part_02(input.as_str()));
}

fn part_01(input: &str) -> usize {
    let machines = input
        .split("\n\n")
        .map(|machine| {
            let mut iter = machine.lines();
            let (x, y) = iter
                .next()
                .expect("button A to be present")
                .split_once(',')
                .expect("each line of a machine in aoc has a ,");
            let button_a_x = x
                .strip_prefix("Button A: X+")
                .unwrap()
                .parse::<f64>()
                .unwrap();
            let button_a_y = y.strip_prefix(" Y+").unwrap().parse::<f64>().unwrap();
            let (x, y) = iter
                .next()
                .expect("button B to be present")
                .split_once(',')
                .expect("each line of a machine in aoc has a ,");
            let button_b_x = x
                .strip_prefix("Button B: X+")
                .unwrap()
                .parse::<f64>()
                .unwrap();
            let button_b_y = y.strip_prefix(" Y+").unwrap().parse::<f64>().unwrap();
            let (x, y) = iter
                .next()
                .expect("Prize to be present")
                .split_once(',')
                .expect("each line of a machine in aoc has a ,");
            let prize_x = x.strip_prefix("Prize: X=").unwrap().parse::<f64>().unwrap();
            let prize_y = y.strip_prefix(" Y=").unwrap().parse::<f64>().unwrap();
            (
                (button_a_x, button_a_y),
                (button_b_x, button_b_y),
                (prize_x, prize_y),
            )
        })
        .collect::<Vec<Machine>>();

    machines
        .iter()
        .filter_map(|machine| match solve(*machine) {
            Ok(result) => {
                let a = result.0.round() as usize * 3;
                let b = result.1.round() as usize;

                Some(a + b)
            }
            Err(_) => None,
        })
        .sum()
}

fn part_02(input: &str) -> usize {
    let machines = input
        .split("\n\n")
        .map(|machine| {
            let mut iter = machine.lines();
            let (x, y) = iter
                .next()
                .expect("button A to be present")
                .split_once(',')
                .expect("each line of a machine in aoc has a ,");
            let button_a_x = x
                .strip_prefix("Button A: X+")
                .unwrap()
                .parse::<f64>()
                .unwrap();
            let button_a_y = y.strip_prefix(" Y+").unwrap().parse::<f64>().unwrap();
            let (x, y) = iter
                .next()
                .expect("button B to be present")
                .split_once(',')
                .expect("each line of a machine in aoc has a ,");
            let button_b_x = x
                .strip_prefix("Button B: X+")
                .unwrap()
                .parse::<f64>()
                .unwrap();
            let button_b_y = y.strip_prefix(" Y+").unwrap().parse::<f64>().unwrap();
            let (x, y) = iter
                .next()
                .expect("Prize to be present")
                .split_once(',')
                .expect("each line of a machine in aoc has a ,");
            let prize_x = x.strip_prefix("Prize: X=").unwrap().parse::<f64>().unwrap();
            let prize_y = y.strip_prefix(" Y=").unwrap().parse::<f64>().unwrap();
            (
                (button_a_x, button_a_y),
                (button_b_x, button_b_y),
                (prize_x + 10000000000000.0, prize_y + 10000000000000.0),
            )
        })
        .collect::<Vec<Machine>>();

    machines
        .iter()
        .filter_map(|machine| match solve(*machine) {
            Ok(result) => {
                let a = result.0.round() as usize * 3;
                let b = result.1.round() as usize;

                Some(a + b)
            }
            Err(_) => None,
        })
        .sum()
}

fn solve(machine: Machine) -> Result<(f64, f64)> {
    variables! {
       vars:
           a;
           b;
    }
    let result = vars
        .minimise(3 * a + b)
        .using(default_solver)
        .with(constraint!(
            a * machine.0 .0 + b * machine.1 .0 == machine.2 .0
        ))
        .with(constraint!(
            a * machine.0 .1 + b * machine.1 .1 == machine.2 .1
        ))
        .solve()?;

    let temp_a = result.value(a).round() as u64;
    let temp_b = result.value(b).round() as u64;
    let temp_a_x = temp_a * machine.0 .0 as u64;
    let temp_a_y = temp_a * machine.0 .1 as u64;
    let temp_b_x = temp_b * machine.1 .0 as u64;
    let temp_b_y = temp_b * machine.1 .1 as u64;
    let temp_Res = (temp_a_x + temp_b_x, temp_a_y + temp_b_y);

    if temp_Res.0 == machine.2 .0 as u64 && temp_Res.1 == machine.2 .1 as u64 {
        return Ok((result.value(a), result.value(b)));
    }
    Err(anyhow::anyhow!("No valid solution"))
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_01() {
        let test_input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(super::part_01(test_input), 1);
    }
}
