#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState{
    AwaitingInputs, 
    PlayerTurn,
    MonsterTurn
}