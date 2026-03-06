import type {
  Block,
  SignatureBlockData,
  SignatureParty,
} from '../../lib/editor/types';

interface SignatureBlockProps {
  block: Block;
  onChange: (block: Block) => void;
  locked?: boolean;
}

export function SignatureBlock({ block, onChange, locked }: SignatureBlockProps) {
  const data = block.data as SignatureBlockData;

  function updateParty(
    index: number,
    field: keyof SignatureParty,
    value: string,
  ) {
    const newParties = data.parties.map((p, i) =>
      i === index ? { ...p, [field]: value } : p,
    );
    onChange({ ...block, data: { ...data, parties: newParties } });
  }

  function updateLine(
    partyIdx: number,
    lineIdx: number,
    field: 'label' | 'value',
    val: string,
  ) {
    const newParties = data.parties.map((p, pi) => {
      if (pi !== partyIdx) return p;
      return {
        ...p,
        lines: p.lines.map((l, li) =>
          li === lineIdx ? { ...l, [field]: val } : l,
        ),
      };
    });
    onChange({ ...block, data: { ...data, parties: newParties } });
  }

  return (
    <div className="my-6 pt-8">
      <h3 className="text-base font-semibold mb-6">Agreed and Accepted</h3>
      <div className="grid grid-cols-2 gap-12">
        {data.parties.map((party, pi) => (
          <div key={pi} className="space-y-4">
            {/* Company and role */}
            <div className="font-semibold">
              {locked ? (
                <span>
                  {party.company} ("{party.role}")
                </span>
              ) : (
                <div className="space-y-1">
                  <input
                    value={party.company}
                    onChange={(e) =>
                      updateParty(pi, 'company', e.target.value)
                    }
                    className="w-full bg-transparent outline-none font-semibold"
                    placeholder="Company name"
                  />
                  <input
                    value={party.role}
                    onChange={(e) => updateParty(pi, 'role', e.target.value)}
                    className="w-full bg-transparent outline-none text-sm text-gray-600"
                    placeholder="Role (e.g. Service Provider)"
                  />
                </div>
              )}
            </div>

            {/* Location and date */}
            <div className="flex gap-2 text-sm">
              {locked ? (
                <span>
                  {party.location}
                  {party.location && party.date ? ', ' : ''}
                  {party.date}
                </span>
              ) : (
                <>
                  <input
                    value={party.location}
                    onChange={(e) =>
                      updateParty(pi, 'location', e.target.value)
                    }
                    className="bg-transparent outline-none border-b border-gray-300 flex-1"
                    placeholder="Location"
                  />
                  <input
                    value={party.date}
                    onChange={(e) => updateParty(pi, 'date', e.target.value)}
                    className="bg-transparent outline-none border-b border-gray-300 flex-1"
                    placeholder="Date"
                  />
                </>
              )}
            </div>

            {/* Signature lines */}
            {party.lines.map((line, li) => (
              <div key={li} className="pt-6">
                <div
                  className="border-b border-gray-900 mb-1"
                  style={{ minHeight: 1 }}
                />
                <div className="flex items-center gap-2 text-sm">
                  {locked ? (
                    <span>
                      {line.label} {line.value}
                    </span>
                  ) : (
                    <>
                      <input
                        value={line.label}
                        onChange={(e) =>
                          updateLine(pi, li, 'label', e.target.value)
                        }
                        className="bg-transparent outline-none w-16 text-gray-600"
                        placeholder="Label:"
                      />
                      <input
                        value={line.value}
                        onChange={(e) =>
                          updateLine(pi, li, 'value', e.target.value)
                        }
                        className="bg-transparent outline-none flex-1"
                        placeholder="Name"
                      />
                    </>
                  )}
                </div>
              </div>
            ))}
          </div>
        ))}
      </div>
    </div>
  );
}
