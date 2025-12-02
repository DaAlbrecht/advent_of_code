fn main() {
    let file = std::fs::read_to_string("input").unwrap();
    dbg!(part_01(&file));
}

fn part_01(file: &str) -> usize {
    let id_ranges = file.split(',');
    let mut invalid_ids = Vec::new();

    for range in id_ranges {
        let (start, end) = range
            .split_once('-')
            .expect("input always has a `-`, otherwise its not valid");

        let start = start.trim().parse::<usize>().unwrap();
        let end = end.trim().parse::<usize>().unwrap();

        for id in start..=end {
            let product_id = id.to_string();
            if product_id[..product_id.len() / 2]
                == product_id[(product_id.len() / 2)..product_id.len()]
            {
                invalid_ids.push(id);
            }
        }
    }

    invalid_ids.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::part_01;

    #[test]
    fn test_01() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(1227775554, part_01(input));
    }
}
