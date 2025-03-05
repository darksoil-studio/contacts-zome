use hdk::prelude::*;
use private_event_sourcing::{create_private_event, SignedEvent};

use crate::private_event::{query_contacts_events, ContactsEvent, Profile};

#[hdk_extern]
pub fn set_my_profile(profile: Profile) -> ExternResult<()> {
    create_private_event(ContactsEvent::SetMyProfile(profile))?;
    Ok(())
}

#[hdk_extern]
pub fn query_my_profile() -> ExternResult<Option<Profile>> {
    let contacts_events = query_contacts_events()?;

    let mut sorted_events: Vec<SignedEvent<ContactsEvent>> =
        contacts_events.into_values().collect();

    sorted_events.sort_by(|e1, e2| e1.event.timestamp.cmp(&e2.event.timestamp));

    let profile_events: Vec<Profile> = sorted_events
        .into_iter()
        .filter_map(|event| match event.event.content {
            ContactsEvent::SetMyProfile(profile) => Some(profile),
            _ => None,
        })
        .collect();

    Ok(profile_events.last().cloned())
}
