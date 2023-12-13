use std::{env, fs};

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("09/example_input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let lines = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(str::parse::<i32>)
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let predictions = lines.iter().map(predict_digit).collect::<Vec<_>>();
    let p1_answer: i32 = predictions.iter().map(|x| x.1).sum();
    println!("The sum of predictions for part 1 is {p1_answer}");
    let p2_answer: i32 = predictions.iter().map(|x| x.0).sum();
    println!("The sum of predictions for part 2 is {p2_answer}");
}

fn predict_digit(line: &Vec<i32>) -> (i32, i32) {
    let mut diff_sequences = vec![line.clone()];
    while !diff_sequences.last().unwrap().iter().all(|&x| x == 0) {
        let last = diff_sequences.last().unwrap().iter().cloned();
        diff_sequences.push(last.clone().zip(last.skip(1)).map(|(a, b)| b - a).collect());
    }
    // 0   3   6   9  12  15   _18_
    //   3   3   3   3   3   _3_
    //     0   0   0   0   0
    let next_pred = diff_sequences
        .iter()
        .map(|seq| seq.last().unwrap_or(&0))
        .sum();
    // _5_  10  13  16  21  30  45
    //   _5_   3   3   5   9  15
    //    _-2_   0   2   4   6
    //       _2_   2   2   2
    //          0   0   0
    let prev_pred = diff_sequences
        .iter()
        .map(|seq| seq.first().unwrap_or(&0))
        .rfold(0i32, |acc, x| x - acc);
    (prev_pred, next_pred)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predict_next_digits() {
        assert_eq!(predict_digit(&vec![0, 3, 6, 9, 12, 15]).1, 18);
        assert_eq!(predict_digit(&vec![1, 3, 6, 10, 15, 21]).1, 28);
    }

    #[test]
    fn predict_previous_digits() {
        assert_eq!(predict_digit(&vec![0, 3, 6, 9, 12, 15]).0, -3);
        assert_eq!(predict_digit(&vec![1, 3, 6, 10, 15, 21]).0, 0);
    }
}
