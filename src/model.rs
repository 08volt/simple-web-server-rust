//! Simplistic Model Layer
//! (with mock-store layer)
use crate::{Error, Result};
use core::time;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// region: -- Ticket Types
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCrate {
    pub title: String,
}
// endregion: -- Ticket Types

// region: -- Model Controller
#[derive(Clone)] // Clone just the Arc, Mutex makes the access to vectore exclusive
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>, // Option just for mock
}

// Contructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

// CRUD Implementation
impl ModelController {
    pub async fn create_ticket(&self, ticket_fc: TicketForCrate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;

        let ticket = Ticket {
            id,
            title: ticket_fc.title,
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_ticket(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailNotFound { id })
    }
}

// endregion: -- Model Controller
