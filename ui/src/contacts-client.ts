import { 
  SignedActionHashed,
  CreateLink,
  Link,
  DeleteLink,
  Delete,
  AppClient, 
  Record, 
  ActionHash, 
  EntryHash, 
  AgentPubKey,
} from '@holochain/client';
import { EntryRecord, ZomeClient } from '@tnesh-stack/utils';

import { ContactsSignal } from './types.js';

export class ContactsClient extends ZomeClient<ContactsSignal> {

  constructor(public client: AppClient, public roleName: string, public zomeName = 'contacts') {
    super(client, roleName, zomeName);
  }
}
