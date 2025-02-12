use crate::board::*;

/// A heuristic function to estimate the cost of reaching the goal state from a given board.
///
/// ```rust
/// let board = Board::new([[8, 7, 3], [2, 0, 5], [1, 4, 6]]);
/// let h = Heuristic::Manhattan.estimate(&board);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Heuristic {
    /// The blind heuristic always returns 0.
    Blind,
    /// The Hamming heuristic, which counts the number of misplaced tiles.
    Hamming,
    /// The Manhattan heuristic, which computes the sum of the Manhattan distances of each tile to its goal position.
    Manhattan,
}

impl Heuristic {
    pub fn estimate(&self, board: &Board) -> u32 {
        match self {
            // blind heuristic always returns 0
            Heuristic::Blind => 0,
            Heuristic::Hamming => {
                let mut misplaced = 0;
                for row in 0..3 {
                    for col in 0..3 {
                        let expected = row * 3 + col + 1;
                        let val = board.value_at(row, col);
                        if val != 0 && val != expected as u8 {
                            misplaced += 1;
                        }
                    }
                }
                misplaced
            }
            Heuristic::Manhattan => {
                let mut total = 0;
                for n in 1..9u32 {
                    let pos = board.position(n as u8);
                    let expected = ((n - 1) / 3, (n - 1) % 3);
                    total += expected.0.abs_diff(pos.0 as u32) + expected.1.abs_diff(pos.1 as u32);
                }
                total
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_heuristic() {
        use super::*;
        let board = Board::new([[8, 7, 3], [2, 0, 5], [1, 4, 6]]);
        assert_eq!(Heuristic::Blind.estimate(&board), 0);
        assert_eq!(Heuristic::Hamming.estimate(&board), 7);
        assert_eq!(Heuristic::Manhattan.estimate(&board), 14);

        assert_eq!(Heuristic::Blind.estimate(&Board::GOAL), 0);
        assert_eq!(Heuristic::Hamming.estimate(&Board::GOAL), 0);
        assert_eq!(Heuristic::Manhattan.estimate(&Board::GOAL), 0);
    }
}
