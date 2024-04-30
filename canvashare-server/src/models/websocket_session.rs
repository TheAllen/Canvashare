use actix::prelude::*;
use actix_web_actors::ws;
use std::time::{Duration, Instant};

use crate::models::server_models::{CanvasServer, ClientData, Connect, Data, Disconnect};


// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub struct WsCanvasSession {
    // Unique session id
    pub id: usize,

    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise, we drop the connection
    pub hb: Instant,

    // Joined room
    pub room: String,

    // Websocket session name
    pub name: Option<String>,

    // Chat server
    pub addr: Addr<CanvasServer>
}

impl WsCanvasSession {
    /// helper method that sends pings to client every 5 seconds (HEARTBEAT_INTERVAL).
    /// 
    /// also this method checks heartbeat from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // Check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket client heartbeat failed. Disconnecting!");

                // notify chat server
                act.addr.do_send(Disconnect { id: act.id });

                // Stop actor
                ctx.stop();

                // stop trying to send a ping
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for WsCanvasSession {
    type Context = ws::WebsocketContext<Self>;

    /// This method is called on actor start
    /// We register ws canvas session with CanvasServer
    fn started(&mut self, ctx: &mut Self::Context) {

        // start heartbeat process on session start
        self.hb(ctx);

        // Register self in CanvasServer. "AsyncContext::wait" register future within
        // context, but context waits until this future resolves before processing any
        // other events. HttpContext::state() is an instance of WsCanvasSession, state 
        // is shared acrosss all routes within application.
        let addr: Addr<WsCanvasSession> = ctx.address();
        self.addr
            .send(Connect { 
                addr: addr.recipient() 
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop() // Something went wrong with canvas server
                }
                // waits until this future resolves before processing events
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // Notify chat server about disconnection
        self.addr.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

/// Handling data from canvas server, we simply send it to peer websocket
impl Handler<Data> for WsCanvasSession {
    type Result = ();

    fn handle(&mut self, msg: Data, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
        ()
    }
}

/// Websocket data handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsCanvasSession {
    // All data goes through handle function
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Ok(msg) => msg,
            Err(_) => {
                ctx.stop();
                return;
            }
        };

        // pattern match to various message types
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            },
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            },
            ws::Message::Text(text) => {
                let m = text.trim();

                if m.starts_with("/") {
                    todo!()
                    // Handle the /sss type of messages
                } else {
                    let data = if let Some(ref name) = self.name {
                        format!("{name}: {m}")
                    } else {
                        m.to_owned()
                    };

                    self.addr.do_send(ClientData::<String> {
                        id: self.id,
                        data,
                        room: self.room.clone()
                    })
                }
            },
            ws::Message::Binary(_binary) => {
                todo!()
            },
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            },
            ws::Message::Continuation(_) => {
                ctx.stop();
            },
            ws::Message::Nop => ()
        }
    }

}