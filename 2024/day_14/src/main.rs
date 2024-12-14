use glam::IVec2;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_01: {}", part_01(input.as_str(), (101, 103).into()));
}

fn part_01(input: &str, grid_size: IVec2) -> usize {
    let mut robots = input
        .lines()
        .map(|line| {
            let (pos, velocity) = line.split_once(' ').unwrap();
            let (pos_x, pos_y) = pos.strip_prefix("p=").unwrap().split_once(',').unwrap();
            let (vel_x, vel_y) = velocity
                .strip_prefix("v=")
                .unwrap()
                .split_once(',')
                .unwrap();
            let pos: IVec2 = IVec2::new(pos_x.parse().unwrap(), pos_y.parse().unwrap());
            let velocity: IVec2 = IVec2::new(vel_x.parse().unwrap(), vel_y.parse().unwrap());
            (pos, velocity)
        })
        .collect::<Vec<(IVec2, IVec2)>>();

    for _i in 0..100 {
        for robot in &mut robots {
            robot.0 = (robot.0 + robot.1).rem_euclid(grid_size);
        }
    }

    let mid_x = grid_size.x / 2;
    let mid_y = grid_size.y / 2;

    let quadrants = [
        ((0..mid_x), (0..mid_y)),
        ((mid_x..grid_size.x), (0..mid_y)),
        ((0..mid_x), (mid_y..grid_size.y)),
        ((mid_x..grid_size.x), (mid_y..grid_size.y)),
    ];

    quadrants
        .iter()
        .map(|(x_range, y_range)| {
            robots
                .iter()
                .filter(|(pos, _)| {
                    let x = pos.x;
                    let y = pos.y;
                    x_range.contains(&x) && y_range.contains(&y) && x != mid_x && y != mid_y
                })
                .count()
        })
        .product()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_01() {
        let test_input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(super::part_01(test_input, (11, 7).into()), 12);
    }
}
