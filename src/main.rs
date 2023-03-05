use crate::{
    board::Board,
    piece::{Color, Piece, Point},
    player::Player,
};

pub mod board;
pub mod piece;
pub mod player;

fn main() {
    println!("Hello, world!");

    let mut black_pieces = Piece::initialize_black_pieces();
    let mut white_pieces = Piece::initialize_white_pieces();

    let mut board = Board::new(&black_pieces, &white_pieces);

    let mut player_one = Player::new("Alice", Color::BLACK);
    let mut player_two = Player::new("Bob", Color::WHITE);

    board.display(&black_pieces, &white_pieces);

    //player_one.move_piece(&mut black_pieces[8], Position { row: 3, col: 5 });
    player_one.move_piece(&mut black_pieces[0], Point { x: 5, y: 0 });
    player_two.move_piece(&mut white_pieces[1], Point { x: 4, y: 1 });

    //println!("{:?}", black_pieces[0].position);
    board.update(&black_pieces, &white_pieces);

    // let rook_valid_moves = black_pieces[8].get_valid_moves(&board);
    // println!("{:?}", rook_valid_moves);
    let pawn_2_valid_moves = white_pieces[1].get_valid_moves(&board);
    let pawn_1_valid_moves = white_pieces[0].get_valid_moves(&board);
    println!("{:?}", pawn_2_valid_moves);
    println!("{:?}", pawn_1_valid_moves);

    board.update(&black_pieces, &white_pieces);
    board.display(&black_pieces, &white_pieces);

    // let game = Game::new(player_one, player_two);

    // game.play();
}
