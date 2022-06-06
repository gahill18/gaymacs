#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    InsertMode,
    ClearBuf,
    SetActiveFilePath,
    LoadFromFilePath,
    PrintMini,
    Quit,
    Save,
    DoNo,
}
