use hdk::prelude::*;
use private_event_sourcing_integrity::*;
use std::collections::BTreeMap;

use crate::{
    agent_encrypted_message::create_encrypted_message, linked_devices::query_my_linked_devices,
    utils::create_relaxed, PrivateEventSourcingRemoteSignal,
};

pub trait PrivateEvent: TryFrom<SerializedBytes> + TryInto<SerializedBytes> {
    fn validate(&self) -> ExternResult<ValidateCallbackResult>;
}

pub fn create_private_event<T: PrivateEvent>(event: T) -> ExternResult<EntryHash> {
    let validation_outcome = event.validate()?;

    let ValidateCallbackResult::Valid = validation_outcome else {
        return Err(wasm_error!(
            "Validation for private event failed: {validation_outcome:?}"
        ));
    };

    let bytes: SerializedBytes = event
        .try_into()
        .map_err(|err| wasm_error!("Failed to serialize private event: {err:?}"))?;
    let entry = PrivateEventEntry(bytes);
    let entry_hash = hash_entry(&entry)?;

    create_entry(EntryTypes::PrivateEvent(entry))?;

    Ok(entry_hash)
}

fn check_is_linked_device(agent: AgentPubKey) -> ExternResult<()> {
    let my_devices = query_my_linked_devices()?;
    if my_devices.contains(&agent) {
        Ok(())
    } else {
        Err(wasm_error!("Given agent is not a linked device."))
    }
}

pub fn receive_private_event_from_linked_device<T: PrivateEvent>(
    provenance: AgentPubKey,
    private_event_entry: PrivateEventEntry,
) -> ExternResult<()> {
    debug!("[receive_private_event_from_linked_device/start]");

    check_is_linked_device(provenance)?;

    let private_event = T::try_from(private_event_entry.0)
        .map_err(|err| wasm_error!("Failed to deserialize the private event: {err:?}."))?;
    let outcome = private_event.validate()?;

    let bytes: SerializedBytes = private_event
        .try_into()
        .map_err(|err| wasm_error!("Failed to serialize private event: {err:?}."))?;
    let entry = PrivateEventEntry(bytes);
    match outcome {
        ValidateCallbackResult::Valid => {
            info!("Received a PrivateEvent from a linked device.");
            create_relaxed(EntryTypes::PrivateEvent(entry))?;
        }
        ValidateCallbackResult::UnresolvedDependencies(unresolved_dependencies) => {
            create_relaxed(EntryTypes::AwaitingDependencies(AwaitingDependencies {
                event: entry,
                unresolved_dependencies,
            }))?;
        }
        ValidateCallbackResult::Invalid(reason) => {
            return Err(wasm_error!("Invalid PrivateEvent: {reason:?}."));
        }
    }
    Ok(())
}

pub fn receive_private_events_from_linked_device<T: PrivateEvent>(
    provenance: AgentPubKey,
    private_event_entries: BTreeMap<EntryHashB64, PrivateEventEntry>,
) -> ExternResult<()> {
    check_is_linked_device(provenance)?;
    let my_private_event_entries = query_private_event_entries(())?;

    for (entry_hash, private_event_entry) in private_event_entries {
        if my_private_event_entries.contains_key(&entry_hash) {
            // We already have this message committed
            continue;
        }
        let bytes = private_event_entry.0.clone();
        let private_event = T::try_from(bytes)
            .map_err(|err| wasm_error!("Failed to deserialize private event: {err:?}."))?;
        let outcome = private_event.validate()?;

        match outcome {
            ValidateCallbackResult::Valid => {
                info!("Received a PrivateEvent from a linked device.");
                create_relaxed(EntryTypes::PrivateEvent(private_event_entry))?;
            }
            ValidateCallbackResult::UnresolvedDependencies(unresolved_dependencies) => {
                create_relaxed(EntryTypes::AwaitingDependencies(AwaitingDependencies {
                    event: private_event_entry,
                    unresolved_dependencies,
                }))?;
            }
            ValidateCallbackResult::Invalid(reason) => {
                return Err(wasm_error!("Invalid PrivateEvent: {reason}."));
            }
        }
    }
    Ok(())
}

pub fn post_commit_private_event<T: PrivateEvent>(private_event: T) -> ExternResult<()> {
    let my_linked_devices = query_my_linked_devices()?;

    let bytes: SerializedBytes = private_event
        .try_into()
        .map_err(|err| wasm_error!("Failed to serialize private event: {err:?}."))?;

    let private_event_entry = PrivateEventEntry(bytes);

    send_remote_signal(
        SerializedBytes::try_from(PrivateEventSourcingRemoteSignal::NewPrivateEvent(
            private_event_entry.clone(),
        ))
        .map_err(|err| wasm_error!(err))?,
        my_linked_devices.clone(),
    )?;

    let bytes = SerializedBytes::try_from(private_event_entry).map_err(|err| wasm_error!(err))?;
    for linked_device in my_linked_devices {
        create_encrypted_message(linked_device, bytes.clone())?;
    }

    Ok(())
}

pub fn query_private_events<T: PrivateEvent>() -> ExternResult<BTreeMap<EntryHashB64, T>> {
    let private_events_entries = query_private_event_entries(())?;

    let private_events = private_events_entries
        .into_iter()
        .filter_map(|(entry_hash, entry)| T::try_from(entry.0).ok().map(|e| (entry_hash, e)))
        .collect();

    Ok(private_events)
}

#[hdk_extern]
pub fn query_private_event_entries() -> ExternResult<BTreeMap<EntryHashB64, PrivateEventEntry>> {
    let filter = ChainQueryFilter::new()
        .entry_type(UnitEntryTypes::PrivateEvent.try_into()?)
        .include_entries(true)
        .action_type(ActionType::Create);
    let records = query(filter)?;
    let private_event_entries = records
        .into_iter()
        .map(|r| {
            let Some(entry_hash) = r.action().entry_hash() else {
                return Err(wasm_error!("PrivateEvents record contained no entry hash."));
            };
            let Some(entry) = r.entry().as_option().clone() else {
                return Err(wasm_error!("PrivateEvents record contained no entry."));
            };
            let entry = PrivateEventEntry::try_from(entry)?;
            Ok((entry_hash.clone().into(), entry))
        })
        .collect::<ExternResult<BTreeMap<EntryHashB64, PrivateEventEntry>>>()?;

    Ok(private_event_entries)
}

pub fn query_private_event_entry(event_hash: EntryHash) -> ExternResult<Option<PrivateEventEntry>> {
    let Some(record) = get(event_hash, GetOptions::local())? else {
        return Ok(None);
    };

    let Some(entry) = record.entry().as_option().clone() else {
        return Err(wasm_error!("PrivateEvents record contained no entry."));
    };
    let entry = PrivateEventEntry::try_from(entry)?;
    Ok(Some(entry))
}

pub fn query_private_event<T: PrivateEvent>(event_hash: EntryHash) -> ExternResult<Option<T>> {
    let Some(private_event_entry) = query_private_event_entry(event_hash)? else {
        return Ok(None);
    };

    let private_event = T::try_from(private_event_entry.0)
        .map_err(|err| wasm_error!("Failed to deserialize private event: {err:?}."))?;
    Ok(Some(private_event))
}
