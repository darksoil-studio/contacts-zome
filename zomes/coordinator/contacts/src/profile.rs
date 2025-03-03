use contacts_integrity::*;
use hdk::prelude::*;

use crate::private_contacts_entry::query_undeleted_private_contacts_entries;

#[hdk_extern]
pub fn set_my_profile(profile: Profile) -> ExternResult<()> {
    let profile_entries = query_undeleted_set_my_profile_entries()?;
    for (action_hash, _profile) in profile_entries {
        delete_entry(action_hash)?;
    }

    create_entry(EntryTypes::PrivateContactsEntry(
        PrivateContactsEntry::SetMyProfile(profile),
    ))?;

    Ok(())
}

fn query_undeleted_set_my_profile_entries() -> ExternResult<BTreeMap<ActionHash, Profile>> {
    let entries = query_undeleted_private_contacts_entries()?;
    let profile_entries = entries
        .into_iter()
        .filter_map(|(action_hash, entry)| match entry {
            PrivateContactsEntry::SetMyProfile(profile) => Some((action_hash, profile)),
            _ => None,
        })
        .collect();
    Ok(profile_entries)
}

#[hdk_extern]
pub fn query_my_profile() -> ExternResult<Option<Profile>> {
    let profile_entries = query_undeleted_set_my_profile_entries()?;
    let Some(last_profile) = profile_entries.into_values().last() else {
        return Ok(None);
    };
    Ok(Some(last_profile))
}
