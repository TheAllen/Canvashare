#![allow(dead_code)]
use std::{
    collections::{HashMap, HashSet}, 
    sync::{atomic::{AtomicUsize, Ordering}, Arc}
};
use actix::prelude::*;
use bytes::Bytes;
use rand::{rngs::ThreadRng, Rng};


/// Canvas server sends this message to a session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Data(pub String);

/// Binary message from client
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct BinaryMessage(pub Bytes);

pub enum MessageTypes {
    Text(Data),
    Binary(BinaryMessage)
}

// Message for chat server communication

/// New Canvas Session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Data>
}

/// Canvas Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize
}

/// Send canvas data to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientData<T> {
    // id of the client session
    pub id: usize,
    // peer data message
    pub data: T,
    // room name
    pub room: String
}

/// List of available rooms
pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

/// Join room. If room does not exist, create a new room.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    // client id
    pub id: usize,
    pub room_name: String
}

/// CanvasServer manages canvas rooms and responsible for coordination of sessions
#[derive(Debug)]
pub struct CanvasServer {
    // Keeps track of room names as keys and session ids as values
    rooms: HashMap<String, HashSet<usize>>,
    // Maps the session id to the recipient
    sessions: HashMap<usize, Recipient<Data>>,
    // For generating random
    rng: ThreadRng,
    visitor_count: Arc<AtomicUsize>

}

impl CanvasServer {
    pub fn new(visitor_count: Arc<AtomicUsize>) -> CanvasServer {
        //Default main room
        let mut rooms: HashMap<String, HashSet<usize>> = HashMap::new();
        rooms.insert("main".to_owned(), HashSet::new());
        CanvasServer {
            rooms,
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
            visitor_count
        }
    }

    pub fn send_str_data(&self, room: &str, data: &str, skip_id: usize) {
        if let Some(sessions) = self.rooms.get(room) {
            for session in sessions {
                if *session != skip_id {
                    if let Some(addr) = self.sessions.get(session) {
                        addr.do_send(Data(data.to_owned()));
                    }
                }
            }
        }
    }

    // TODO: Create a function to send binary data
}

/// Make actor from "CanvasServer"
impl Actor for CanvasServer {
    // We are going to use simple context - we need the ability to communicate with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect Message.
/// 
/// Connect {
///     addr: Recipient<Message>
/// }
/// 
/// Register new session and assign unique id to this session.
impl Handler<Connect> for CanvasServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _ctx: &mut Context<Self>) -> Self::Result {

        // Notify all users in the same room about someone joining
        // TODO change: right now only joining the main room. 
        self.send_str_data("main", "Someone has joined", 0);

        let id: usize = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        // auto join session to main room
        self.rooms.entry("main".to_owned()).or_default().insert(id);

        let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        self.send_str_data("main", &format!("Total visitors {}", count), 0);

        id
    }
}

/// Handler for Disconnect message
/// 
/// Disconnect {
///     id: usize
/// }
impl Handler<Disconnect> for CanvasServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Context<Self>) -> Self::Result {

        let mut rooms: Vec<String> = Vec::new();

        // Remove address
        if self.sessions.remove(&msg.id).is_some() {
            // remove session from all rooms
            for (room_name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id) {
                    rooms.push(room_name.to_owned());
                }
            }
        }

        // Notify users in rooms
        for room in rooms {
            self.send_str_data(&room, "Someone disconnected", 0);
        }
    }
}

/// Handler for Data message
impl Handler<ClientData<String>> for CanvasServer {
    type Result = ();

    fn handle(&mut self, msg: ClientData<String>, _ctx: &mut Context<Self>) -> Self::Result {
        self.send_str_data(&msg.room, &msg.data, msg.id);
        ()
    }
}

/// Handler for "ListRooms" data
impl Handler<ListRooms> for CanvasServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _msg: ListRooms, _ctx: &mut Context<Self>) -> Self::Result {
        let mut listrooms: Vec<String> = Vec::new();

        for key in self.rooms.keys() {
            listrooms.push(key.to_owned());
        }

        MessageResult(listrooms)
    }
}

/// Join room, send notification to room
/// 
/// Join {
///     id: usize,
///     room_name: String
/// }
/// 
/// Send join message to new room
impl Handler<Join> for CanvasServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _ctx: &mut Context<Self>) -> Self::Result {
        let Join { id, room_name} = msg;
        let mut rooms: Vec<String> = Vec::new();

        // remove session from all rooms
        for (name, sessions) in &mut self.rooms {
            if sessions.remove(&id) {
                rooms.push(name.to_owned());
            }
        }

        /*
        Send message to other users
        */
        for room in rooms {
            self.send_str_data(&room, "Someone disconnected", id);
        }

        self.rooms.entry(room_name.clone()).or_default().insert(id);

        self.send_str_data(&room_name, "Someone connected", id);

        ()
    }
}