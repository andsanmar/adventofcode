use aoc::input_file;

/// Represents the input data structure containing vectors of 6 integers, representing the coefficients and target values of x and y for each of the buttons
#[derive(Debug)]
struct Data {
    claws: Vec<[i64; 6]>,
}

impl std::str::FromStr for Data {
    type Err = std::io::Error;

    /// Parses input string into Data struct
    /// Input format is blocks of numbers separated by newlines,
    /// with numbers separated by +, comma, newline or equals
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let claws = input
            .split("\n\n")
            .map(|block| {
                let numbers: Vec<i64> = block
                    .split(|c| c == '+' || c == ',' || c == '\n' || c == '=')
                    .filter_map(|number_str| number_str.parse::<i64>().ok())
                    .collect();
                numbers.try_into().unwrap()
            })
            .collect();
        Ok(Data { claws })
    }
}

/// Calculates coin values based on input parameters
/// Parameters:
/// - xa, ya: First coin coefficients
/// - xb, yb: Second coin coefficients  
/// - xr, yr: Target values
/// - star2: Boolean flag for part 2 calculation
/// Returns: Option containing (a,b) coin values if solution exists
fn coins(
    [a_x, a_y, b_x, b_y, mut target_x, mut target_y]: &[i64; 6],
    star2: bool,
) -> Option<(i64, i64)> {
    if star2 {
        target_x += 10000000000000;
        target_y += 10000000000000;
    }

    let cross_product_b = a_y * b_x;
    let cross_product_target = a_y * target_x;
    let cross_product_b_2 = a_x * b_y;
    let cross_product_target_2 = a_x * target_y;

    if (cross_product_target_2 - cross_product_target) % (cross_product_b_2 - cross_product_b) != 0
    {
        None
    } else {
        let coin_b =
            (cross_product_target_2 - cross_product_target) / (cross_product_b_2 - cross_product_b);
        if (target_x - (coin_b * b_x)) % a_x != 0 {
            None
        } else {
            let coin_a = (target_x - (coin_b * b_x)) / a_x;
            Some((coin_a, coin_b))
        }
    }
}

/// Calculates and prints solutions for both star1 and star2
fn stars(Data { claws }: Data) {
    println!(
        "Star1: {:?}",
        claws
            .iter()
            .filter_map(|coefficients| coins(coefficients, false))
            .map(|(coin_a, coin_b)| coin_a * 3 + coin_b)
            .sum::<i64>()
    );
    println!(
        "Star2: {:?}",
        claws
            .iter()
            .filter_map(|coefficients| coins(coefficients, true))
            .map(|(coin_a, coin_b)| coin_a * 3 + coin_b)
            .sum::<i64>()
    );
}

/// Main function that reads input file and processes the data
fn main() -> Result<(), std::io::Error> {
    let input_raw = std::fs::read_to_string(input_file(file!()))?;
    let parsed_data: Data = input_raw.parse()?;
    stars(parsed_data);
    Ok(())
}
