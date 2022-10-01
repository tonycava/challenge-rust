pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    if diagonals("X", &table) || horizontal("X", &table) || vertical("X", &table) {
        return "player X won".to_string();
    }
    if diagonals("O", &table) || horizontal("O", &table) || vertical("O", &table) {
        return "player O won".to_string();
    }

    return "Tie".to_string();
}
pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    if table[0][0] == player && table[1][1] == player && table[2][2] == player {
        return true;
    }
    if table[2][0] == player && table[1][1] == player && table[0][2] == player {
        return true;
    }
    return false;
}
pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    if table[0][0] == player && table[0][1] == player && table[0][2] == player {
        return true;
    }
    if table[1][0] == player && table[1][1] == player && table[1][2] == player {
        return true;
    }
    if table[2][0] == player && table[2][1] == player && table[2][2] == player {
        return true;
    }
    return false;
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    if table[0][0] == player && table[1][0] == player && table[2][0] == player {
        return true;
    }

    if table[0][1] == player && table[1][1] == player && table[2][1] == player {
        return true;
    }

    if table[0][2] == player && table[1][2] == player && table[2][2] == player {
        return true;
    }
    return false;
}