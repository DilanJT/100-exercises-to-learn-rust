use std::sync::mpsc::{Receiver, Sender};

use crate::store::TicketStore;

pub mod data;
pub mod store;

pub enum Command {
    Insert(data::TicketDraft),
}

// Start the system by spawning the server thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: The server task should **never** stop.
//  Enter a loop: wait for a command to show up in
//  the channel, then execute it, then start waiting
//  for the next command.
pub fn server(receiver: Receiver<Command>) {
    loop {
        let rec = receiver.recv();
        match rec {
            Ok(Command::Insert(ticket_draft)) => {

                
                println!("insert ticket draft");

                let mut ticket_store = TicketStore::new();
                let _ = ticket_store.add_ticket(ticket_draft);
            }
            Err(e) => {
                println!("Error occurred");
            }
        }
    }
}
