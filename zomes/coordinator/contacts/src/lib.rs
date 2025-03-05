use hdk::prelude::*;
use private_event::ContactsEvent;
use private_event_sourcing::*;

mod all_contacts;
mod private_event;
mod profile;
mod share_contact_request;
mod synchronize;

#[hdk_extern]
pub fn recv_remote_signal(signal_bytes: SerializedBytes) -> ExternResult<()> {
    if let Ok(private_event_sourcing_remote_signal) =
        PrivateEventSourcingRemoteSignal::try_from(signal_bytes)
    {
        recv_private_events_remote_signal::<ContactsEvent>(private_event_sourcing_remote_signal)
    } else {
        Ok(())
    }
}
