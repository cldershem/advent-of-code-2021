fn vec_of_bits_to_int(bits: Vec<u8>) -> u64 {
    let out = bits
        .iter()
        .fold((0, 1), |(acc, mul), &bit| {
            (acc + (mul * (1 & bit as u64)), mul.wrapping_add(mul))
        })
        .0;

    dbg!(&out);
    out
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

    let gamma_rate_decimal = vec_of_bits_to_int(gamma_rate);
    let epsilon_rate_decimal = vec_of_bits_to_int(epsilon_rate);

    gamma_rate_decimal * epsilon_rate_decimal
}

// stolen from https://old.reddit.com/r/rust/comments/3xgeo0/biginner_question_how_can_i_get_the_value_of_a/
fn get_bit_at(input: u64, n: u8) -> u8 {
    if n < 64 && (input & (1 << n) != 0) {
        1
    } else {
        0
    }
}

// fn narrow_down_most(current_list: Vec<u64>, step: u8) -> Vec<u64> {
//     if current_list.len() == 1 {
//         current_list
//     } else {
//         for item in current_list {
//             let col = current_list.iter().map(|line| line[step]).collect();
//             let zero_count = col.iter().filter(|x| **x == '0').count();
//             let one_count = col.iter().filter(|x| **x == '1').count();

//         // if zero_count > one_count {
//         //     0
//         // } else {
//         //     1
//         // }
//         }
//     }
// }

// fn oxygen_gen_rating(input: &str) -> u64 {
//     let mut columns = vec![Vec::new(); 12];

//     let mut current_list: Vec<_> = input
//         .lines()
//         .map(|num| num.parse::<u64>().unwrap())
//         .collect();

//     for (number_index, number) in current_list.iter().enumerate() {
//         for bit_index in 0..12 {
//             let bit_value = get_bit_at(*number, bit_index);
//             columns[bit_index as usize].push(bit_value);
//         }
//     }

//     dbg!(columns.len());
//     dbg!(columns[0].len());

//     // let gamma_rate: [u8; 12];

//     // dbg!(current_list);

//     // while current_list.len() > 1 {
//     //     for place in 1..=12 {
//     //         current_list = narrow_down_most(current_list, place);
//     //     }
//     // }

//     // let most_common_nums: Vec<_> = columns
//     //     .iter()
//     //     .enumerate()
//     //     .map(|(index, col)| {
//     //         let zero_count = col.iter().filter(|x| **x == '0').count();
//     //         let one_count = col.iter().filter(|x| **x == '1').count();

//     //         if zero_count > one_count {
//     //             0
//     //         } else {
//     //             1
//     //         }
//     //     })
//     //     .collect();

//     // dbg!(most_common_nums);

//     0
// }

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let power_consumption = power_consumption(&input);
    dbg!(power_consumption);

    // oxygen_gen_rating(&input);
}
