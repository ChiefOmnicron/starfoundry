#[derive(Clone, Debug)]
pub struct Finding {
    pub blueprint:  String,
    pub has:        usize,
    pub want:       usize,
    pub action:     FindingAction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FindingAction {
    Critical,
    NonCritical,
    Ignore,
    Unknown,
}
