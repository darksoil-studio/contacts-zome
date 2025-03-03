use contacts_integrity::*;
use hdk::prelude::*;
use std::collections::BTreeMap;

use crate::{
    linked_devices::query_all_my_agents,
    private_contacts_entry::query_undeleted_private_contacts_entries, profile::query_my_profile,
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
    create_entry(EntryTypes::PrivateContactsEntry(
        PrivateContactsEntry::OutgoingContactRequest(peer_contact),
    ))?;
    Ok(())
}

#[hdk_extern]
pub fn query_incoming_share_contact_requests() -> ExternResult<BTreeMap<ActionHashB64, Contact>> {
    let entries = query_undeleted_private_contacts_entries()?;
    let incoming_share_contact_requests_entries = entries
        .into_iter()
        .filter_map(|(action_hash, entry)| match entry {
            PrivateContactsEntry::IncomingContactRequest(contact) => {
                Some((ActionHashB64::from(action_hash), contact))
            }
            _ => None,
        })
        .collect();
    Ok(incoming_share_contact_requests_entries)
}

#[hdk_extern]
pub fn query_outgoing_share_contact_requests() -> ExternResult<BTreeMap<ActionHashB64, Contact>> {
    let entries = query_undeleted_private_contacts_entries()?;
    let outgoing_share_contact_requests_entries = entries
        .into_iter()
        .filter_map(|(action_hash, entry)| match entry {
            PrivateContactsEntry::OutgoingContactRequest(contact) => {
                Some((ActionHashB64::from(action_hash), contact))
            }
            _ => None,
        })
        .collect();
    Ok(outgoing_share_contact_requests_entries)
}

#[hdk_extern]
pub fn accept_share_contact_request(
    incoming_share_contact_request: ActionHash,
) -> ExternResult<()> {
    let entries = query_undeleted_private_contacts_entries()?;

    let Some(private_contacts_entry) = entries.get(&incoming_share_contact_request) else {
        return Err(wasm_error!("IncomingShareContactRequest not found."));
    };
    let PrivateContactsEntry::IncomingContactRequest(contact) = private_contacts_entry else {
        return Err(wasm_error!(
            "The given action hash is not for an IncomingShareContactRequest."
        ));
    };

    create_entry(EntryTypes::PrivateContactsEntry(
        PrivateContactsEntry::AddContact(contact.clone()),
    ))?;
    delete_entry(incoming_share_contact_request)?;
    Ok(())
}

#[hdk_extern]
pub fn reject_share_contact_request(
    incoming_share_contact_request: ActionHash,
) -> ExternResult<()> {
    delete_entry(incoming_share_contact_request)?;
    Ok(())
}

#[hdk_extern]
pub fn cancel_share_contact_request(
    outgoing_share_contact_request: ActionHash,
) -> ExternResult<()> {
    delete_entry(outgoing_share_contact_request)?;
    Ok(())
}
