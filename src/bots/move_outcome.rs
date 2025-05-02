use crate::Game;
use super::bot_input::BotInput;


#[derive(Clone, Debug)]
pub struct MoveOutcome {
    pub move_sequence : Vec<BotInput>,
    pub lines_cleared : f32, 
    pub aggregate_height : f32, 
    pub holes : f32, 
    pub bumpiness : f32, 
    pub is_t_spin : bool, 
}

#[derive(Clone)]
pub struct MovementState{
    pub game : Game, 
    pub moves_so_far : Vec<BotInput>
}

