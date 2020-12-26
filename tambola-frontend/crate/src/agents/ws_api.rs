use yew::agent::{Agent, HandlerId, AgentLink};
use yew::prelude::worker::Context;
use std::collections::HashSet;
use anyhow::Error;
use yew::format::{Json, Nothing, Toml};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::{html, Component, ComponentLink, Html, ShouldRender, Bridge};
use yew::services::{DialogService, StorageService, Task, ConsoleService};
use serde::{Deserialize,Serialize};
use wasm_bindgen::__rt::std::process::Output;
use tambola_lib::game::proto::{Input, AnnouncementOutput};
use yewtil::store::{StoreWrapper, Bridgeable, ReadOnly};
use agents::store::{TambolaStore, StoreInput};
use yew_router::service::RouteService;
use yew_router::prelude::RouteAgentDispatcher;
use yew_router::agent::RouteRequest;
use router::TambolaRouter;
use yew_router::route::Route;
use yew::services::storage::Area;
use tambola_lib::game::User;

pub enum Command{
    SendData(Input)
}

pub enum InternalMessage{
    Connected,
    Lost,
    WsMessage(Result<tambola_lib::game::proto::Output, Error>),
    StrMessage(ReadOnly<TambolaStore>)
}
pub struct WSApi{
    connected:bool,
    ws:WebSocketTask,
    buffer:Vec<Input>,
    subscribers: HashSet<HandlerId>,
    store:Box<dyn Bridge<StoreWrapper<TambolaStore>>>
}
impl Agent for WSApi{
    type Reach = Context<Self>;
    type Message = InternalMessage;
    type Input = Command;
    type Output = ();

    fn create(link: AgentLink<Self>) -> Self {
        let callback = link.callback(InternalMessage::StrMessage);
        let ws_callback = link.callback(|Json(data)| InternalMessage::WsMessage(data));
        let notification = link.callback(|status| match status {
            WebSocketStatus::Opened => InternalMessage::Connected,
            WebSocketStatus::Closed | WebSocketStatus::Error => {
                InternalMessage::Lost
            }
        });
        let ws =
            WebSocketService::connect("ws://localhost:8888/api", ws_callback, notification)
                .unwrap();
        Self{
            ws,
            subscribers: HashSet::new(),
            store:TambolaStore::bridge(callback),
            buffer:vec![],
            connected:false,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Self::Message::WsMessage(result)=>{
                let data = result.map(|data| data).ok();
                if let Some(op) = data {
                    match op {
                        tambola_lib::game::proto::Output::ConnectionEstablished(cs)=>{
                            self.store.send(StoreInput::Connected);
                        },
                        tambola_lib::game::proto::Output::NewGameHosted(ngho)=>{
                            if let Ok(mut storage) = StorageService::new(Area::Local) {
                                storage.store("user", Json(&ngho.user));
                            }
                            let route = TambolaRouter::Game(ngho.game_id.to_string());
                            RouteAgentDispatcher::new().send(RouteRequest::ChangeRoute::<()>(Route::from(route)));
                            self.store.send(StoreInput::NewGameHosted(ngho.game_id.to_string(),ngho.user.clone()));
                        },
                        tambola_lib::game::proto::Output::Announcement(anc)=>{
                            ConsoleService::log("Announcement");
                            self.store.send(StoreInput::NewAnnouncement(anc));
                        },
                        tambola_lib::game::proto::Output::ReconnectedToGame(rtgo)=>{
                            self.store.send(StoreInput::Reconnected(rtgo.user.clone(),rtgo.snapshot.clone(),rtgo.announcements));
                        },
                        tambola_lib::game::proto::Output::ConnectedToGame(ctgo)=>{
                            if let Ok(mut storage) = StorageService::new(Area::Local) {
                                storage.store("user", Json(&ctgo.user));
                            }
                            self.store.send(StoreInput::ConnectedToGame(ctgo.user,ctgo.snapshot,ctgo.announcements))
                        },
                        tambola_lib::game::proto::Output::ClaimNumberSuccess(cns)=>{
                            ConsoleService::log("Claim Number success");
                            self.store.send(StoreInput::ClaimSuccess(cns.number));
                        },
                        tambola_lib::game::proto::Output::ClaimNumberFailure(cnf)=>{
                            ConsoleService::log("Claim Number Failed");
                        },
                        _=>{
                            DialogService::alert("Some other output")
                        }
                    }
                }
            },
            Self::Message::Connected=>{
                self.connected = true;
                for data in self.buffer.clone() {
                    self.ws.send(Json(&data));
                }
                self.buffer = vec![];
            }
            _=>{}
        }
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        match msg {
            Command::SendData(data)=>{
                if self.connected {
                    ConsoleService::log("Sending Data");
                    self.ws.send(Json(&data))
                } else {
                    self.buffer.push(data);
                }

            }
        }
    }
    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}