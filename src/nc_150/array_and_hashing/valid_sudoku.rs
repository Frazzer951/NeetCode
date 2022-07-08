/// Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:
///
/// Each row must contain the digits 1-9 without repetition.
/// Each column must contain the digits 1-9 without repetition.
/// Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
/// Note:
///
/// A Sudoku board (partially filled) could be valid but is not necessarily solvable.
/// Only the filled cells need to be validated according to the mentioned rules.

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    // check rows
    for row in &board {
        let mut seen = std::collections::HashSet::new();
        for cur in row.iter().take(9) {
            if *cur != '.' {
                if seen.contains(&cur) {
                    return false;
                }
                seen.insert(cur);
            }
        }
    }

    // check cols
    for col in 0..9 {
        let mut seen = std::collections::HashSet::new();
        for row in board.iter().take(9) {
            let cur = row[col];
            if cur != '.' {
                if seen.contains(&cur) {
                    return false;
                }
                seen.insert(cur);
            }
        }
    }

    // check 3x3s
    for i in 0..3 {
        for j in 0..3 {
            let si = i * 3;
            let sj = j * 3;
            let mut seen = std::collections::HashSet::new();
            for row in board.iter().skip(si).take(3) {
                for cur in row.iter().skip(sj).take(3) {
                    if *cur != '.' {
                        if seen.contains(&cur) {
                            return false;
                        }
                        seen.insert(cur);
                    }
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = is_valid_sudoku(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let result = is_valid_sudoku(vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]);
        assert!(!result);
    }
}
