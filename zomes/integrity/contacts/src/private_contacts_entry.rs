use std::collections::BTreeMap;

use hdi::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub avatar: Option<String>,
    pub custom_fields: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contact {
    pub agents: Vec<AgentPubKey>,
    pub profile: Profile,
}

#[hdk_entry_helper]
#[derive(Clone)]
#[serde(tag = "type")]
pub enum PrivateContactsEntry {
    SetMyProfile(Profile),
    AddContact(Contact),
    OutgoingContactRequest(Contact),
    IncomingContactRequest(Contact),
}

pub fn validate_create_private_contacts_entry(
    _action: EntryCreationAction,
    _entry: PrivateContactsEntry,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_update_private_contacts_entry(
    _action: Update,
    _entry: PrivateContactsEntry,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(format!(
        "PrivateContactsEntries cannot be updated"
    )))
}

pub fn validate_delete_private_contacts_entry(
    action: Delete,
) -> ExternResult<ValidateCallbackResult> {
    let create = must_get_action(action.deletes_address)?;
    if action.author.ne(create.hashed.content.author()) {
        return Ok(ValidateCallbackResult::Invalid(format!(
            "PrivateContactsEntries can only be deleted by their authors"
        )));
    }

    Ok(ValidateCallbackResult::Valid)
}
