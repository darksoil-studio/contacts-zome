use std::collections::BTreeMap;

use contacts_integrity::*;
use hdk::prelude::*;
use linked_devices::query_my_linked_devices;

mod agent_encrypted_message;
mod all_contacts;
mod linked_devices;
mod private_contacts_entry;
mod profile;
mod share_contact_request;
mod synchronize;
mod utils;

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let mut fns: BTreeSet<GrantedFunction> = BTreeSet::new();
    fns.insert((zome_info()?.name, FunctionName::from("recv_remote_signal")));
    let functions = GrantedFunctions::Listed(fns);
    let cap_grant = ZomeCallCapGrant {
        tag: String::from("receive_messages"),
        access: CapAccess::Unrestricted,
        functions,
    };
    create_cap_grant(cap_grant)?;

    schedule("scheduled_synchronize_with_linked_devices")?;

    Ok(InitCallbackResult::Pass)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Signal {
    LinkCreated {
        action: SignedActionHashed,
        link_type: LinkTypes,
    },
    LinkDeleted {
        action: SignedActionHashed,
        create_link_action: SignedActionHashed,
        link_type: LinkTypes,
    },
    EntryCreated {
        action: SignedActionHashed,
        app_entry: EntryTypes,
    },
    EntryUpdated {
        action: SignedActionHashed,
        app_entry: EntryTypes,
        original_app_entry: EntryTypes,
    },
    EntryDeleted {
        action: SignedActionHashed,
        original_app_entry: EntryTypes,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ContactsRemoteSignal {
    NewPrivateMessengerEntry(PrivateMessengerEntry),
    PeerChatTypingIndicator { peer_chat_hash: EntryHash },
    GroupChatTypingIndicator { group_chat_hash: EntryHash },
    SynchronizeEntriesWithLinkedDevice(BTreeMap<EntryHashB64, PrivateMessengerEntry>),
    SynchronizeGroupEntriesWithNewGroupMember(BTreeMap<EntryHashB64, PrivateMessengerEntry>),
}

#[hdk_extern]
pub fn recv_remote_signal(signal: MessengerRemoteSignal) -> ExternResult<()> {
    // TODO: take into account whether the recipient is blocked
    match signal {
        MessengerRemoteSignal::NewPrivateMessengerEntry(private_messenger_entry) => {
            receive_private_messenger_entry(private_messenger_entry)
        }
        MessengerRemoteSignal::SynchronizeEntriesWithLinkedDevice(entries) => {
            let call_info = call_info()?;
            let my_devices = query_my_linked_devices()?;

            if !my_devices.contains(&call_info.provenance) {
                return Err(wasm_error!("Agent {} tried to synchronize its entries with us but we have not linked devices", call_info.provenance));
            }

            receive_private_messenger_entries(entries)
        }
        MessengerRemoteSignal::SynchronizeGroupEntriesWithNewGroupMember(entries) => {
            receive_private_messenger_entries(entries)
        }
        MessengerRemoteSignal::PeerChatTypingIndicator { peer_chat_hash } => {
            let call_info = call_info()?;

            let peer = call_info.provenance;

            emit_signal(Signal::PeerChatTypingIndicator {
                peer_chat_hash,
                peer,
            })?;
            Ok(())
        }
        MessengerRemoteSignal::GroupChatTypingIndicator { group_chat_hash } => {
            let call_info = call_info()?;

            let peer = call_info.provenance;

            emit_signal(Signal::GroupChatTypingIndicator {
                peer,
                group_chat_hash,
            })?;
            Ok(())
        }
    }
}

#[hdk_extern(infallible)]
pub fn post_commit(committed_actions: Vec<SignedActionHashed>) {
    for action in committed_actions {
        if let Err(err) = signal_action(action) {
            error!("Error signaling new action: {:?}", err);
        }
    }
}
fn signal_action(action: SignedActionHashed) -> ExternResult<()> {
    match action.hashed.content.clone() {
        Action::CreateLink(create_link) => {
            if let Ok(Some(link_type)) =
                LinkTypes::from_type(create_link.zome_index, create_link.link_type)
            {
                emit_signal(Signal::LinkCreated {
                    action: action.clone(),
                    link_type,
                })?;
            }
            Ok(())
        }
        Action::DeleteLink(delete_link) => {
            let record = get(delete_link.link_add_address.clone(), GetOptions::default())?.ok_or(
                wasm_error!(WasmErrorInner::Guest(
                    "Failed to fetch CreateLink action".to_string()
                )),
            )?;
            match record.action() {
                Action::CreateLink(create_link) => {
                    if let Ok(Some(link_type)) =
                        LinkTypes::from_type(create_link.zome_index, create_link.link_type)
                    {
                        emit_signal(Signal::LinkDeleted {
                            action,
                            link_type,
                            create_link_action: record.signed_action.clone(),
                        })?;
                    }
                    Ok(())
                }
                _ => Err(wasm_error!(WasmErrorInner::Guest(
                    "Create Link should exist".to_string()
                ))),
            }
        }
        Action::Create(_create) => {
            if let Ok(Some(app_entry)) = get_entry_for_action(&action.hashed.hash) {
                emit_signal(Signal::EntryCreated {
                    action: action.clone(),
                    app_entry: app_entry.clone(),
                })?;
                match app_entry {
                    EntryTypes::PrivateContactsEntry(entry) => {
                        // TODO: update my other devices and all my contacts, if necessary
                    }
                    _ => {}
                };
            }
            Ok(())
        }
        Action::Update(update) => {
            if let Ok(Some(app_entry)) = get_entry_for_action(&action.hashed.hash) {
                if let Ok(Some(original_app_entry)) =
                    get_entry_for_action(&update.original_action_address)
                {
                    emit_signal(Signal::EntryUpdated {
                        action,
                        app_entry,
                        original_app_entry,
                    })?;
                }
            }
            Ok(())
        }
        Action::Delete(delete) => {
            if let Ok(Some(original_app_entry)) = get_entry_for_action(&delete.deletes_address) {
                emit_signal(Signal::EntryDeleted {
                    action,
                    original_app_entry,
                })?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn get_entry_for_action(action_hash: &ActionHash) -> ExternResult<Option<EntryTypes>> {
    let record = match get_details(action_hash.clone(), GetOptions::default())? {
        Some(Details::Record(record_details)) => record_details.record,
        _ => {
            return Ok(None);
        }
    };
    let entry = match record.entry().as_option() {
        Some(entry) => entry,
        None => {
            return Ok(None);
        }
    };
    let (zome_index, entry_index) = match record.action().entry_type() {
        Some(EntryType::App(AppEntryDef {
            zome_index,
            entry_index,
            ..
        })) => (zome_index, entry_index),
        _ => {
            return Ok(None);
        }
    };
    EntryTypes::deserialize_from_type(*zome_index, *entry_index, entry)
}
