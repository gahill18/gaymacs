#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    InsertMode,
    ClearBuf,
    SetActiveFilePath,
    PrintMini,
    Quit,
    Save,
    DoNo,
}
