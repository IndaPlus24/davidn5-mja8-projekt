#[derive(Clone, Debug, PartialEq)]
pub enum BotInput {
    MoveLeft,
    MoveRight,
    MoveDown,
    RotateCW,
    RotateCCW,
    HardDrop
}