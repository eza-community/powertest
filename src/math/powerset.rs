pub fn generate_powerset<T: Clone>(set: &[T], depth: usize) -> Vec<Vec<T>> {
    let mut powerset = vec![vec![]];

    for item in set.iter() {
        let current_len = powerset.len();
        for i in 0..current_len {
            let mut new_subset = powerset[i].clone();
            new_subset.push(item.clone());
            if new_subset.len() <= depth {
                powerset.push(new_subset);
            }
        }
    }

    powerset
}
pub fn generate_powerset_combined(
    set: &[(Option<String>, Option<String>)],
    depth: usize,
) -> Vec<Vec<(Option<String>, Option<String>)>> {
    let mut powerset = vec![vec![]];

    for item in set.iter() {
        let current_len = powerset.len();
        for i in 0..current_len {
            let mut new_subset = powerset[i].clone();
            new_subset.push(item.clone());
            if new_subset.len() <= depth {
                powerset.push(new_subset);
            }
        }
    }

    powerset
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_powerset() {
        let set: Vec<i32> = vec![1, 2, 3];
        let depth = 2;
        let mut result = generate_powerset(&set, depth);
        let mut expected = vec![
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
        ];
        // NOTE: Order isn't guaranteed, so we do this.
        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_powerset_empty() {
        let set: Vec<i32> = vec![];
        let depth = 2;
        let result = generate_powerset(&set, depth);
        let expected: Vec<Vec<i32>> = vec![vec![]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_powerset_combined() {
        let set = vec![
            (Some("a".to_string()), Some("b".to_string())),
            (Some("c".to_string()), None),
            (None, Some("d".to_string())),
            (None, None),
        ];
        let depth = 2;
        let mut result = generate_powerset_combined(&set, depth);
        let mut expected = vec![
            vec![],
            vec![(Some("a".to_string()), Some("b".to_string()))],
            vec![(Some("c".to_string()), None)],
            vec![(None, Some("d".to_string()))],
            vec![(None, None)],
            vec![
                (Some("a".to_string()), Some("b".to_string())),
                (Some("c".to_string()), None),
            ],
            vec![
                (Some("a".to_string()), Some("b".to_string())),
                (None, Some("d".to_string())),
            ],
            vec![(Some("a".to_string()), Some("b".to_string())), (None, None)],
            vec![(Some("c".to_string()), None), (None, Some("d".to_string()))],
            vec![(Some("c".to_string()), None), (None, None)],
            vec![(None, Some("d".to_string())), (None, None)],
        ];

        // NOTE: Order isn't guaranteed, so we do this.
        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_powerset_combined_empty() {
        let set: Vec<(Option<String>, Option<String>)> = vec![];
        let depth = 2;
        let result = generate_powerset_combined(&set, depth);
        let expected: Vec<Vec<(Option<String>, Option<String>)>> = vec![vec![]];
        assert_eq!(result, expected);
    }
}
