use bevy::prelude::{Query, Text};
use crate::entities::Score;

// function to update the score board
pub fn score_board(
    mut score_board: Query<(&Score, &mut Text)>
) {
    // update the score board
    for (score_board, mut score_board_text) in score_board.iter_mut() {
        score_board_text.sections[0].value = format!("{}", score_board.score_1p);
        score_board_text.sections[2].value = format!("{}", score_board.score_2p);
    }
}