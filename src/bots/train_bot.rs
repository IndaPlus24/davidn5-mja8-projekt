use rand::Rng;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

use crate::Game;

use super::bot::Bot;


const POP_SIZE: usize = 1000;
const ITERATIONS: usize = 20;

pub fn train_ai() {

    let pool = ThreadPoolBuilder::new()
        .num_threads(15)
        .build_global()
        .expect("Failed to build thread pool");

    let mut population: Vec<Bot> = (0..POP_SIZE)
        .map(|_| Bot {
            game: Game::new(),
            inputs: Vec::new(),
            fitness: 0.0,
            weights: Bot::random_weights(),
            game_steps: 0,
        })
        .collect();

    for i in 0..ITERATIONS {
        println!("Starting training cycle {} ...", i);
        population = train_population(&mut population, i);
    }

    population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

    let best_agent = population[0].clone();

    // TODO SAVE TO FILE

    println!(
        "Best Agent... \n 
    {:?}", best_agent.weights
    );
    std::process::exit(0)
}

pub fn train_population(population: &mut Vec<Bot>, epoch : usize) -> Vec<Bot> {
    println!("Starting game on {} agents...", population.len());

    // Generate multiple random seeds for testing
    let mut rng = rand::rng();
    let seeds: Vec<u64> = (0..5).map(|_| rng.random()).collect(); // Test on 5 different maps

    population.par_iter_mut().enumerate().for_each(|(i, agent)|{
        agent.run_game_without_ui(100000);
    });

    println!("Evaluating Agents...");

    // Sort by fitness in descending order
    population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

    for (i, bot) in population.iter().take(10).enumerate() {
        println!("Top {} fitness: {}", i, bot.fitness);
    }
    
    // Take top 10% as elites
    let elite_count = (population.len() as f32 * 0.1).ceil() as usize;
    let elite_population: Vec<Bot> = population[..elite_count].to_vec();

    let mut new_population: Vec<Bot> = Vec::new();

    // Keep the elite
    for elite in &elite_population {
        new_population.push(elite.clone());
    }

    // Reproduce until population is full
    let mut rng = rand::rng();
    while new_population.len() < population.len() {
        let i = rng.random_range(0..elite_population.len());
        let mut j = rng.random_range(0..elite_population.len());
        while j == i {
            j = rng.random_range(0..elite_population.len());
        }

        let parent1 = &elite_population[i];
        let parent2 = &elite_population[j];

        let mut child = Bot::crossover(parent1, parent2);
        let mut mutation_rate = 0.05;
        if epoch > 750{mutation_rate = 0.01}
        else if epoch > 500{mutation_rate = 0.03}

        child.mutate(mutation_rate); // Apply mutation
        new_population.push(child);
    }

    new_population
}