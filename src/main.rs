use rand::Rng;

const NUMBER_OF_BACK_RANK_PIECES: usize = 8;

fn main() {
    let mut black_back_rank: [char; NUMBER_OF_BACK_RANK_PIECES] = [' '; NUMBER_OF_BACK_RANK_PIECES];
    let inner_fen = "/pppppppp/8/8/8/8/PPPPPPPP/";

    let black_pieces: [char; NUMBER_OF_BACK_RANK_PIECES] = ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'];

    let mut rng = rand::rng();

    for i in 0..NUMBER_OF_BACK_RANK_PIECES {
        let mut available_pieces = black_pieces.to_vec();

        available_pieces.retain(|p| {
            black_back_rank
                .iter()
                .filter(|&&rank_char| rank_char == *p)
                .count()
                < 2
        });

        // King should be between 2 rooks, there should only be one king
        if i == 0 || black_back_rank.contains(&'k') {
            available_pieces.retain(|&p| p != 'k');
        }

        // Only one queen
        if black_back_rank.contains(&'q') {
            available_pieces.retain(|&p| p != 'q');
        }

        // King should be between 2 rooks
        if black_back_rank.contains(&'r') && !black_back_rank.contains(&'k') {
            available_pieces.retain(|&p| p != 'r');
        }

        // Bishops should be on opposite colors
        if let Some(bishop_index) = black_back_rank.iter().position(|&p| p == 'b') {
            if i % 2 == bishop_index % 2 {
                available_pieces.retain(|&p| p != 'b');
            }
        }

        let random_piece_index = rng.random_range(..available_pieces.len());

        let random_piece = available_pieces[random_piece_index];

        black_back_rank[i] = random_piece;
    }

    let black_fen = String::from_iter(black_back_rank);

    let white_fen = black_fen.as_str().to_uppercase();

    let fen = black_fen + inner_fen + &white_fen;

    println!("{}", fen);
}
