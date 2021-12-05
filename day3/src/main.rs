#[derive(Debug)]
struct Column(Vec<char>);

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut columns = vec![Vec::new(); 12];

    for line in input.lines() {
        for (column, character) in line.chars().enumerate() {
            columns[column as usize].push(character);
        }
    }

    dbg!(columns[0].len());

    let gamma_rate_vec: Vec<char> = columns
        .iter()
        .map(|col| {
            let zero_count = col.iter().filter(|x| **x == '0').count();
            let one_count = col.iter().filter(|x| **x == '1').count();

            if zero_count > one_count {
                '0'
            } else {
                '1'
            }
        })
        .collect();

    let gamma_rate_as_string = String::from_iter(gamma_rate_vec);
    let gamma_rate_decimal = isize::from_str_radix(gamma_rate_as_string.as_str(), 2).unwrap();

    let epsilon_rate_vec: Vec<char> = columns
        .iter()
        .map(|col| {
            let zero_count = col.iter().filter(|x| **x == '0').count();
            let one_count = col.iter().filter(|x| **x == '1').count();

            if zero_count < one_count {
                '0'
            } else {
                '1'
            }
        })
        .collect();

    let epsilon_rate_as_string = String::from_iter(epsilon_rate_vec);
    let epsilon_rate_decimal = isize::from_str_radix(epsilon_rate_as_string.as_str(), 2).unwrap();

    let power_consumption = gamma_rate_decimal * epsilon_rate_decimal;
    dbg!(power_consumption);
}
