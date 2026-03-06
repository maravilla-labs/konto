import type { Block, ContactInfoData } from '../../lib/editor/types';
import { InspectorCommon, Field } from './InspectorCommon';

interface Contact {
  id: string;
  name: string;
  company?: string;
  address?: string;
  city?: string;
  postal_code?: string;
  country: string;
}

interface InspectorContactPanelProps {
  block: Block;
  onChange: (block: Block) => void;
  templateMode?: boolean;
  contacts: Contact[];
}

function buildContactLines(contact: Contact): string[] {
  const lines: string[] = [];
  if (contact.company) lines.push(contact.company);
  lines.push(contact.name);
  if (contact.address) lines.push(contact.address);
  if (contact.postal_code || contact.city) {
    const parts = [contact.postal_code, contact.city].filter(Boolean);
    lines.push(parts.join(' '));
  }
  if (contact.country && contact.country !== 'CH') {
    lines.push(contact.country);
  }
  return lines;
}

export function InspectorContactPanel({
  block,
  onChange,
  templateMode,
  contacts,
}: InspectorContactPanelProps) {
  const data = block.data as ContactInfoData;

  function handleContactChange(contactId: string) {
    if (!contactId) {
      onChange({
        ...block,
        data: { contactId: '', lines: [] } as ContactInfoData,
      });
      return;
    }
    const contact = contacts.find((c) => c.id === contactId);
    if (!contact) return;
    const lines = buildContactLines(contact);
    onChange({
      ...block,
      data: { ...data, contactId, lines } as ContactInfoData,
    });
  }

  const selectedContact = contacts.find((c) => c.id === data.contactId);

  return (
    <div className="space-y-4">
      <Field label="Type">
        <span className="text-sm font-mono">contact_info</span>
      </Field>

      <Field label="Contact">
        <select
          value={data.contactId}
          onChange={(e) => handleContactChange(e.target.value)}
          className="w-full border rounded px-2 py-1 text-sm"
        >
          <option value="">Select contact...</option>
          {contacts.map((c) => (
            <option key={c.id} value={c.id}>
              {c.name}{c.company ? ` (${c.company})` : ''}
            </option>
          ))}
        </select>
      </Field>

      {selectedContact && data.lines.length > 0 && (
        <Field label="Preview">
          <div className="rounded border bg-gray-50 p-2 text-xs space-y-0.5">
            {data.lines.map((line, i) => (
              <div key={i} className="text-gray-700">{line}</div>
            ))}
          </div>
        </Field>
      )}

      {!selectedContact && data.contactId && (
        <p className="text-xs text-amber-600">
          Contact not found. It may have been deleted.
        </p>
      )}

      <InspectorCommon block={block} onChange={onChange} templateMode={templateMode} />
    </div>
  );
}
