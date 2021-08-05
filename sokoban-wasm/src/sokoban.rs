

// pub mod sokoban {
    use csv::Reader;
    use std::error::Error;

    #[derive(Debug, Deserialize)]
    struct GameLevel {
        level: i8,
        board: i8,
        state: String,
    }

    #[derive(PartialEq, Eq, Hash, Debug)]
    struct Coord {
        row: i8,
        col: i8,
    }

    #[derive(PartialEq, Eq, Hash, Debug)]
    struct Player {
        coord: Coord,
        on_spot: bool,
    }
    
    fn find_player(board: &str) -> Player {
        let board_arr: Vec<String> = board.split("|").map(|s| s.to_string()).collect();
        for (i, row) in board_arr.iter().enumerate() {
            if row.contains('P') {
                return Player { coord: Coord { row: i as i8, col: row.find('P').unwrap() as i8 }, on_spot: false };
            } else if row.contains('Y') {
                return Player { coord: Coord { row: i as i8, col: row.find('Y').unwrap() as i8 }, on_spot: true };
            }
        }
        Player { coord: Coord { col: -1, row: -1 }, on_spot: false }
    }

    fn check_bounds(board_arr: Vec<Vec<char>>, new_position: Coord) -> Coord {
        let new_row: i8 =
            if new_position.row < 0 {
                0
            } else if new_position.row >= board_arr.len() as i8 {
                (board_arr.len() - 1) as i8
            } else {
                new_position.row
            };
    
        let new_col: i8 =
            if new_position.col < 0 {
                0
            } else if new_position.col >= board_arr[new_row as usize].len() as i8 {
                (board_arr[new_position.row as usize].len() - 1) as i8
            } else {
                new_position.col
            };
    
        Coord { row: new_row, col: new_col }
    }

    pub fn read_level(level: i8, board: i8) -> Result<String, Box<dyn Error>> {
        let boards: &str = include_str!("levels.csv");
        let mut rdr: Reader<&[u8]> = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_reader(boards.as_bytes());
        for result in rdr.deserialize() {
            let record: GameLevel = result?;
            if record.board == board && record.level == level {
                return Ok(record.state);
            }
        }
        Ok("No Level".to_string())
    }

    pub fn win_state(board: &str) -> bool
    {
        if board.is_empty() || board.matches('B').count() > 0 {
            false
        } else {
            true
        }
    }

    pub fn move_player(board: &str, direction: &str) -> String {
        let player: Player = find_player(board);
        let mut board_arr: Vec<Vec<char>> = board.split("|").map(|s| s.to_string().chars().collect()).collect();
        let move_direction: Coord = match direction {
            "L" => Coord { row: 0, col: -1 },
            "U" => Coord { row: -1, col: 0 },
            "R" => Coord { row: 0, col: 1 },
            "D" => Coord { row: 1, col: 0 },
            _ => Coord { row: 0, col: 0 }
        };
    
        let move_to_type: char = board_arr[(player.coord.row + move_direction.row) as usize][(player.coord.col + move_direction.col) as usize];
        let new_empty_space: char = if player.on_spot { 'S' } else { '.' };
        let move_to_space: Coord = check_bounds(board_arr.clone(),
                                                Coord {
                                                    row: player.coord.row + (move_direction.row * 2),
                                                    col: player.coord.col + (move_direction.col * 2),
                                                });
    
        let push_to_space: char = board_arr[move_to_space.row as usize][move_to_space.col as usize];
    
        let move_spaces: i8 = match move_to_type {
            'W' => 0,
            '.' => 1,
            'S' => 1,
            'B' | 'Z' => {
                if push_to_space == '.' || push_to_space == 'S' { 2 } else { 0 }
            }
            _ => 0
        };
    
        if move_spaces > 0 {
            let new_box_char: char = if move_spaces == 2 && push_to_space == 'S' {
                'Z'
            } else { 'B' };
    
            let new_player_char: char = match move_to_type {
                'S' | 'Z' => 'Y',
                _ => 'P'
            };
            board_arr[player.coord.row as usize][player.coord.col as usize] = new_empty_space;
            if move_direction.row != 0 {
                board_arr[(player.coord.row + move_direction.row) as usize][player.coord.col as usize] = new_player_char;
            } else {
                board_arr[player.coord.row as usize][(player.coord.col + move_direction.col) as usize] = new_player_char;
            }
            if move_spaces == 2 {
                if move_direction.row != 0 {
                    board_arr[(player.coord.row + (move_direction.row * 2)) as usize][player.coord.col as usize] = new_box_char;
                } else {
                    board_arr[player.coord.row as usize][(player.coord.col + (move_direction.col * 2)) as usize] = new_box_char;
                }
            }
        }
        let z: Vec<String> = board_arr.into_iter().map(|x| x.into_iter().collect()).collect();
        z.join("|")
    }
// }

#[cfg(test)]
mod misc_tests {
    use super::*;

    #[test]
    fn find_player_on_empty() {
        let player: Player = Player { coord: Coord { col: 3, row: 2 }, on_spot: false };
        assert_eq!(find_player("WWWWWWW|WS.B.SW|W.BPB.W|WS.B.SW|WWWWWWW"), player);
    }

    #[test]
    fn find_player_on_spot() {
        let player: Player = Player { coord: Coord { col: 3, row: 2 }, on_spot: true };
        assert_eq!(find_player("WWWWWWW|WS.B.SW|W.BYB.W|WS.B.SW|WWWWWWW"), player);
    }

    #[test]
    fn find_not_won_state() {
        assert_eq!(win_state("WWWWWWW|WS.B.SW|W.BYB.W|WS.B.SW|WWWWWWW"), false);
    }

    #[test]
    fn find_winning_state() {
        assert_eq!(win_state("WWWWWWW|WS...SW|W..Y..W|WS...SW|WWWWWWW"), true);
    }
}

#[cfg(test)]
mod move_tests {
    use super::*;

    #[test]
    fn move_player_up_empty() {
        assert_eq!(
            move_player("WWWWWWW|WS...SW|W..P..W|WS...SW|WWWWWWW", "U"),
            "WWWWWWW|WS.P.SW|W.....W|WS...SW|WWWWWWW");
    }

    #[test]
    fn move_player_down_empty() {
        assert_eq!(
            move_player("WWWWWWW|WS...SW|W..P..W|WS...SW|WWWWWWW", "D"),
            "WWWWWWW|WS...SW|W.....W|WS.P.SW|WWWWWWW");
    }

    #[test]
    fn move_player_left_empty() {
        assert_eq!(
            move_player("WWWWWWW|WS...SW|W..P..W|WS...SW|WWWWWWW", "L"),
            "WWWWWWW|WS...SW|W.P...W|WS...SW|WWWWWWW");
    }

    #[test]
    fn move_player_right_empty() {
        assert_eq!(
            move_player("WWWWWWW|WS...SW|W..P..W|WS...SW|WWWWWWW", "R"),
            "WWWWWWW|WS...SW|W...P.W|WS...SW|WWWWWWW");
    }

    #[test]
    fn move_player_and_block_up() {
        assert_eq!(
            move_player("WWWWWWW|W.....W|WS.B.SW|W.BPB.W|WS.B.SW|W.....W|WWWWWWW", "U"),
            "WWWWWWW|W..B..W|WS.P.SW|W.B.B.W|WS.B.SW|W.....W|WWWWWWW");
    }

    #[test]
    fn move_player_and_block_down() {
        assert_eq!(
            move_player("WWWWWWW|W.....W|WS.B.SW|W.BPB.W|WS.B.SW|W.....W|WWWWWWW", "D"),
            "WWWWWWW|W.....W|WS.B.SW|W.B.B.W|WS.P.SW|W..B..W|WWWWWWW");
    }

    #[test]
    fn move_player_and_block_right() {
        assert_eq!(
            move_player("WWWWWWW|W.....W|WS.B.SW|W.BPB.W|WS.B.SW|W.....W|WWWWWWW", "R"),
            "WWWWWWW|W.....W|WS.B.SW|W.B.PBW|WS.B.SW|W.....W|WWWWWWW");
    }

    #[test]
    fn move_player_and_block_left() {
        assert_eq!(
            move_player("WWWWWWW|W.....W|WS.B.SW|W.BPB.W|WS.B.SW|W.....W|WWWWWWW", "L"),
            "WWWWWWW|W.....W|WS.B.SW|WBP.B.W|WS.B.SW|W.....W|WWWWWWW");
    }

    #[test]
    fn move_player_to_spot() {
        assert_eq!(
            move_player("WWWWWWW|W.....W|WSPB.SW|W.B.B.W|WS.B.SW|W.....W|WWWWWWW", "L"),
            "WWWWWWW|W.....W|WY.B.SW|W.B.B.W|WS.B.SW|W.....W|WWWWWWW");
    }

    #[test]
    fn move_block_to_spot() {
        assert_eq!(
            move_player("WWWWWWW|W.....W|WSBP.SW|W.B.B.W|WS.B.SW|W.....W|WWWWWWW", "L"),
            "WWWWWWW|W.....W|WZP..SW|W.B.B.W|WS.B.SW|W.....W|WWWWWWW");
    }
}

#[cfg(test)]
mod wall_move_tests {
    use super::*;

    #[test]
    fn move_player_against_wall_left() {
        assert_eq!(
            move_player("WWWWWWW|WP...SW|W.B.B.W|WS.B.SW|WWWWWWW", "L"),
            "WWWWWWW|WP...SW|W.B.B.W|WS.B.SW|WWWWWWW");
    }
    #[test]
    fn move_player_against_wall_right() {
        assert_eq!(
            move_player("WWWWWWW|W....PW|W.B.B.W|WS.B.SW|WWWWWWW", "R"),
            "WWWWWWW|W....PW|W.B.B.W|WS.B.SW|WWWWWWW");
    }
    #[test]
    fn move_player_against_wall_up() {
        assert_eq!(
            move_player("WWWWWWW|WP...SW|W.B.B.W|WS.B.SW|WWWWWWW", "U"),
            "WWWWWWW|WP...SW|W.B.B.W|WS.B.SW|WWWWWWW");
    }
    #[test]
    fn move_player_against_wall_down() {
        assert_eq!(
            move_player("WWWWWWW|W....SW|W.B.B.W|WS.B.PW|WWWWWWW", "D"),
            "WWWWWWW|W....SW|W.B.B.W|WS.B.PW|WWWWWWW");
    }
}