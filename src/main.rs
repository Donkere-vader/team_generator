use std::io::Write;
use rand::seq::SliceRandom;
use rand::thread_rng;

const AMOUNT_OF_PLAYERS: usize = 100;

fn clear_console() {
    print!("{}[2J", 27 as char);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn all_duos(team: &Vec<usize>) -> Vec<[usize; 2]> {
    let mut all: Vec<[usize; 2]> = Vec::new();

    for p in team.iter() {
        for p2 in team.iter() {
            if p < p2 {
                all.push([*p, *p2]);
            }
        }
    }

    all
}

fn validate_team(matrix: &[[usize; AMOUNT_OF_PLAYERS]; AMOUNT_OF_PLAYERS], team: &Vec<usize>) -> bool {
    for duo in all_duos(&team) {
        if matrix[duo[0]][duo[1]] != 0 {
            return false;
        }
    }

    true
}

fn print_matrix(matrix: &[[usize; AMOUNT_OF_PLAYERS]; AMOUNT_OF_PLAYERS]) {
    for y in 0..AMOUNT_OF_PLAYERS {
        for x in 0..AMOUNT_OF_PLAYERS {
            let cell = matrix[y][x];
            let mut color = "";
            if cell > 0 {
                color = "\u{001b}[30;50;42m";
            }
            print!("{}  {}\u{001b}[0;0;0m", color, cell);
        }
        println!();
    }
}

fn export_txt(teams: &Vec<Vec<usize>>) {
    let mut file = std::fs::File::create("output.txt").expect("create failed");

    for t in teams {
        file.write_all(format!("{} {} {} {}\n", t[0], t[1], t[2], t[3]).as_bytes()).expect("write failed");
    }
}

fn main() {
    let mut highest_teams = 0;

    loop {
        let mut teams: Vec<Vec<usize>> = Vec::new();

        let mut matrix: [[usize; AMOUNT_OF_PLAYERS]; AMOUNT_OF_PLAYERS] = [[0; AMOUNT_OF_PLAYERS]; AMOUNT_OF_PLAYERS];

        for i in 0..AMOUNT_OF_PLAYERS {
            matrix[i][i] = 1;
        }

        let mut done_players: Vec::<usize> = Vec::new();

        let mut c = true; // continue
        while c {
            c = false;

            let mut sorted_ys: Vec<usize> = Vec::new();
            let mut empty_places: Vec<usize> = Vec::new();

            for player in 0..AMOUNT_OF_PLAYERS {
                let empty = matrix[player].iter().filter(|&&x| x == 0 as usize).count();
                let mut index = 0;
                if empty_places.iter().position(|&r| r == empty) != None {
                    index = empty_places.iter().position(|&r| r == empty).unwrap();
                }

                sorted_ys.insert(index, player);
                empty_places.insert(index, empty)
            }

            // clear_console();
            // print_matrix(&matrix);

            let mut rng = thread_rng();
            let mut shuffled_ys = [0; AMOUNT_OF_PLAYERS];
            for i in 0..AMOUNT_OF_PLAYERS {
                shuffled_ys[i] = i;
            }
            shuffled_ys.shuffle(&mut rng);

            for player in shuffled_ys.iter() {
                if done_players.contains(&player) {
                    continue
                }

                let mut changes = false;
                let mut duos_in_new_team: Vec<[usize; 2]> = Vec::new();
                if matrix[*player].iter().filter(|&&x| x == 0 as usize).count() >= 3 {
                    let mut all_zeros = Vec::<usize>::new();
                    let mut b: bool = false;
                    
                    let mut rng = thread_rng();
                    let mut shuffled_ys2 = [0; AMOUNT_OF_PLAYERS];
                    for i in 0..AMOUNT_OF_PLAYERS {
                        shuffled_ys2[i] = i;
                    }
                    shuffled_ys2.shuffle(&mut rng);


                    for x in shuffled_ys2.iter() {
                        let cell = matrix[*player][*x];
                        if cell == 0 {
                            all_zeros.push(*x);
                        }

                        let mut team: Vec<usize> = vec![*player];
                        for zero in all_zeros.iter().rev() {
                            let mut new_team: Vec<usize> = (*team).to_vec();
                            new_team.push(*zero);

                            if validate_team(&matrix, &new_team) {
                                if new_team.len() == 4 {
                                    duos_in_new_team = all_duos(&new_team);
                                    teams.push((*new_team).to_vec());
                                    changes = true;
                                    b = true;
                                    break;
                                }
                                else {
                                    team = (*new_team).to_vec();
                                }
                            }
                        }

                        if b {
                            break;
                        }
                    }
                }

                for duo in duos_in_new_team.iter() {
                    matrix[duo[0]][duo[1]] = 1;
                    matrix[duo[1]][duo[0]] = 1;
                }

                if !changes {
                    done_players.push(*player);
                } else {
                    c = true;
                }
            }
        }

        // for t in teams.iter() {
        //     println!("{} {} {} {}", t[0], t[1], t[2], t[3]);
        // }
        
        // If new highscore is reached output it to output.txt
        let mut color = ""; // no higher score color is default
        if teams.len() > highest_teams {
            highest_teams = teams.len();
            color = "\u{001b}[5;30;50;42m"; // if higher score color is green and flashing
            export_txt(&teams);
        } else if highest_teams - teams.len() < 4 {
            color = "\u{001b}[5;30;50;43m"; // if difference is less than two than make it orange
        }
        println!("Found {} {} \u{001b}[0;0;0m combinations", color, teams.len());
    }
}
