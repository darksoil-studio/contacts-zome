use hdk::prelude::*;
use private_event_sourcing::create_private_event;

use crate::private_event::{query_contacts_events, ContactsEvent};

#[hdk_extern]
pub fn set_my_profile(profile: Profile) -> ExternResult<()> {
    create_private_event(ContactsEvent)?;
    Ok(())
}

#[hdk_extern]
pub fn query_my_profile() -> ExternResult<Option<Profile>> {
    let contacts_entries = query_contacts_events()?;
    let Some(last_profile) = profile_entries.into_values().last() else {
        return Ok(None);
    };
    Ok(Some(last_profile))
}
