use crate::utils::lockable::LockableState;

#[derive(PartialEq, Debug, Default)]
pub struct Door {
    pub state: LockableState,
}

impl Door {
    pub fn new(state: LockableState) -> Door {
        Door { state }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_be_created() {
        let d1 = Door::new(LockableState::Open);
        let d2 = Door::new(LockableState::ClosedAndUnlocked);
        let d3 = Door::new(LockableState::Locked);
        assert_eq!(
            d1,
            Door {
                state: LockableState::Open
            }
        );
        assert_eq!(
            d2,
            Door {
                state: LockableState::ClosedAndUnlocked
            }
        );
        assert_eq!(
            d3,
            Door {
                state: LockableState::Locked
            }
        );
    }
}
