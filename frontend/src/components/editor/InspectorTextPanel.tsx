import type { Block, BlockMeta } from '../../lib/editor/types';
import { InspectorCommon, Field } from './InspectorCommon';

const LINE_HEIGHT_OPTIONS = [1.0, 1.15, 1.5, 1.75, 2.0];

const FONT_OPTIONS: { value: BlockMeta['font']; label: string }[] = [
  { value: 'system', label: 'System (Sans)' },
  { value: 'serif', label: 'Serif' },
  { value: 'mono', label: 'Monospace' },
];

interface InspectorTextPanelProps {
  block: Block;
  onChange: (block: Block) => void;
  templateMode?: boolean;
}

export function InspectorTextPanel({
  block,
  onChange,
  templateMode,
}: InspectorTextPanelProps) {
  const { meta } = block;

  function updateMeta(updates: Partial<BlockMeta>) {
    onChange({ ...block, meta: { ...block.meta, ...updates } });
  }

  return (
    <div className="space-y-4">
      <Field label="Type">
        <span className="text-sm font-mono capitalize">{block.type}</span>
      </Field>

      <Field label="Font Size (pt)">
        <input
          type="number"
          value={meta.fontSize ?? ''}
          placeholder="Default"
          onChange={(e) =>
            updateMeta({ fontSize: e.target.value ? Number(e.target.value) : null })
          }
          className="w-full border rounded px-2 py-1 text-sm"
        />
      </Field>

      <Field label="Line Height">
        <select
          value={meta.lineHeight}
          onChange={(e) => updateMeta({ lineHeight: Number(e.target.value) })}
          className="w-full border rounded px-2 py-1 text-sm"
        >
          {LINE_HEIGHT_OPTIONS.map((lh) => (
            <option key={lh} value={lh}>{lh}</option>
          ))}
        </select>
      </Field>

      <Field label="Font">
        <select
          value={meta.font}
          onChange={(e) => updateMeta({ font: e.target.value as BlockMeta['font'] })}
          className="w-full border rounded px-2 py-1 text-sm"
        >
          {FONT_OPTIONS.map((opt) => (
            <option key={opt.value} value={opt.value}>{opt.label}</option>
          ))}
        </select>
      </Field>

      <InspectorCommon block={block} onChange={onChange} templateMode={templateMode} />
    </div>
  );
}
