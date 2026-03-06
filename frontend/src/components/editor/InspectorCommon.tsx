import type { Block, BlockMeta } from '../../lib/editor/types';

const ALIGNMENT_OPTIONS: { value: BlockMeta['align']; icon: string }[] = [
  { value: 'left', icon: '\u2590\u2591\u2591' },
  { value: 'center', icon: '\u2591\u2590\u2591' },
  { value: 'right', icon: '\u2591\u2591\u2590' },
  { value: 'justify', icon: '\u2590\u2590\u2590' },
];

interface InspectorCommonProps {
  block: Block;
  onChange: (block: Block) => void;
  templateMode?: boolean;
}

export function InspectorCommon({ block, onChange, templateMode }: InspectorCommonProps) {
  const { meta } = block;

  function updateMeta(updates: Partial<BlockMeta>) {
    onChange({ ...block, meta: { ...block.meta, ...updates } });
  }

  return (
    <div className="space-y-3 border-t border-gray-200 pt-3 mt-4">
      <Field label="Alignment">
        <div className="flex gap-1">
          {ALIGNMENT_OPTIONS.map((opt) => (
            <button
              key={opt.value}
              onClick={() => updateMeta({ align: opt.value })}
              className={`flex-1 py-1 rounded text-xs capitalize ${
                meta.align === opt.value ? 'bg-gray-200 font-medium' : 'hover:bg-gray-100'
              }`}
            >
              {opt.icon}
            </button>
          ))}
        </div>
      </Field>

      <div className="space-y-2">
        <span className="text-xs text-gray-500 block">Page Breaks</span>
        <InspectorCheckbox
          label="Keep with next block"
          checked={meta.keepWithNext}
          onChange={(v) => updateMeta({ keepWithNext: v })}
        />
        <InspectorCheckbox
          label="Page break before"
          checked={meta.pageBreakBefore}
          onChange={(v) => updateMeta({ pageBreakBefore: v })}
        />
      </div>

      {templateMode && (
        <div className="border-t border-gray-200 pt-3">
          <InspectorCheckbox
            label="Lock block"
            checked={meta.locked}
            onChange={(v) => updateMeta({ locked: v })}
            bold
          />
          <p className="text-xs text-gray-400 mt-1">
            Locked blocks cannot be edited in documents
          </p>
        </div>
      )}
    </div>
  );
}

export function Field({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <div>
      <label className="text-xs text-gray-500 block mb-1">{label}</label>
      {children}
    </div>
  );
}

export function InspectorCheckbox({
  label,
  checked,
  onChange,
  bold,
}: {
  label: string;
  checked: boolean;
  onChange: (v: boolean) => void;
  bold?: boolean;
}) {
  return (
    <label className="flex items-center gap-2 text-xs">
      <input
        type="checkbox"
        checked={checked}
        onChange={(e) => onChange(e.target.checked)}
      />
      <span className={bold ? 'font-medium' : ''}>{label}</span>
    </label>
  );
}
