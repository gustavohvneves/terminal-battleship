pub mod computer;
pub mod multiplayer;
pub mod singleplayer;
pub mod player;

use crate::{GameBoard, Position, Ship, ShipType, Tile};

pub struct AttackFeedback {
    tile_at_attack: Tile,
    valid_attack: bool,
    sunk_a_ship: bool,
    won_the_game: bool,
}

impl AttackFeedback {
    pub fn new(
        tile_at_attack: Tile,
        valid_attack: bool,
        sunk_a_ship: bool,
        won_the_game: bool,
    ) -> Self {
        Self {
            tile_at_attack,
            valid_attack,
            sunk_a_ship,
            won_the_game,
        }
    }
}

// the render bool is used to allow seeing placing ship/selector when it is over a non empty tile
pub fn place_ship_on_board(
    mut board: [[Tile; 10]; 10],
    ship: &Ship,
    row: usize,
    col: usize,
    render: bool,
) -> (bool, [[Tile; 10]; 10]) {
    match ship.ship_type {
        ShipType::CarrierHorizontal
        | ShipType::BattleshipHorizontal
        | ShipType::CruiserHorizontal
        | ShipType::SubmarineHorizontal
        | ShipType::DestroyerHorizontal => {
            if col + ship.length as usize <= 10 {
                let mut valid = true;
                for i in col..col + ship.length as usize {
                    if board[row][i] != Tile::Unknown && !render {
                        valid = false;
                    }
                }

                if valid {
                    for i in col..col + ship.length as usize {
                        board[row][i] = Tile::Ship(ship.ship_type);
                    }
                    (true, board)
                } else {
                    (false, board)
                }
            } else {
                (false, board)
            }
        }
        _ => {
            if row + ship.length as usize <= 10 {
                let mut valid = true;
                for i in row..row + ship.length as usize {
                    if board[i][col] != Tile::Unknown && !render {
                        valid = false;
                    }
                }

                if valid {
                    for i in row..row + ship.length as usize {
                        board[i][col] = Tile::Ship(ship.ship_type);
                    }
                    (true, board)
                } else {
                    (false, board)
                }
            } else {
                (false, board)
            }
        }
    }
}

pub fn process_attack(defender_board: GameBoard, attack_position: Position) -> AttackFeedback {
    let tile_at_attack_position = defender_board.get_tile_at_position(attack_position);

    let valid_attack = match tile_at_attack_position {
        Tile::Unknown | Tile::Ship(_) => true,
        _ => false,
    };

    if !valid_attack {
        return AttackFeedback::new(tile_at_attack_position, false, false, false);
    }

    match tile_at_attack_position {
        Tile::Unknown => AttackFeedback::new(tile_at_attack_position, true, false, false),
        Tile::Ship(_) => {
            let attack_sunk_a_ship = defender_board.check_if_hit_is_a_sink(tile_at_attack_position);
            let attack_won_the_game = defender_board.check_if_hit_won_the_game();

            AttackFeedback::new(
                tile_at_attack_position,
                true,
                attack_sunk_a_ship,
                attack_won_the_game,
            )
        }
        _ => AttackFeedback::new(tile_at_attack_position, false, false, false),
    }
}
