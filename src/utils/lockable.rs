
#[derive(PartialEq, Debug, Default)]
pub enum LockableState {
    #[default]
    ClosedAndUnlocked,
    Open,
    Locked,
}

#[derive(PartialEq, Debug)]
pub enum LockableError {
    Locked,
    Open,
    AlreadyClosed,
    AlreadyLocked,
    AlreadyOpen,
}

impl std::error::Error for LockableError {}
impl std::fmt::Display for LockableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            LockableError::AlreadyClosed => write!(f, "This is already closed"),
            LockableError::AlreadyLocked => write!(f, "This is already locked"),
            LockableError::AlreadyOpen => write!(f, "This is already open"),
            LockableError::Locked => write!(f, "This can't be opened because it's locked"),
            LockableError::Open => write!(f, "This can't be locked because it's open."),
        }
    }
}

pub trait Lockable {
    fn can_close(&self) -> bool;
    fn can_open(&self) -> bool;
    fn close_and_lock(&mut self) -> Result<(), LockableError>;
    fn close(&mut self) -> Result<(), LockableError>;
    fn is_closed(&self) -> bool;
    fn is_locked(&self) -> bool;
    fn is_open(&self) -> bool;
    fn lock(&mut self) -> Result<(), LockableError>;
    fn open(&mut self) -> Result<(), LockableError>;
}
