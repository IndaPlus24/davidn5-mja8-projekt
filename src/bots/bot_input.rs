#[derive(Clone, Debug, PartialEq, Copy)]
#[allow(unused)]
pub enum BotInput {
    MoveLeft,
    MoveRight,
    MoveDown,
    RotateCW,
    RotateCCW,
    HardDrop,
    Hold
}