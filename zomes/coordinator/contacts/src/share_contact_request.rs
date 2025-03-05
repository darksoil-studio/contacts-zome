use hdk::prelude::*;
use private_event_sourcing::{create_private_event, query_all_my_agents, PrivateEvent};
use std::collections::BTreeMap;

use crate::{
    private_event::{Contact, ContactsEvent},
    profile::query_my_profile,
};

fn build_my_contact() -> ExternResult<Option<Contact>> {
    let Some(my_profile) = query_my_profile(())? else {
        return Ok(None);
    };
    let agents = query_all_my_agents()?;

    Ok(Some(Contact {
        agents,
        profile: my_profile,
    }))
}

#[hdk_extern]
pub fn send_share_contact_request(peer_contact: Contact) -> ExternResult<()> {
    let Some(my_contact) = build_my_contact()? else {
        return Err(wasm_error!(
            "Can't request to share contacts before creating a profile."
        ));
    };

    create_private_event(ContactsEvent::ShareContactRequest {
        from: my_contact,
        to: peer_contact,
    })?;
    Ok(())
}

#[hdk_extern]
pub fn accept_share_contact_request(share_contact_request: EntryHash) -> ExternResult<()> {
    create_private_event(ContactsEvent::AcceptShareContactRequest(
        share_contact_request,
    ))?;
    Ok(())
}

#[hdk_extern]
pub fn reject_share_contact_request(share_contact_request: EntryHash) -> ExternResult<()> {
    create_private_event(ContactsEvent::RejectShareContactRequest(
        share_contact_request,
    ))?;
    Ok(())
}

#[hdk_extern]
pub fn cancel_share_contact_request(share_contact_request: EntryHash) -> ExternResult<()> {
    create_private_event(ContactsEvent::CancelShareContactRequest(
        share_contact_request,
    ))?;
    Ok(())
}
