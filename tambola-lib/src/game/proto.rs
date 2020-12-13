use serde::{Deserialize,Serialize};
use crate::game::{Winning, Draw, UserType, User, GameSnapshot, Ticket};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum Input{
    Nill,
    HostNewGame(HostNewGameInput),
    ConfigureAndStart(ConfigureAndStartInput),
    DrawNumber(DrawNumberInput),
    ClaimNumber(ClaimNumberInput),
    ClaimWin(ClaimWinInput),
    SendMessage(SendMessageInput),
    Reconnect(ReconnectInput),
    ConnectMeAs(ConnectMeAsInput)
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostNewGameInput {
    pub name: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigureAndStartInput {
    pub winnings: Vec<Winning>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawNumberInput {
    pub draw: Draw,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimNumberInput {
    pub number: u8,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimWinInput {
    pub win_name: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageInput {
    pub message: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReconnectInput {
    pub user_id:Uuid,
    pub game_id:Uuid,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectMeAsInput {
    pub game:Uuid,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum Output{
    ConnectionEstablished(ConnectionEstablished),
    NewGameHosted(NewGameHostedOutput),
    Announcement(AnnouncementOutput),
    ClaimNumberSuccess(ClaimNumberSuccessOutput),
    ClaimNumberFailure(ClaimNumberFailureOutput),
    ReconnectedToGame(ReconnectedToGameOutput),
    ReconnectFailure(ReconnectFailureOutput),
    ConnectedToGame(ConnectedToGameOutput),
    NotConnectedToGame(NotConnectedToGameOutput)
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionEstablished {
    pub message:String
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewGameHostedOutput {
    pub game_id: Uuid,
    pub user:User
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum AnnouncementOutput {
    NewNumber(NewNumberAnnouncement),
    NewMessage(NewMessageAnnouncement),
    NewWinner(NewWinnerAnnouncement),
    GameStarted(GameStartedAnnoucement),
    NewUserJoined(NewUserJoinedAnnoucement),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewNumberAnnouncement {
    pub number: u8
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewMessageAnnouncement {
    pub message: String,
    pub user:String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewWinnerAnnouncement {
    pub win_name:String,
    pub user_name:String,
    pub ticket:Ticket
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameStartedAnnoucement {
    pub winnings: Vec<Winning>
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUserJoinedAnnoucement {
    pub name:String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimNumberSuccessOutput {
    pub number: u8
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimNumberFailureOutput {
    pub number: u8,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReconnectedToGameOutput {
    pub snapshot: GameSnapshot,
    pub user:User,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReconnectFailureOutput {
    pub reason:String
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectedToGameOutput {
    pub snapshot: GameSnapshot,
    pub user:User,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotConnectedToGameOutput {
    pub reason:String
}
impl Output{
    pub fn new_connection_established()->Self{
        Output::ConnectionEstablished(ConnectionEstablished{
            message:format!("You are connected to server now")
        })
    }
    pub fn new_game_hosted(game_id:Uuid,user:User)->Self{
        Output::NewGameHosted(NewGameHostedOutput{
            game_id,
            user
        })
    }
    pub fn new_reconnected_to_game(user:User,snapshot:GameSnapshot)->Self{
        Output::ReconnectedToGame(ReconnectedToGameOutput{
            user,
            snapshot,
        })
    }
    pub fn game_started(winnings:Vec<Winning>)->Self{
        Output::Announcement(AnnouncementOutput::GameStarted(GameStartedAnnoucement{
            winnings,
        }))
    }
    pub fn new_number(num:u8)->Self{
        Output::Announcement(AnnouncementOutput::NewNumber(NewNumberAnnouncement{
            number:num
        }))
    }
    pub fn claim_number_success(num:u8)->Self{
        Output::ClaimNumberSuccess(ClaimNumberSuccessOutput{
            number:num
        })
    }
    pub fn new_message(user:String,message:String)->Self{
        Output::Announcement(AnnouncementOutput::NewMessage(NewMessageAnnouncement{
            user,
            message
        }))
    }
    pub fn new_winner(win_name:String,user_name:String,ticket:Ticket)->Self{
        Output::Announcement(AnnouncementOutput::NewWinner(NewWinnerAnnouncement{
            win_name,
            user_name,
            ticket
        }))
    }
    pub fn connected_to_game(user:User,snapshot:GameSnapshot)->Self{
        Output::ConnectedToGame(ConnectedToGameOutput{
            user,
            snapshot
        })
    }
    pub fn connect_me_failed(reason:String)->Self{
        Output::NotConnectedToGame(NotConnectedToGameOutput{
            reason
        })
    }
    pub fn user_joined(name:String)->Self{
        Output::Announcement(AnnouncementOutput::NewUserJoined(NewUserJoinedAnnoucement{
            name
        }))
    }
}

