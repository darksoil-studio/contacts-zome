use std::collections::BTreeMap;

use hdk::prelude::*;
use private_event_sourcing::SignedEvent;

use crate::private_event::{query_contacts_events, Contact, ContactsEvent};

#[hdk_extern]
pub fn query_all_contacts() -> ExternResult<Vec<Contact>> {
    let contacts_events = query_contacts_events()?;

    let mut contacts_addeds: BTreeMap<EntryHashB64, Contact> = contacts_events
        .iter()
        .filter_map(
            |(entry_hash, contact_event)| match &contact_event.event.content {
                ContactsEvent::ContactAdded(contact) => Some((entry_hash.clone(), contact.clone())),
                _ => None,
            },
        )
        .collect();

    for contact_event in contacts_events.values() {
        let ContactsEvent::ContactRemoved { contact_added_hash } = &contact_event.event.content
        else {
            continue;
        };
        contacts_addeds.remove(&EntryHashB64::from(contact_added_hash.clone()));
    }

    let mut sorted_events: Vec<SignedEvent<ContactsEvent>> =
        contacts_events.into_values().collect();

    sorted_events.sort_by(|e1, e2| e1.event.timestamp.cmp(&e2.event.timestamp));

    for contact_event in sorted_events {
        let ContactsEvent::ContactUpdated {
            contact_added_hash,
            new_contact,
        } = contact_event.event.content
        else {
            continue;
        };
        if contacts_addeds.contains_key(&EntryHashB64::from(contact_added_hash.clone())) {
            contacts_addeds.insert(contact_added_hash.clone().into(), new_contact);
        }
    }

    Ok(contacts_addeds.into_values().collect())
}
