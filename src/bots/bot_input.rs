#[derive(Clone, Debug)]
pub enum BotInput {
    MoveLeft,
    MoveRight,
    MoveDown,
    RotateCW,
    RotateCCW,
    HardDrop
}