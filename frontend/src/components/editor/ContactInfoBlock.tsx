import type { Block, ContactInfoData } from '../../lib/editor/types';

interface ContactInfoBlockProps {
  block: Block;
  onChange: (block: Block) => void;
  locked?: boolean;
}

export function ContactInfoBlock({ block }: ContactInfoBlockProps) {
  const data = block.data as ContactInfoData;
  const hasContact = data.contactId && data.lines.length > 0;

  if (!hasContact) {
    return (
      <div className="border-dashed border border-gray-300 rounded p-4 text-gray-400 text-sm">
        Select a client
      </div>
    );
  }

  return (
    <div className="py-1">
      {data.lines.map((line, i) => (
        <div
          key={i}
          className={
            i === 0
              ? 'text-sm text-gray-900 font-bold'
              : 'text-sm text-gray-700'
          }
        >
          {line}
        </div>
      ))}
    </div>
  );
}
