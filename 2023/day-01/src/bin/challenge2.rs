fn main() {
    let data = include_str!("./input2.txt");
    let result = data
        .split('\n')
        .map(|row| {
            let mut row = row.replace("one", "o1e");
            row = row.replace("two", "t2o");
            row = row.replace("three", "t3e");
            row = row.replace("four", "f4r");
            row = row.replace("five", "f5e");
            row = row.replace("six", "s6x");
            row = row.replace("seven", "s7n");
            row = row.replace("eight", "e8t");
            row = row.replace("nine", "n9e");
            let mut number_iter = row.chars().filter(|char| char.is_digit(10));
            let mut val = number_iter.next().unwrap_or_else(|| '0').to_string();
            val.push(
                number_iter
                    .last()
                    .unwrap_or_else(|| val.chars().next().unwrap_or_else(|| '0')),
            );

            let val = val.parse::<i32>().unwrap();
            val
        })
        .sum::<i32>();
    println!("Result: {}", result)
}
