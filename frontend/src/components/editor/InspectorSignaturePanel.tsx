import type { Block, SignatureBlockData, SignatureParty } from '../../lib/editor/types';
import { InspectorCommon, Field } from './InspectorCommon';
import { Plus, Trash2 } from 'lucide-react';

interface InspectorSignaturePanelProps {
  block: Block;
  onChange: (block: Block) => void;
  templateMode?: boolean;
}

function emptyParty(): SignatureParty {
  return {
    role: '',
    company: '',
    location: '',
    date: '',
    lines: [
      { label: 'Name:', value: '' },
      { label: 'Title:', value: '' },
    ],
  };
}

export function InspectorSignaturePanel({
  block,
  onChange,
  templateMode,
}: InspectorSignaturePanelProps) {
  const data = block.data as SignatureBlockData;

  function updateParties(parties: SignatureParty[]) {
    onChange({
      ...block,
      data: { ...data, parties } as SignatureBlockData,
    });
  }

  function updateParty(index: number, updates: Partial<SignatureParty>) {
    const next = [...data.parties];
    next[index] = { ...next[index], ...updates };
    updateParties(next);
  }

  function addParty() {
    updateParties([...data.parties, emptyParty()]);
  }

  function removeParty(index: number) {
    if (data.parties.length <= 1) return;
    updateParties(data.parties.filter((_, i) => i !== index));
  }

  function updateLine(
    partyIndex: number,
    lineIndex: number,
    field: 'label' | 'value',
    value: string,
  ) {
    const next = [...data.parties];
    const lines = [...next[partyIndex].lines];
    lines[lineIndex] = { ...lines[lineIndex], [field]: value };
    next[partyIndex] = { ...next[partyIndex], lines };
    updateParties(next);
  }

  function addLine(partyIndex: number) {
    const next = [...data.parties];
    const lines = [...next[partyIndex].lines, { label: '', value: '' }];
    next[partyIndex] = { ...next[partyIndex], lines };
    updateParties(next);
  }

  function removeLine(partyIndex: number, lineIndex: number) {
    const party = data.parties[partyIndex];
    if (party.lines.length <= 1) return;
    const next = [...data.parties];
    next[partyIndex] = {
      ...next[partyIndex],
      lines: party.lines.filter((_, i) => i !== lineIndex),
    };
    updateParties(next);
  }

  return (
    <div className="space-y-4">
      <Field label="Type">
        <span className="text-sm font-mono">signature</span>
      </Field>

      <div className="flex items-center justify-between">
        <span className="text-xs text-gray-500">Parties ({data.parties.length})</span>
        <button
          onClick={addParty}
          className="flex items-center gap-1 px-2 py-0.5 rounded border text-xs hover:bg-gray-100"
        >
          <Plus className="h-3 w-3" /> Add
        </button>
      </div>

      {data.parties.map((party, pi) => (
        <div key={pi} className="rounded border p-2 space-y-2">
          <div className="flex items-center justify-between">
            <span className="text-xs font-medium text-gray-600">Party {pi + 1}</span>
            {data.parties.length > 1 && (
              <button
                onClick={() => removeParty(pi)}
                className="text-gray-400 hover:text-red-500"
              >
                <Trash2 className="h-3 w-3" />
              </button>
            )}
          </div>

          <input
            type="text"
            value={party.company}
            onChange={(e) => updateParty(pi, { company: e.target.value })}
            placeholder="Company"
            className="w-full border rounded px-2 py-1 text-xs"
          />
          <input
            type="text"
            value={party.role}
            onChange={(e) => updateParty(pi, { role: e.target.value })}
            placeholder="Role"
            className="w-full border rounded px-2 py-1 text-xs"
          />

          <div className="space-y-1">
            <div className="flex items-center justify-between">
              <span className="text-[10px] text-gray-400">Signature Lines</span>
              <button
                onClick={() => addLine(pi)}
                className="text-[10px] text-blue-500 hover:underline"
              >
                + Line
              </button>
            </div>
            {party.lines.map((line, li) => (
              <div key={li} className="flex gap-1 items-center">
                <input
                  type="text"
                  value={line.label}
                  onChange={(e) => updateLine(pi, li, 'label', e.target.value)}
                  placeholder="Label"
                  className="w-1/3 border rounded px-1 py-0.5 text-xs"
                />
                <input
                  type="text"
                  value={line.value}
                  onChange={(e) => updateLine(pi, li, 'value', e.target.value)}
                  placeholder="Value"
                  className="flex-1 border rounded px-1 py-0.5 text-xs"
                />
                {party.lines.length > 1 && (
                  <button
                    onClick={() => removeLine(pi, li)}
                    className="text-gray-400 hover:text-red-500"
                  >
                    <Trash2 className="h-3 w-3" />
                  </button>
                )}
              </div>
            ))}
          </div>
        </div>
      ))}

      <InspectorCommon block={block} onChange={onChange} templateMode={templateMode} />
    </div>
  );
}
