import type { Block, ImageBlockData } from '../../lib/editor/types';
import { InspectorCommon, Field } from './InspectorCommon';

interface InspectorImagePanelProps {
  block: Block;
  onChange: (block: Block) => void;
  templateMode?: boolean;
}

export function InspectorImagePanel({
  block,
  onChange,
  templateMode,
}: InspectorImagePanelProps) {
  const data = block.data as ImageBlockData;

  function updateData(updates: Partial<ImageBlockData>) {
    onChange({
      ...block,
      data: { ...data, ...updates } as ImageBlockData,
    });
  }

  return (
    <div className="space-y-4">
      <Field label="Type">
        <span className="text-sm font-mono">image</span>
      </Field>

      {data.src && (
        <Field label="Preview">
          <div className="rounded border bg-gray-50 p-1">
            <img
              src={data.src}
              alt={data.alt || 'Preview'}
              className="max-h-32 w-full object-contain"
            />
          </div>
        </Field>
      )}

      {!data.src && (
        <p className="text-xs text-gray-400">No image source set</p>
      )}

      <Field label="Width (px)">
        <input
          type="number"
          value={data.width || ''}
          onChange={(e) => updateData({ width: Number(e.target.value) })}
          className="w-full border rounded px-2 py-1 text-sm"
        />
      </Field>

      <Field label="Height (px)">
        <input
          type="number"
          value={data.height || ''}
          onChange={(e) => updateData({ height: Number(e.target.value) })}
          className="w-full border rounded px-2 py-1 text-sm"
        />
      </Field>

      <Field label="Alt Text">
        <input
          type="text"
          value={data.alt || ''}
          onChange={(e) => updateData({ alt: e.target.value })}
          className="w-full border rounded px-2 py-1 text-sm"
        />
      </Field>

      <InspectorCommon block={block} onChange={onChange} templateMode={templateMode} />
    </div>
  );
}
