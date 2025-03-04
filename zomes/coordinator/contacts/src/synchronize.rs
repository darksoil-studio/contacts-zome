use hdk::prelude::*;
use private_event_sourcing::*;

use crate::private_event::ContactsEvent;

#[hdk_extern(infallible)]
fn scheduled_synchronize_with_linked_devices(_: Option<Schedule>) -> Option<Schedule> {
    if let Err(err) = commit_my_pending_encrypted_messages::<ContactsEvent>() {
        error!("Failed to commit my encrypted messages: {err:?}");
    }
    if let Err(err) = synchronize_with_linked_devices(()) {
        error!("Failed to synchronize with other agents: {err:?}");
    }

    Some(Schedule::Persisted("*/30 * * * * * *".into())) // Every 30 seconds
}
