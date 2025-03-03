use contacts_integrity::*;
use hdk::prelude::*;
use std::collections::BTreeMap;

pub fn query_private_contacts_entries() -> ExternResult<BTreeMap<ActionHash, PrivateContactsEntry>>
{
    let filter = ChainQueryFilter::new()
        .entry_type(UnitEntryTypes::PrivateContactsEntry.try_into()?)
        .include_entries(true)
        .action_type(ActionType::Create);
    let records = query(filter)?;
    let private_contacts_entries = records
        .into_iter()
        .map(|r| {
            let Some(entry) = r.entry().as_option().clone() else {
                return Err(wasm_error!(
                    "PrivateContactsEntry record contained no entry"
                ));
            };
            let entry = PrivateContactsEntry::try_from(entry)?;
            Ok((r.action_address().clone(), entry))
        })
        .collect::<ExternResult<BTreeMap<ActionHash, PrivateContactsEntry>>>()?;

    Ok(private_contacts_entries)
}

pub fn query_undeleted_private_contacts_entries(
) -> ExternResult<BTreeMap<ActionHash, PrivateContactsEntry>> {
    let mut entries = query_private_contacts_entries()?;

    let delete_records = query(ChainQueryFilter::new().action_type(ActionType::Delete))?;

    for delete_record in delete_records {
        let Action::Delete(delete) = delete_record.action() else {
            continue;
        };

        entries.remove(&delete.deletes_address);
    }

    Ok(entries)
}
