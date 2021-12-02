fn part_one(input: &[i32]) -> i32 {
    let mut count = 0;

    for thing in input.windows(2) {
        if thing[0] < thing[1] {
            count += 1;
        }
    }

    count
}

fn part_two(input: &[i32]) -> i32 {
    let windows: Vec<i32> = input.windows(3).map(|nums| nums.iter().sum()).collect();

    part_one(&windows)
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let input: Vec<_> = input
        .lines()
        .map(|num| num.trim().parse::<i32>().unwrap())
        .collect();

    dbg!(&input.len());

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    dbg!(answer_one);
    dbg!(answer_two);
}
