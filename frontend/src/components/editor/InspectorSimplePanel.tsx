import type { Block, SpacerData, PlaceholderData } from '../../lib/editor/types';
import { InspectorCommon, Field } from './InspectorCommon';

interface InspectorSimplePanelProps {
  block: Block;
  onChange: (block: Block) => void;
  templateMode?: boolean;
}

export function InspectorSimplePanel({
  block,
  onChange,
  templateMode,
}: InspectorSimplePanelProps) {
  return (
    <div className="space-y-4">
      <Field label="Type">
        <span className="text-sm font-mono capitalize">{block.type}</span>
      </Field>

      {block.type === 'spacer' && (
        <SpacerControls block={block} onChange={onChange} />
      )}

      {block.type === 'placeholder' && (
        <PlaceholderControls block={block} onChange={onChange} />
      )}

      {block.type === 'divider' && (
        <p className="text-xs text-gray-500">Horizontal divider line</p>
      )}

      <InspectorCommon block={block} onChange={onChange} templateMode={templateMode} />
    </div>
  );
}

function SpacerControls({
  block,
  onChange,
}: {
  block: Block;
  onChange: (block: Block) => void;
}) {
  const data = block.data as SpacerData;

  return (
    <Field label="Height (px)">
      <input
        type="number"
        value={data.height}
        min={1}
        onChange={(e) =>
          onChange({
            ...block,
            data: { ...data, height: Number(e.target.value) } as SpacerData,
          })
        }
        className="w-full border rounded px-2 py-1 text-sm"
      />
    </Field>
  );
}

function PlaceholderControls({
  block,
  onChange,
}: {
  block: Block;
  onChange: (block: Block) => void;
}) {
  const data = block.data as PlaceholderData;

  return (
    <Field label="Variable Name">
      <input
        type="text"
        value={data.variable}
        onChange={(e) =>
          onChange({
            ...block,
            data: { ...data, variable: e.target.value } as PlaceholderData,
          })
        }
        placeholder="variable_name"
        className="w-full border rounded px-2 py-1 text-sm font-mono"
      />
    </Field>
  );
}
