use contacts_integrity::{Contact, EncryptedMessage, EntryTypes, LinkTypes, Profile};
use hdk::prelude::*;

use crate::{
    utils::{create_link_relaxed, create_relaxed},
    ContactsRemoteSignal,
};

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub struct EncryptedMessageBytes(XSalsa20Poly1305EncryptedData);

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub enum MessageBetweenUsers {
    SendShareContactRequest { my_contact: Contact },
    AcceptShareContactRequest { my_contact: Contact },
    RejectShareContactRequest,
    CancelShareContactRequest,
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub enum MessageBetweenLinkedDevices {
    NewContactAdded { contact: Contact },
    ProfileUpdated { profile: Profile },
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub enum ContactsEncryptedMessage {
    MessageBetweenUsers(MessageBetweenUsers),
    MessageBetweenLinkedDevices(MessageBetweenLinkedDevices),
}

fn create_encrypted_message(
    recipient: AgentPubKey,
    message: ContactsEncryptedMessage,
) -> ExternResult<()> {
    send_remote_signal(
        ContactsRemoteSignal::NewPrivateContactsEntry(message.clone()),
        agents,
    )?;

    let message_bytes = SerializedBytes::try_from(message).map_err(|err| wasm_error!(err))?;
    let encrypted_data = ed_25519_x_salsa20_poly1305_encrypt(
        agent_info()?.agent_latest_pubkey,
        recipient.clone(),
        message_bytes.bytes().clone().into(),
    )?;

    let bytes = SerializedBytes::try_from(EncryptedMessageBytes(encrypted_data))
        .map_err(|err| wasm_error!(err))?;

    if bytes.bytes().len() > 900 {
        let entry = EncryptedMessage {
            recipient: recipient.clone(),
            content: bytes,
        };
        let entry_hash = hash_entry(&entry)?;
        create_relaxed(EntryTypes::EncryptedMessage(entry))?;
        create_link_relaxed(
            recipient.clone(),
            entry_hash,
            LinkTypes::AgentEncryptedMessage,
            (),
        )?;
    } else {
        create_link_relaxed(
            recipient.clone(),
            recipient,
            LinkTypes::AgentEncryptedMessage,
            bytes.bytes().clone(),
        )?;
    }

    Ok(())
}

fn get_message(agent_encrypted_message_link: &Link) -> ExternResult<Option<SerializedBytes>> {
    if agent_encrypted_message_link
        .base
        .eq(&agent_encrypted_message_link.target)
    {
        let tag = agent_encrypted_message_link.tag.clone();
        let bytes = SerializedBytes::from(UnsafeBytes::from(tag.into_inner()));
        Ok(Some(bytes))
    } else {
        let Some(entry_hash) = agent_encrypted_message_link
            .target
            .clone()
            .into_entry_hash()
        else {
            return Err(wasm_error!(
                "AgentToEncryptedMessage link does not have an entry hash as its target"
            ));
        };
        let Some(record) = get(entry_hash, GetOptions::default())? else {
            return Ok(None);
        };

        let Ok(Some(encrypted_message)) = record.entry().to_app_option::<EncryptedMessage>() else {
            return Err(wasm_error!("Invalid EncryptedMessage target"));
        };
        Ok(Some(encrypted_message.content))
    }
}

// #[hdk_extern]
// pub fn commit_my_pending_encrypted_messages() -> ExternResult<()> {
//     let my_pub_key = agent_info()?.agent_latest_pubkey;
//     let links = get_links(
//         GetLinksInputBuilder::try_new(my_pub_key.clone(), LinkTypes::AgentEncryptedMessage)?
//             .build(),
//     )?;

//     let private_messenger_entries = query_private_messenger_entries(())?;

//     for link in links {
//         let Some(bytes) = get_message(&link)? else {
//             continue;
//         };
//         let encrypted_data =
//             EncryptedMessageBytes::try_from(bytes).map_err(|err| wasm_error!(err))?;

//         let decrypted_data = ed_25519_x_salsa20_poly1305_decrypt(
//             my_pub_key.clone(),
//             link.author.clone(),
//             encrypted_data.0,
//         )?;

//         let decrypted_bytes = decrypted_data.as_ref().to_vec();
//         let decrypted_serialized_bytes = SerializedBytes::from(UnsafeBytes::from(decrypted_bytes));

//         let message = MessengerEncryptedMessage::try_from(decrypted_serialized_bytes)
//             .map_err(|err| wasm_error!(err))?;

//         let private_messenger_entry = message.0;

//         let private_messager_entry_hash = hash_entry(&private_messenger_entry)?;
//         if private_messenger_entries.contains_key(&private_messager_entry_hash.into()) {
//             delete_link_relaxed(link.create_link_hash)?;
//             continue;
//         }

//         let valid = validate_private_messenger_entry(&private_messenger_entry)?;

//         match valid {
//             ValidateCallbackResult::Valid => {
//                 create_relaxed(EntryTypes::PrivateMessengerEntry(
//                     private_messenger_entry.clone(),
//                 ))?;
//                 post_receive_entry(private_messenger_entry)?;
//                 delete_link_relaxed(link.create_link_hash)?;
//             }
//             ValidateCallbackResult::Invalid(invalid_reason) => {
//                 error!("Invalid PrivateMessengerEntry: {invalid_reason}");
//                 delete_link_relaxed(link.create_link_hash)?;
//             }
//             ValidateCallbackResult::UnresolvedDependencies(_deps) => {
//                 create_relaxed(EntryTypes::AwaitingDepsEntry(
//                     private_messenger_entry.clone(),
//                 ))?;

//                 delete_link_relaxed(link.create_link_hash)?;
//             }
//         }
//     }

//     Ok(())
// }
