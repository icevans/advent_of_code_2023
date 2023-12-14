pub fn extrapolate(sequences: Vec<Vec<i32>>) -> i32 {
    let mut previous_added_value = 0;

    for sequence in sequences {
        previous_added_value = sequence.last().unwrap() + previous_added_value;
    }

    return previous_added_value;
}

pub fn extrapolate_backwards(sequences: Vec<Vec<i32>>) -> i32 {
    let mut previous_added_value = 0;

    for sequence in sequences {
        previous_added_value = sequence.first().unwrap() - previous_added_value;
    }

    return previous_added_value;
}

pub fn difference_sequences(sequence: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];

    for i in 1..sequence.len() {
        result.push(sequence[i] - sequence[i - 1]);
    }

    return if result.iter().all(|x| *x == 0) {
        vec![result, sequence]
    } else {
        let mut differences = difference_sequences(result);
        differences.push(sequence);
        differences
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difference_sequences() {
        let result = difference_sequences(vec![0, 3, 6, 9, 12, 15]);
        let expected = vec![
            vec![0, 0, 0, 0],
            vec![3, 3, 3, 3, 3],
            vec![0, 3, 6, 9, 12, 15],
        ];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_difference_sequences_2() {
        let result = difference_sequences(vec![1, 3, 6, 10, 15, 21]);
        let expected = vec![
            vec![0, 0, 0],
            vec![1, 1, 1, 1],
            vec![2, 3, 4, 5, 6],
            vec![1, 3, 6, 10, 15, 21],
        ];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_extrapolate_1() {
        let sequences = vec![
            vec![0, 0, 0],
            vec![1, 1, 1, 1],
            vec![2, 3, 4, 5, 6],
            vec![1, 3, 6, 10, 15, 21],
        ];

        assert_eq!(28, extrapolate(sequences));
    }

    #[test]
    fn test_extrapolate_2() {
        let sequences = vec![
            vec![0, 0, 0, 0],
            vec![3, 3, 3, 3, 3],
            vec![0, 3, 6, 9, 12, 15],
        ];

        assert_eq!(18, extrapolate(sequences));
    }
}
