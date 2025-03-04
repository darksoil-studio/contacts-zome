use hdi::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventWithTimestamp {
    pub timestamp: Timestamp,
    pub event: SerializedBytes,
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct PrivateEventEntry {
    pub author: AgentPubKey,
    pub signature: Signature,
    pub event: EventWithTimestamp,
}

pub fn validate_create_private_event(
    _action: EntryCreationAction,
    _event: PrivateEventEntry,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_update_private_event(
    _action: Update,
    _event: PrivateEventEntry,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(format!(
        "PrivateEvents cannot be updated"
    )))
}

pub fn validate_delete_private_event(_action: Delete) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(format!(
        "PrivateEvents cannot be deleted"
    )))
}
