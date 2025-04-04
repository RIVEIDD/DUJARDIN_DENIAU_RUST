use std::{io, thread, time::Duration};
use crate::input::wait_for_key;
use crate::objective::generate_objectives;
use crate::player::Player;
use crate::score::calculate_score;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

/// Lance la partie entre deux joueurs jusqu’à ce que l’un perde toute sa vitalité.
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

/// Effectue un tour de jeu : génère les objectifs, mesure le temps de réaction et calcule le score.
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

    let compteur = Arc::new(Mutex::new(0));
    let miss = Arc::new(Mutex::new(0));
    let running = Arc::new(AtomicBool::new(true));

    let c_clone = compteur.clone();
    let m_clone = miss.clone();
    let r_clone = running.clone();
    let speed = player.speed;

    let handle = thread::spawn(move || {
        while r_clone.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(speed.into()));
            let mut c = c_clone.lock().unwrap();
            *c += 1;
            if *c > 100 {
                *c = 0;
                let mut m = m_clone.lock().unwrap();
                *m += 1;
            }
        }
    });

    let key_pressed = loop {
        if let Some(c) = wait_for_key() {
            break c;
        }
    };

    running.store(false, Ordering::SeqCst);
    handle.join().unwrap();

    let score = *compteur.lock().unwrap();
    let miss = *miss.lock().unwrap();

    let (cible, valeur) = objectives.iter().next().unwrap();
    if key_pressed == *cible {
        let diff = crate::utils::circular_diff(score, *valeur);
        let point = calculate_score(diff, player.strength, miss);
        println!("→ Objectif {}:{} : Miss = {} | Compteur = {} => Score = {}", cible, valeur, miss, score, point);
        return point;
    } else {
        println!("→ Mauvaise touche {} au lieu de {} : Score = 0", key_pressed, cible);
    }

    let final_score = (0 as f32 / n_objectives as f32).ceil() as u32;
    println!("→ Score moyen {}", final_score);
    final_score
}
