import type { Block, DocMetaData } from '../../lib/editor/types';

interface DocMetaBlockProps {
  block: Block;
  onChange: (block: Block) => void;
  locked?: boolean;
}

function formatDate(str: string): string {
  if (!str) return '';
  const d = new Date(str);
  if (isNaN(d.getTime())) return str;
  return d.toLocaleDateString('en-GB', {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
  });
}

export function DocMetaBlock({ block }: DocMetaBlockProps) {
  const data = block.data as DocMetaData;

  const fields: { label: string; value: string }[] = [
    { label: 'Document', value: data.docNumber },
    { label: 'Date', value: formatDate(data.docDate) },
    { label: 'Valid Until', value: formatDate(data.validUntil) },
    { label: 'Project', value: data.projectName },
  ];

  return (
    <div className="grid grid-cols-2 gap-x-8 gap-y-1 text-sm py-1">
      {fields.map((f) => (
        <div key={f.label} className="flex gap-2">
          <span className="text-gray-500">{f.label}:</span>
          <span className={f.value ? 'text-gray-900' : 'text-gray-300'}>
            {f.value || '\u2014'}
          </span>
        </div>
      ))}
    </div>
  );
}
