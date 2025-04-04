use std::{collections::HashMap, io, thread, time::Duration};
use crate::input::wait_for_key;
use crate::objective::generate_objectives;
use crate::player::Player;
use crate::score::calculate_score;

/// Lance une partie complète entre deux joueurs
pub fn start_game(player1: &mut Player, player2: &mut Player, n_objectives: usize) {
    let mut round = 1;

    loop {
        println!("## Manche {} ##", round);

        let score1 = play_turn(player1, n_objectives);
        let score2 = play_turn(player2, n_objectives);

        if score1 > score2 {
            let diff = score1 - score2;
            player2.vitality -= diff as i32;
            println!("{} gagne la manche. {} perd {} points de vitalité.", player1.name, player2.name, diff);
            poison_choice(player1, player2);
        } else if score2 > score1 {
            let diff = score2 - score1;
            player1.vitality -= diff as i32;
            println!("{} gagne la manche. {} perd {} points de vitalité.", player2.name, player1.name, diff);
            poison_choice(player2, player1);
        } else {
            println!("Égalité parfaite ! Aucun changement.");
        }

        if !player1.is_alive() || !player2.is_alive() {
            println!("##### Partie terminée #####");
            break;
        }

        round += 1;
    }
}

fn poison_choice(winner: &Player, loser: &mut Player) {
    println!("{} vous devez choisir quel poison appliquer à {} :", winner.name, loser.name);
    println!("→ 1: -5 speed\n→ 2: -5 strength");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim().parse::<u8>().unwrap_or(0);
    loser.apply_poison(choice);
}

fn play_turn(player: &Player, n_objectives: usize) -> u32 {
    player.display_stats();
    let objectives = generate_objectives(n_objectives);
    println!("→ Objectifs : {:?}", objectives);
    println!("→ Appuyer sur ENTREE pour démarrer le tour...");

    loop {
        if wait_for_key().is_none() {
            break;
        }
    }

    let mut total_score = 0;

    for (letter, target) in objectives {
        let mut compteur = 0;
        let mut miss = 0;

        // Compteur simulé
        let speed = player.speed;
        let handle = thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(speed));
                compteur = (compteur + 1) % 101;
                if compteur == 0 {
                    miss += 1;
                }
            }
            #[allow(unreachable_code)]
            (compteur, miss)
        });

        let key_pressed = loop {
            if let Some(c) = wait_for_key() {
                break c;
            }
        };

        let _ = handle.thread().unpark(); // Simule l'arrêt du thread
        let (score, miss) = (compteur, miss); // À ajuster si le thread est contrôlé proprement

        if key_pressed == letter {
            let diff = crate::utils::circular_diff(score, target);
            let point = calculate_score(diff, player.strength, miss);
            println!("→ Objectif {}:{} : Miss = {} | Compteur = {} => Score = {}", letter, target, miss, score, point);
            total_score += point;
        } else {
            println!("→ Mauvaise touche {} au lieu de {} : Score = 0", key_pressed, letter);
        }
    }

    let final_score = (total_score as f32 / n_objectives as f32).ceil() as u32;
    println!("→ Score moyen {}", final_score);
    final_score
}
