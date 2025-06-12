use std::sync::mpsc::{Receiver, Sender};
use crate::store::{TicketId, TicketStore};

pub mod data;
pub mod store;

use crate::data::Ticket;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert { 
        draft: data::TicketDraft,
        response_sender: Sender<TicketId>
    },
    Get { 
        id: TicketId,
        response_sender: Sender<data::Ticket>
    }
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {draft, response_sender }) => {
                println!("Draft {:?}", draft.title);
                let id = store.add_ticket(draft);
                if let Err(e) = response_sender.send(id) {
                    println!("Error {:?}", e);
                }
            }
            Ok(Command::Get {
                id, response_sender
            }) => {
                if let Some(ticket) = store.get(id) {
                    if let Err(e) = response_sender.send(ticket.to_owned()) {
                        println!("Error {:?}", e);
                    }
                }
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break
            },
        }
    }
}
