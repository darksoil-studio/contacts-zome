use hdk::prelude::*;

#[hdk_extern(infallible)]
fn scheduled_synchronize_with_linked_devices(_: Option<Schedule>) -> Option<Schedule> {
    // if let Err(err) = commit_my_pending_encrypted_messages(()) {
    //     error!("Failed to commit my encrypted messages: {err:?}");
    // }
    // if let Err(err) = synchronize_with_linked_devices(()) {
    //     error!("Failed to synchronize with other agents: {err:?}");
    // }

    Some(Schedule::Persisted("*/30 * * * * * *".into())) // Every 30 seconds
}

// #[hdk_extern]
// pub fn synchronize_with_linked_devices() -> ExternResult<()> {
//     let my_pub_key = agent_info()?.agent_latest_pubkey;

//     let agents = query_my_linked_devices()?;
//     let other_agents = agents.into_iter().filter(|a| a.ne(&my_pub_key));

//     let private_messenger_entries = query_private_messenger_entries(())?;

//     let private_messenger_entries_unit_entry_types: EntryType =
//         UnitEntryTypes::PrivateMessengerEntry.try_into()?;

//     for agent in other_agents {
//         let private_messenger_entries_agent_activity = get_agent_activity(
//             agent.clone(),
//             ChainQueryFilter::new()
//                 .entry_type(private_messenger_entries_unit_entry_types.clone())
//                 .clone(),
//             ActivityRequest::Full,
//         )?;

//         let actions_get_inputs = private_messenger_entries_agent_activity
//             .valid_activity
//             .into_iter()
//             .map(|(_, action_hash)| GetInput::new(action_hash.into(), GetOptions::network()))
//             .collect();

//         let records = HDK.with(|hdk| hdk.borrow().get(actions_get_inputs))?;
//         let existing_private_messenger_entries_hashes: HashSet<EntryHash> = records
//             .into_iter()
//             .filter_map(|r| r)
//             .filter_map(|r| r.action().entry_hash().cloned())
//             .collect();

//         let missing_private_entry_hashes: Vec<EntryHashB64> = private_messenger_entries
//             .entry_hashes
//             .clone()
//             .into_iter()
//             .filter(|entry_hash| {
//                 !existing_private_messenger_entries_hashes.contains(&entry_hash.clone().into())
//             })
//             .collect();

//         let mut missing_private_messenger_entries: Vec<(EntryHashB64, PrivateMessengerEntry)> =
//             Vec::new();

//         for entry_hash in missing_private_entry_hashes {
//             let Some(entry) = private_messenger_entries.entries.get(&entry_hash) else {
//                 return Err(wasm_error!("Unreachable: QueriedPrivateMessengerEntries entries did not contain one of its entry hashes."));
//             };
//             missing_private_messenger_entries.push((entry_hash.clone(), entry.clone()));
//         }

//         let mut offline = false;

//         for (_, private_messenger_entry) in missing_private_messenger_entries {
//             if !offline {
//                 let response = call_remote(
//                     agent.clone(),
//                     zome_info()?.name,
//                     "receive_private_messenger_entry".into(),
//                     None,
//                     private_messenger_entry.clone(),
//                 )?;

//                 match response {
//                     ZomeCallResponse::Ok(_) => continue,
//                     _ => offline = true,
//                 }
//             }
//             create_encrypted_message(
//                 agent.clone(),
//                 MessengerEncryptedMessage(private_messenger_entry.clone()),
//             )?;
//         }
//     }

//     Ok(())
// }

// #[hdk_extern]
// pub fn synchronize_with_linked_device(linked_device: AgentPubKey) -> ExternResult<()> {
//     let entries = query_private_messenger_entries(())?;

//     send_remote_signal(
//         MessengerRemoteSignal::SynchronizeEntriesWithLinkedDevice(entries),
//         vec![linked_device],
//     )?;

//     Ok(())
// }
