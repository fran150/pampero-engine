#[derive(Eq, Hash, PartialEq)]
pub enum GameLoopPhase {
    Init,
    PreLoop,
    PrePhysics,
    Physics,
    PostPhysics,
    Loop,
    PreFrame,
    Frame,
    PostFrame,
    PostLoop,
    Finish,
}