use crate::utils::lockable::LockableState;

#[derive(PartialEq, Debug, Default)]
pub struct Window {
    pub state: LockableState,
}

impl Window {
    pub fn new(state: LockableState) -> Window {
        Window { state }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_be_created() {
        let w1 = Window::new(LockableState::Open);
        let w2 = Window::new(LockableState::ClosedAndUnlocked);
        let w3 = Window::new(LockableState::Locked);
        assert_eq!(
            w1,
            Window {
                state: LockableState::Open
            }
        );
        assert_eq!(
            w2,
            Window {
                state: LockableState::ClosedAndUnlocked
            }
        );
        assert_eq!(
            w3,
            Window {
                state: LockableState::Locked
            }
        );
    }
}
