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
    /// Profile
    SetMyProfile(Profile),
    /// Share Contact Request
    ShareContactRequest {
        from: Contact,
        to: Contact,
    },
    AcceptShareContactRequest(EntryHash),
    RejectShareContactRequest(EntryHash),
    CancelShareContactRequest(EntryHash),
    /// Contacts
    ContactAdded(Contact),
    ContactUpdated {
        contact_added_hash: EntryHash,
        new_contact: Contact,
    },
    ContactRemoved {
        contact_added_hash: EntryHash,
    },
}

impl PrivateEvent for ContactsEvent {
    fn validate(&self) -> ExternResult<ValidateCallbackResult> {
        Ok(ValidateCallbackResult::Valid)
    }
    fn recipients(&self) -> ExternResult<Vec<AgentPubKey>> {
        // match self {
        //     ContactsEvent::SetMyProfile()
        // }
        Ok(vec![])
    }
}

pub fn query_contacts_events() -> ExternResult<BTreeMap<EntryHashB64, SignedEvent<ContactsEvent>>> {
    query_private_events()
}

#[hdk_extern]
pub fn post_commit_private_event(private_event_entry: PrivateEventEntry) -> ExternResult<()> {
    private_event_sourcing::post_commit_private_event::<ContactsEvent>(private_event_entry)?;

    Ok(())
}

#[hdk_extern]
pub fn attempt_commit_awaiting_deps_entries() -> ExternResult<()> {
    private_event_sourcing::attempt_commit_awaiting_deps_entries::<ContactsEvent>()?;

    Ok(())
}
