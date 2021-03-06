use crate::{Challenge, GameState, Player, PlayerStats};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum ClientToServerMessage {
    SetPlayerName {
        name: String,
    },
    ReadyForGame,
    VoteKickPlayer {
        player_id: Uuid,
    },
    RevertVoteKickPlayer {
        player_id: Uuid,
    },
    Answers {
        answers: Vec<String>,
    },
    SetCharacter {
        stats: PlayerStats,
    },
    RejectClue,
    OfferChallenge {
        challenge: Challenge,
    },
    OfferChallengeFinal {
        challenge: Challenge,
        clue_idx: usize,
    },
    ChallengeAccepted,
    ChallengeRejected,
    UseArtifact,
    TakeWound,
    AcceptFate,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum ServerToClientMessage {
    GameIsFull,
    GameIsOngoing,
    PushState {
        players: HashMap<Uuid, Player>,
        game_state: GameState,
    },
    Questions {
        questions: Vec<(String, Option<String>)>,
    },
    PushClue {
        clue: String,
    },
    ClueRejectionRejected,
    AbortedChallenge,
    ChallengeResult(ChallengeResult),
}

impl ClientToServerMessage {
    pub fn into_json(self) -> Result<String, serde_json::error::Error> {
        serde_json::to_string(&self)
    }
}

impl ServerToClientMessage {
    pub fn into_json(self) -> Result<String, serde_json::error::Error> {
        serde_json::to_string(&self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChallengeResult {
    pub rolls: Vec<u8>,
    pub success: bool,
    pub possession: bool,
    pub can_use_artifact: bool,
}
