#[derive(Debug)]
pub enum EngineError {
    InvalidCardValue(u8),
    InvalidBid,
    UnfinishedHand,
    NotBegunHand,
    OutOfOrderPlay,
    DoesNotHaveCard,
    HasToFollowSuit,
    HasToTrump,
    HasToOvertrump,
    FinishedHand,
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineError::InvalidCardValue(arg) => write!(f, "Card can not have value {}", arg),
            EngineError::InvalidBid => write!(f, "Invalid bid detected",),
            EngineError::UnfinishedHand => write!(
                f,
                "This operation is not possible as trick is not finished yet",
            ),
            EngineError::FinishedHand => write!(
                f,
                "This operation is not possible as trick is already finished",
            ),
            EngineError::NotBegunHand => write!(
                f,
                "This operation is not possible as trick is not begun yet",
            ),
            EngineError::OutOfOrderPlay => write!(f, "Player tried to play out of turn",),
            EngineError::DoesNotHaveCard => {
                write!(f, "Player does not have  the card he is trying to play",)
            }
            EngineError::HasToFollowSuit => {
                write!(
                    f,
                    "Player has a card of the right color but did not play it",
                )
            }
            EngineError::HasToTrump => {
                write!(f, "Player has a trump but did not play it",)
            }
            EngineError::HasToOvertrump => {
                write!(f, "Player should have overtrumped but did not",)
            }
        }
    }
}

impl std::error::Error for EngineError {}
