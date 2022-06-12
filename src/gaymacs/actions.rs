#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Eol,//End of Line
    Bol,//Begining of Line
    SetActiveFilePath,
    LoadFromFilePath,
    PrintMini,
    Quit,
    Save,
    DoNo,
}
