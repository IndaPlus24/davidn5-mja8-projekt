use rand::seq::IndexedRandom;
use rand::Rng;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

use super::bot::Bot;

const POP_SIZE: usize = 100;
const ITERATIONS: usize = 10;
const TOURNAMENT_SIZE: usize = 100;
const OFFSPRING_COUNT: usize = (POP_SIZE as f64 * 0.3) as usize;
const GAMES_PER_EVALUATION: usize = 25;
const STEPS_PER_GAME: i32 = 800;

pub fn train_ai() {
    let _ = ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .expect("Failed to build thread pool");

    let mut population: Vec<Bot> = (0..POP_SIZE)
        .map(|_| Bot::with_random_unit_weights())
        .collect();

    for generation in 0..ITERATIONS {
        println!("Generation {} ...", generation);

        population.par_iter_mut().for_each(|bot| {
            bot.fitness = (0..GAMES_PER_EVALUATION)
                .map(|_| bot.run_game_without_ui(STEPS_PER_GAME))
                .sum();
        });

        population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        println!("Top fitness: {}", population[0].fitness);
        println!("Best weights: {:?}", population[0].weights);
        println!("Amount of steps: {}", population[0].game_steps);

        let mut rng = rand::rng();
        let mut offspring: Vec<Bot> = vec![];

        while offspring.len() < OFFSPRING_COUNT {
            let tournament: Vec<&Bot> = population
                .choose_multiple(&mut rng, TOURNAMENT_SIZE)
                .collect();

            let mut selected = tournament.clone();
            selected.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

            let parent1 = selected[0];
            let parent2 = selected[1];

            let mut child = Bot::random_crossover(parent1, parent2);
            if rng.random_bool(0.05) {
                child.mutate(0.1);
            }

            offspring.push(child);
        }

        population.truncate(POP_SIZE - OFFSPRING_COUNT);
        population.extend(offspring);
    }

    population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    println!("Best weights: {:?}", population[0].weights);
}
