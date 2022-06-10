#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    EOL,//End of Line
    BOL,//Begining of Line
    SetActiveFilePath,
    LoadFromFilePath,
    PrintMini,
    Quit,
    Save,
    DoNo,
}
