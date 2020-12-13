use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};
use tokio::sync::{mpsc, RwLock};
use warp::Filter;
use warp::ws::{Message, WebSocket};
use futures::{FutureExt, StreamExt};
use futures::stream::SplitStream;
use crate::game::hub::Hub;

use crate::Config;
use tambola_lib::game::proto::{Output, Input};
use std::result;
use warp::Error;
use std::collections::HashMap;

pub struct Server{
    port: u16,
    hub:Arc<Hub>
}
pub type Result<T> = result::Result<T, Error>;
impl Server{
    pub fn new(port: u16) -> Self {
        let games = RwLock::new(HashMap::new());
        Server {
            port,
            hub:Arc::new(Hub{
                games
            })
        }
    }
    pub async fn run(&self,config:Config) {
        let hub = self.hub.clone();
        let web = warp::get()
            .and(warp::fs::dir(config.wroot.clone()));
        let ws_api = warp::path("api")
            .and(warp::ws())
            .and(warp::any().map(move || hub.clone()))
            .map(
                move |ws: warp::ws::Ws,
                      hub: Arc<Hub>| {
                    ws
                        .on_upgrade(move |web_socket| async move {
                            tokio::spawn(Self::user_connected(hub,web_socket));
                        })
                },
            );
        let site=ws_api.or(web);

        let shutdown = async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install CTRL+C signal handler");
        };
        let (_, serving) =
            warp::serve(site).bind_with_graceful_shutdown(([0, 0, 0, 0], self.port), shutdown);


        tokio::select! {
        _ = serving => {}
        }
    }
    async fn user_connected(
        hub: Arc<Hub>,
        web_socket: WebSocket,
    ) {

        // Split the socket into a sender and receive of messages.
        let (user_ws_tx, user_ws_rx) = web_socket.split();
        let (tx, rx) = mpsc::unbounded_channel();
        tokio::task::spawn(rx.forward(user_ws_tx).map(|result| {
            if let Err(e) = result {
                eprintln!("websocket send error: {}", e);
            }
        }));
        let user_s_handle = UserWSSHandle::new(tx);
        let user_r_handle = UserWSRHandle::new(user_ws_rx);
        user_s_handle.send(Output::new_connection_established());
        hub.start_or_resume(user_s_handle,user_r_handle).await;
    }
}
pub struct UserWSSHandle{
    pub tx:mpsc::UnboundedSender<Result<Message>>,
}
pub struct UserWSRHandle{
    pub user_ws_rx:SplitStream<WebSocket>
}
impl UserWSSHandle {
    pub fn new(tx:mpsc::UnboundedSender<Result<Message>>)->Self{
        UserWSSHandle{
            tx
        }
    }
    pub fn send(&self,output:Output){
        if let Err(_disconnected) = self.tx.send(Ok(Message::text(serde_json::to_string(&output).unwrap()))) {

        }
    }

}
impl UserWSRHandle {
    pub fn new(user_ws_rx:SplitStream<WebSocket>)->Self{
        UserWSRHandle{
            user_ws_rx
        }
    }

    pub async fn get_message(&mut self)->Input{
        let mut ret=Input::Nill;
        ret=if let Some(result) = self.user_ws_rx.next().await {
            let message = match result {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("{:?}",e);
                    unimplemented!()
                }
            };
            let input:Input = serde_json::from_str(message.to_str().unwrap()).unwrap();
            input
        } else {
            ret
        };
        ret
    }

}