use std::collections::BTreeMap;

use hdk::prelude::*;
use private_event_sourcing::*;

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

#[derive(Serialize, Deserialize, Debug, Clone, SerializedBytes)]
#[serde(tag = "type")]
pub enum ContactsEvent {
    SetMyProfile(Profile),
    ContactAdded(Contact),
    ContactUpdated(Contact),
    ContactRemoved(Vec<AgentPubKey>),
    ShareContactRequest { from: Contact, to: Contact },
    AcceptShareContactRequest(EntryHash),
    RejectShareContactRequest(EntryHash),
    CancelShareContactRequest(EntryHash),
}

impl PrivateEvent for ContactsEvent {
    fn validate(&self) -> ExternResult<ValidateCallbackResult> {
        Ok(ValidateCallbackResult::Valid)
    }
    fn recipients(&self) -> ExternResult<Vec<AgentPubKey>> {
        Ok(vec![])
    }
}

pub fn query_contacts_events() -> ExternResult<BTreeMap<EntryHashB64, ContactsEvent>> {
    query_private_events()
}

#[hdk_extern]
pub fn post_commit_private_event(private_event_entry: PrivateEventEntry) -> ExternResult<()> {
    let private_event =
        ContactsEvent::try_from(private_event_entry.0).map_err(|err| wasm_error!(err))?;
    private_event_sourcing::post_commit_private_event::<ContactsEvent>(private_event)?;

    Ok(())
}
