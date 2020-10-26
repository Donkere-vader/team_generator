use std::io::Write;
use std::fs::File;
use std::io::prelude::*;
use rand::Rng;
use std::thread;

const AMOUNT_OF_PLAYERS: usize = 100;
const PROCESSES: usize = 16;

#[allow(dead_code)]

fn clear_console() {
    print!("{}[2J", 27 as char);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

#[allow(dead_code)]
fn print_matrix(matrix: &[[usize; AMOUNT_OF_PLAYERS]; AMOUNT_OF_PLAYERS]) {
    print!("   ");
    for x in 0..AMOUNT_OF_PLAYERS {
        print!("{}", format!("{: >3}", x + 1));
    }
    println!();

    for y in 0..AMOUNT_OF_PLAYERS {
        print!("{}",format!("{: >3}", y + 1));
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

fn generate_list(from: usize, to: usize) -> [usize; AMOUNT_OF_PLAYERS] {
    let mut new_list = [0; AMOUNT_OF_PLAYERS];
    for num in from..to {
        new_list[num] = num;
    }

    new_list
}

fn num_in_range(num: i128, min: i128, max: i128) -> i128 {
    if num < min {
        return min;
    } else if num > max - 1 {
        return max - 1;
    }

    num
}

fn shuffle_list(list: &mut [usize; AMOUNT_OF_PLAYERS], range: &usize, shuffles: &usize) {
    // don't hate, I'm not very experienced with rust

    let mut rng = rand::thread_rng();
    for _ in 0..*shuffles {
        let num = rng.gen_range(0, AMOUNT_OF_PLAYERS);
        let mut shuffle_with = rng.gen_range(*range as i128 * -1, *range as i128);
        shuffle_with += num as i128;
        let shuffle_with_idx = num_in_range(shuffle_with, 0, AMOUNT_OF_PLAYERS as i128) as usize;
        
        let temp = list[num];
        list[num] = list[shuffle_with_idx];
        list[shuffle_with_idx] = temp;
    }
}

fn start_loop() -> Vec<Vec<usize>> {
    let mut rng = rand::thread_rng();
    
    let mut most_teams = Vec::new();

    for _ in 0..1000 {
        // setup
        let mut teams: Vec<Vec<usize>> = Vec::new();
        let mut matrix: [[usize; AMOUNT_OF_PLAYERS]; AMOUNT_OF_PLAYERS] = [[0; AMOUNT_OF_PLAYERS]; AMOUNT_OF_PLAYERS];
        let mut shuffled_ys = generate_list(0, AMOUNT_OF_PLAYERS);
        let mut shuffled_ys2 = generate_list(0, AMOUNT_OF_PLAYERS);
        let mut shuffle_range = rng.gen_range(5, 30);
        let mut shuffle_amount = rng.gen_range(5, 40);
        // let mut shuffle_range: usize = 15;
        // let mut shuffle_amount: usize = 25;
        let mut loop_iteration = 0;

        // fill in the diagonal ((0, 0), (1, 1), (2, 2) ..etc)
        for i in 0..AMOUNT_OF_PLAYERS {
            matrix[i][i] = 1;
        }

        let mut done_players: Vec::<usize> = Vec::new();
        let mut not_c = 0;
        let mut c = true; // continue
        while c || not_c < 100 {
            if !c {
                not_c += 1;
            }
            c = false;

            if loop_iteration % 20  == 0{
            //     shuffle_range = rng.gen_range(1, 30);
            //     shuffle_amount = rng.gen_range(0, 30);
                shuffle_range = num_in_range((shuffle_range - 1) as i128, 5, AMOUNT_OF_PLAYERS as i128) as usize;
                shuffle_amount = num_in_range((shuffle_amount - 1) as i128, 5, AMOUNT_OF_PLAYERS as i128) as usize;
                loop_iteration = 0;
            }
            loop_iteration += 1;

            shuffle_list(&mut shuffled_ys, &shuffle_range, &shuffle_amount);

            for player in shuffled_ys.iter() {
                let mut changes = false;
                let mut duos_in_new_team: Vec<[usize; 2]> = Vec::new();
                if matrix[*player].iter().filter(|&&x| x == 0 as usize).count() >= 3 {
                    let mut all_zeros = Vec::<usize>::new();
                    
                    shuffle_list(&mut shuffled_ys2, &shuffle_range, &shuffle_amount);


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
        
        if teams.len() > most_teams.len() {
            most_teams = teams;
        }
    }

    most_teams
}

fn main() {
    loop {
        let mut threads: Vec<std::thread::JoinHandle<()>> = Vec::new();

        for _ in 0..PROCESSES {
            threads.push(
                thread::spawn(move || {
                    let teams = start_loop();
    
                    // If new highscore is reached output it to output.txt
                    let mut color = ""; // no higher score color is default
                    let highest_teams = get_highest_teams();
                    if teams.len() > highest_teams {
                        color = "\u{001b}[5;30;50;42m"; // if higher score color is green and flashing
                        export_txt(&teams);
                    } else if highest_teams - teams.len() < 10 {
                        color = "\u{001b}[30;50;43m"; // if difference is 0 then the color is orange
                    }
                    println!("Found {} {} \u{001b}[0;0;0m combinations", color, teams.len());
                })
            );
        }

        for thread in threads {
            thread.join().expect("Error joining thread");
        }
    }
}
