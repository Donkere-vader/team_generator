use std::io::Write;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::prelude::*;

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

fn get_highest_teams() -> usize {
    // the most ugly rust function you will ever see. This is to get the file length of output.txt

    let mut file = match File::open("output.txt") {
        Err(_why) => panic!("couldn't open output.txt"),
        Ok(file) => file,
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    let mut vec: Vec<&str> = contents.split("\n").collect();
    let index = vec.iter().position(|x| *x == "").unwrap();
    vec.remove(index);
    vec.len()
}

fn main() {
    loop {
        let mut teams: Vec<Vec<usize>> = Vec::new();

        let mut matrix: [[usize; AMOUNT_OF_PLAYERS]; AMOUNT_OF_PLAYERS] = [[0; AMOUNT_OF_PLAYERS]; AMOUNT_OF_PLAYERS];

        for i in 0..AMOUNT_OF_PLAYERS {
            matrix[i][i] = 1;
        }

        let mut done_players: Vec::<usize> = Vec::new();
        let mut not_c = 0;
        let mut c = true; // continue
        while c && not_c < 100 {
            if !c {
                not_c += 1;
            }
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
                // if done_players.contains(&player) {
                //     continue
                // }

                let mut changes = false;
                let mut duos_in_new_team: Vec<[usize; 2]> = Vec::new();
                if matrix[*player].iter().filter(|&&x| x == 0 as usize).count() >= 3 {
                    let mut all_zeros = Vec::<usize>::new();
                    
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
                    }

                    let mut team: Vec<usize> = vec![*player];
                    for zero in all_zeros.iter() {
                        let mut new_team: Vec<usize> = (*team).to_vec();
                        new_team.push(*zero);

                        if new_team.len() == 2 || validate_team(&matrix, &new_team) {
                            if new_team.len() == 4 {
                                duos_in_new_team = all_duos(&new_team);
                                teams.push((*new_team).to_vec());
                                changes = true;
                                break;
                            }
                            else {
                                team = (*new_team).to_vec();
                            }
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
        let highest_teams = get_highest_teams();
        if teams.len() > highest_teams {
            color = "\u{001b}[5;30;50;42m"; // if higher score color is green and flashing
            export_txt(&teams);
        } else if highest_teams - teams.len() < 4 {
            color = "\u{001b}[5;30;50;43m"; // if difference is less than two than make it orange
        }
        // clear_console();
        // print_matrix(&matrix);
        println!("Found {} {} \u{001b}[0;0;0m combinations", color, teams.len());
    }
}
