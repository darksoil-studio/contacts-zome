import { css, html, LitElement } from 'lit';
import { provide, consume } from '@lit/context';
import { customElement, property } from 'lit/decorators.js';
import { AppClient } from '@holochain/client';
import { appClientContext } from '@tnesh-stack/elements';

import { contactsStoreContext } from '../context.js';
import { ContactsStore } from '../contacts-store.js';
import { ContactsClient } from '../contacts-client.js';

/**
 * @element contacts-context
 */
@customElement('contacts-context')
export class ContactsContext extends LitElement {
  @consume({ context: appClientContext })
  private client!: AppClient;

  @provide({ context: contactsStoreContext })
  @property({ type: Object })
  store!: ContactsStore;

  @property()
  role!: string;

  @property()
  zome = 'contacts';

  connectedCallback() {
    super.connectedCallback();
    if (this.store) return;
    if (!this.role) {
      throw new Error(`<contacts-context> must have a role="YOUR_DNA_ROLE" property, eg: <contacts-context role="role1">`);
    }
    if (!this.client) {
      throw new Error(`<contacts-context> must either:
        a) be placed inside <app-client-context>
          or 
        b) receive an AppClient property (eg. <contacts-context .client=\${client}>) 
          or 
        c) receive a store property (eg. <contacts-context .store=\${store}>)
      `);
    }

    this.store = new ContactsStore(new ContactsClient(this.client, this.role, this.zome));
  }
  
  render() {
    return html`<slot></slot>`;
  }

  static styles = css`
    :host {
      display: contents;
    }
  `;
}

