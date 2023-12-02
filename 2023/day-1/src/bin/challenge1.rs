fn main() {
    let data = include_str!("./input1.txt");
    let result = data
        .split('\n')
        .map(|row| {
            let mut number_iter = row.chars().filter(|char| char.is_digit(10));
            let mut val = number_iter.next().unwrap_or_else(|| '0').to_string();
            val.push(
                number_iter
                    .last()
                    .unwrap_or_else(|| val.chars().next().unwrap_or_else(|| '0')),
            );

            let val = val.parse::<i32>().unwrap();
            println!("{}", val);
            val
        })
        .sum::<i32>();
    println!("{}", result)
}
