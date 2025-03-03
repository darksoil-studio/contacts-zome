use contacts_integrity::*;
use hdk::prelude::*;

use crate::private_contacts_entry::query_undeleted_private_contacts_entries;

#[hdk_extern]
pub fn query_all_contacts() -> ExternResult<Vec<Contact>> {
    let entries = query_undeleted_private_contacts_entries()?;
    let contacts = entries
        .into_iter()
        .filter_map(|(_action_hash, entry)| match entry {
            PrivateContactsEntry::AddContact(contact) => Some(contact),
            _ => None,
        })
        .collect();
    Ok(contacts)
}
