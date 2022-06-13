#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Eol,//End of Line
    Bol,//Begining of Line
    Kill,//Cut line after cursor
    Yank,//Paste
    SetActiveFilePath,
    LoadFromFilePath,
    PrintMini,
    Quit,
    Save,
    DoNo,
}
