fn vec_of_bits_to_int(bits: Vec<u8>) -> u64 {
    bits.iter()
        .fold((0, 1), |(acc, mul), &bit| {
            (acc + (mul * (1 & bit as u64)), mul.wrapping_add(mul))
        })
        .0
}

// stolen from https://old.reddit.com/r/rust/comments/3xgeo0/biginner_question_how_can_i_get_the_value_of_a/
fn get_bit_at(input: u64, n: u8) -> u8 {
    if n < 64 && (input & (1 << n) != 0) {
        1
    } else {
        0
    }
}

fn power_consumption(input: &str) -> u64 {
    let mut columns = vec![Vec::new(); 12];

    let current_list: Vec<_> = input
        .lines()
        // .map(|num| num.parse::<u64>().unwrap())
        .map(|num| isize::from_str_radix(num, 2).unwrap() as u64)
        .collect();

    for number in current_list.iter() {
        for bit_index in 0..12 {
            let bit_value = get_bit_at(*number, bit_index);
            columns[bit_index as usize].push(bit_value);
        }
    }

    let gamma_rate: Vec<_> = columns
        .iter()
        .map(|col| {
            let zero_count = col.iter().filter(|x| **x == 0).count();
            let one_count = col.iter().filter(|x| **x == 1).count();

            if zero_count > one_count {
                0
            } else {
                1
            }
        })
        .collect();

    let epsilon_rate: Vec<_> = columns
        .iter()
        .map(|col| {
            let zero_count = col.iter().filter(|x| **x == 0).count();
            let one_count = col.iter().filter(|x| **x == 1).count();

            if zero_count < one_count {
                0
            } else {
                1
            }
        })
        .collect();

    vec_of_bits_to_int(gamma_rate) * vec_of_bits_to_int(epsilon_rate)
}

fn most_common_bit_values(current_list: &[u64]) -> Vec<u64> {
    let columns: Vec<Vec<_>> = (0..12)
        .map(|bit_index| {
            current_list
                .iter()
                .map(move |number| get_bit_at(*number, bit_index))
                .collect()
        })
        .collect();

    columns
        .iter()
        .map(|col| {
            let zero_count = col.iter().filter(|x| **x == 0).count();
            let one_count = col.iter().filter(|x| **x == 1).count();

            if zero_count < one_count {
                0
            } else {
                1
            }
        })
        .collect()
}

fn narrow_down_list(current_list: &mut Vec<u64>, step: u8) -> Vec<u64> {
    let most_common_bit_values = most_common_bit_values(current_list);

    let mut new_list: Vec<_> = vec![];

    // current_list
    //     .iter()
    //     .cloned()
    //     .filter(|number| get_bit_at(*number, step) == (most_common_bit_values[step as usize]) as u8)
    //     .collect()

    for number in current_list {
        if get_bit_at(*number, step) == (most_common_bit_values[step as usize]) as u8 {
            new_list.push(*number);
        }
    }

    dbg!(new_list.len());
    new_list
}

fn oxygen_gen_rating(input: &str) -> u64 {
    let mut current_list: Vec<_> = input
        .lines()
        .map(|num| isize::from_str_radix(num, 2).unwrap() as u64)
        .collect();

    for bit_index in 0..12 {
        if current_list.len() == 1 {
            break;
        }

        let new_list = narrow_down_list(&mut current_list, bit_index);
        dbg!(new_list.len());
        current_list = new_list;
    }
    dbg!(&current_list);

    current_list[0]
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    // let power_consumption = power_consumption(&input);
    // dbg!(power_consumption);

    let ox_rating = oxygen_gen_rating(&input);
    dbg!(ox_rating);

    // let co_rating = co2_scrubber_rating(&input);
    // dbg!(co_rating);
}
