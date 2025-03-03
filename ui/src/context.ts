import { createContext } from '@lit/context';
import { ContactsStore } from './contacts-store.js';

export const contactsStoreContext = createContext<ContactsStore>(
  'contacts/store'
);

