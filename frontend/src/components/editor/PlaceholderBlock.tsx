import type { Block, PlaceholderData } from '../../lib/editor/types';

interface PlaceholderBlockProps {
  block: Block;
  onChange: (b: Block) => void;
}

export function PlaceholderBlock({ block, onChange }: PlaceholderBlockProps) {
  const data = block.data as PlaceholderData;

  if (data.resolved) {
    return (
      <div className="inline-block text-base">
        {data.resolved}
      </div>
    );
  }

  return (
    <div className="inline-flex items-center gap-1 bg-amber-100 text-amber-800 rounded px-2 py-0.5 text-sm font-mono">
      <span>{'{'}{'{'}</span>
      <input
        value={data.variable}
        onChange={(e) =>
          onChange({
            ...block,
            data: { ...data, variable: e.target.value },
          })
        }
        className="bg-transparent outline-none w-32"
      />
      <span>{'}'}{'}'}</span>
    </div>
  );
}
