use std::fs::File;
use std::io::prelude::*;

use itertools::Itertools;

fn main() -> std::io::Result<()> {
    let mut file = File::create("chess960_positions.txt")?;

    let black_pieces = vec!['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'];
    let inner_fen = "/pppppppp/8/8/8/8/PPPPPPPP/";

    for perm in black_pieces
        .iter()
        .permutations(black_pieces.len())
        .unique()
        .filter(|p| {
            // Filter invalid chess960 positions, i.e king not inbetween rooks and bishops on same color
            // Use of unwraps here as we know that these chars will be in the vec
            let king_position = p.iter().position(|&&c| c == 'k').unwrap();

            p.iter().position(|&&c| c == 'r').unwrap() < king_position
                && p.iter().rposition(|&&c| c == 'r').unwrap() > king_position
                && p.iter().position(|&&c| c == 'b').unwrap() % 2
                    != p.iter().rposition(|&&c| c == 'b').unwrap() % 2
        })
    {
        let black_fen = String::from_iter(perm);
        let white_fen = black_fen.as_str().to_uppercase();

        let fen = black_fen + inner_fen + &white_fen;

        file.write_all(format!("{}\n", fen).as_bytes())?;
    }

    Ok(())
}
