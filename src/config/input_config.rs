use ggez::input::keyboard::KeyCode;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum GameAction {
    MoveRight,
    MoveLeft,
    SoftDrop,
    HardDrop,
    RotateCw,
    RotateCcw,
    Rotate180,
    HoldPiece,
    Up,
}

/*
    Returns default_keyboard_keybindings
*/
pub fn default_keyboard_keybindings () -> HashMap<GameAction, KeyCode> {
    use GameAction::*;

    let mut map = HashMap::new();
    map.insert(MoveRight, KeyCode::Right);
    map.insert(MoveLeft, KeyCode::Left);
    map.insert(SoftDrop, KeyCode::Down);
    map.insert(HardDrop, KeyCode::Space);
    map.insert(RotateCw, KeyCode::X);
    map.insert(RotateCcw, KeyCode::Z);
    map.insert(Rotate180, KeyCode::A);
    map.insert(HoldPiece, KeyCode::C);
    map.insert(Up,KeyCode::Up);
    map
    
}

/*
    Returns keybindings for 1v1 on keyboard
*/
pub fn multi_controller_keyboard_keybindings() -> Vec<HashMap<GameAction, KeyCode>> {
    use GameAction::*;

    let mut player1 = HashMap::new();
    player1.insert(MoveRight, KeyCode::A);
    player1.insert(SoftDrop, KeyCode::S);
    player1.insert(MoveLeft, KeyCode::D);
    player1.insert(HardDrop, KeyCode::X);
    player1.insert(RotateCcw, KeyCode::Q);
    player1.insert(Rotate180, KeyCode::W);
    player1.insert(RotateCw, KeyCode::E);
    player1.insert(HoldPiece, KeyCode::Z);
    player1.insert(Up, KeyCode::Up);

    let mut player2 = HashMap::new();
    player2.insert(MoveRight, KeyCode::L);
    player2.insert(SoftDrop, KeyCode::K);
    player2.insert(MoveLeft, KeyCode::J);
    player2.insert(HardDrop, KeyCode::Comma);
    player2.insert(RotateCcw, KeyCode::U);
    player2.insert(Rotate180, KeyCode::I);
    player2.insert(RotateCw, KeyCode::O);
    player2.insert(HoldPiece, KeyCode::M);
    player2.insert(Up, KeyCode::Up);


    vec![player1, player2]
}

/*
    Returns keybindings for drivarkaden 
    if single player -- only use player1
*/
pub fn default_drivarkaden_keybindings() -> Vec<HashMap<GameAction, KeyCode>>{
    use GameAction::*;

    let mut player1 = HashMap::new();
    player1.insert(MoveRight, KeyCode::Right);
    player1.insert(MoveLeft, KeyCode::Left);
    player1.insert(SoftDrop, KeyCode::Down);
    player1.insert(HardDrop, KeyCode::LAlt); // A on drivarkaden (i think bad handwriting maybe)
    player1.insert(RotateCw, KeyCode::LShift); // X
    player1.insert(RotateCcw, KeyCode::Space); // Y
    player1.insert(Rotate180, KeyCode::Z); // top right
    player1.insert(HoldPiece, KeyCode::LControl); // B
    player1.insert(Up, KeyCode::Up);



    let mut player2 = HashMap::new();
    player2.insert(MoveRight, KeyCode::G);
    player2.insert(MoveLeft, KeyCode::D);
    player2.insert(SoftDrop, KeyCode::F);
    player2.insert(HardDrop, KeyCode::S); // A
    player2.insert(RotateCw, KeyCode::Q); // X
    player2.insert(RotateCcw, KeyCode::W); // Y
    player2.insert(Rotate180, KeyCode::I); // Top Right
    player2.insert(HoldPiece, KeyCode::A); //B
    player2.insert(Up, KeyCode::R);



    vec![player1, player2]
}