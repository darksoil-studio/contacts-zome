use hdk::prelude::*;
use private_event_sourcing::create_private_event;

use crate::private_event::ContactsEvent;

#[hdk_extern]
pub fn set_my_profile(profile: Profile) -> ExternResult<()> {
    create_private_event(ContactsEvent)?;
    Ok(())
}
