#[derive(Eq, Hash, PartialEq)]
pub enum GameLoopPhase {
    Init,
    PreLoop,
    GameLoop,
    PostLoop,
    PrePhysics,
    Physics,
    PostPhysics,
    PreFrame,
    Frame,
    PostFrame,
    Finish
}