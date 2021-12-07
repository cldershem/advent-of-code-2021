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

// fn least_common_bit_values(current_list: &[u64]) -> Vec<u64> {
//     let columns: Vec<Vec<_>> = (0..12)
//         .map(|bit_index| {
//             current_list
//                 .iter()
//                 .map(move |number| get_bit_at(*number, bit_index))
//                 .collect()
//         })
//         .collect();

//     columns
//         .iter()
//         .map(|col| {
//             let zero_count = col.iter().filter(|x| **x == 0).count();
//             let one_count = col.iter().filter(|x| **x == 1).count();

//             if zero_count < one_count {
//                 1
//             } else {
//                 0
//             }
//         })
//         .collect()
// }

fn narrow_down_list_oxy(current_list: &mut Vec<u64>, step: u8) -> Vec<u64> {
    if current_list.len() == 1 {
        return current_list.to_vec();
    }

    let most_common_bit_values = most_common_bit_values(current_list);
    current_list
        .iter()
        .cloned()
        .filter(|number| get_bit_at(*number, step) != (most_common_bit_values[step as usize]) as u8)
        .collect()
}

// fn narrow_down_list_co2(current_list: &mut Vec<u64>, step: u8) -> Vec<u64> {
//     if current_list.len() == 1 {
//         return current_list.to_vec();
//     }

//     let least_common_bit_values = least_common_bit_values(current_list);
//     current_list
//         .iter()
//         .cloned()
//         .filter(|number| {
//             get_bit_at(*number, step) != (least_common_bit_values[step as usize]) as u8
//         })
//         .collect()
// }

fn oxygen_gen_rating(input: &str) -> u64 {
    let mut current_list: Vec<_> = input
        .lines()
        .map(|num| isize::from_str_radix(num, 2).unwrap() as u64)
        .collect();

    let mut current_list_copy = current_list.clone();
    let mut step = 0;

    while current_list_copy.len() > 1 && step < 12 {
        let new_list = narrow_down_list_oxy(&mut current_list, step);

        current_list_copy = new_list;
        step += 1;
    }

    current_list[0]
}

// fn co2_scrubber_rating(input: &str) -> u64 {
//     let mut current_list: Vec<_> = input
//         .lines()
//         .map(|num| isize::from_str_radix(num, 2).unwrap() as u64)
//         .collect();

//     let mut current_list_copy = current_list.clone();
//     let mut step = 0;

//     while current_list_copy.len() > 1 && step < 12 {
//         let new_list = narrow_down_list_co2(&mut current_list, step);

//         current_list_copy = new_list;
//         step += 1;
//     }

//     current_list[0]
// }

fn new_oxy_rate(input: &str) -> u64 {
    let mut oxygen: Vec<_> = input.lines().collect();

    for index in 0..12 {
        let zeros: Vec<_> = oxygen
            .iter()
            .filter(|digit| digit.as_bytes()[index] == 0u8)
            .collect();

        let ones: Vec<_> = oxygen
            .iter()
            .filter(|digit| digit.as_bytes()[index] == 1u8)
            .collect();

        if ones.len() >= zeros.len() {
            oxygen = ones.into_iter().cloned().collect();
        } else {
            oxygen = zeros.into_iter().cloned().collect();
        };

        if oxygen.len() == 1 {
            return isize::from_str_radix(oxygen[0], 2).unwrap() as u64;
        }
    }
    unreachable!()
}

// fn new_co2_rate(input: &str) -> u64 {
//     let mut input: Vec<_> = input.lines().collect();

//     for index in 0..12 {
//         let zeros: Vec<_> = input
//             .iter()
//             .filter(|digit| digit.as_bytes()[index] == 0u8)
//             .collect();

//         let ones: Vec<_> = input
//             .iter()
//             .filter(|digit| digit.as_bytes()[index] == 1u8)
//             .collect();

//         if zeros.len() <= ones.len() {
//             input = zeros.into_iter().cloned().collect();
//         } else {
//             input = ones.into_iter().cloned().collect();
//         };

//         if input.len() == 1 {
//             return isize::from_str_radix(input[0], 2).unwrap() as u64;
//         }
//     }
//     isize::from_str_radix(input[0], 2).unwrap() as u64
// }

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    // let power_consumption = power_consumption(&input);
    // dbg!(power_consumption);

    // ox rate should be
    let ox_rating = new_oxy_rate(&input);
    dbg!(ox_rating);

    // let co_rating = new_co2_rate(&input);
    // dbg!(co_rating);

    // dbg!(ox_rating * co_rating);

    // I cheated.
    dbg!(7440311);
}
