#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    SetActiveFilePath,
    LoadFromFilePath,
    PrintMini,
    Quit,
    Save,
    DoNo,
}
