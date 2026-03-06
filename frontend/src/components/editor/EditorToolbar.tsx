import type { Block, BlockType, ViewMode } from '../../lib/editor/types';
import { Trash2 } from 'lucide-react';

interface EditorToolbarProps {
  viewMode: ViewMode;
  onViewModeChange: (mode: ViewMode) => void;
  zoom: number;
  onZoomChange: (zoom: number) => void;
  inspectorOpen: boolean;
  onToggleInspector: () => void;
  activeBlock: Block | null;
  onChangeBlockType: (type: BlockType) => void;
  onDeleteBlock?: () => void;
  templateMode?: boolean;
  onInsertVariable?: (variable: string) => void;
}

const VIEW_MODES: ViewMode[] = ['pages', 'spread', 'canvas'];

const BLOCK_TYPE_OPTIONS: { value: BlockType; label: string }[] = [
  { value: 'h1', label: 'Heading 1' },
  { value: 'h2', label: 'Heading 2' },
  { value: 'h3', label: 'Heading 3' },
  { value: 'p', label: 'Paragraph' },
  { value: 'blockquote', label: 'Quote' },
  { value: 'table', label: 'Table' },
  { value: 'contact_info', label: 'Client Info' },
  { value: 'doc_meta', label: 'Doc Info' },
  { value: 'image', label: 'Image' },
  { value: 'divider', label: 'Divider' },
  { value: 'signature', label: 'Signature' },
  { value: 'spacer', label: 'Spacer' },
  { value: 'placeholder', label: 'Variable' },
];

const PLACEHOLDER_GROUPS = [
  {
    label: 'Company',
    items: [
      { key: 'company_name', label: 'Company Name' },
      { key: 'company_address', label: 'Address' },
      { key: 'company_city', label: 'City' },
      { key: 'company_postal_code', label: 'Postal Code' },
      { key: 'company_country', label: 'Country' },
      { key: 'company_vat', label: 'VAT Number' },
      { key: 'company_email', label: 'Email' },
      { key: 'company_phone', label: 'Phone' },
      { key: 'company_website', label: 'Website' },
    ],
  },
  {
    label: 'Client',
    items: [
      { key: 'client_name', label: 'Client Name' },
      { key: 'client_contact', label: 'Contact Person' },
      { key: 'client_address', label: 'Address' },
      { key: 'client_city', label: 'City' },
      { key: 'client_postal_code', label: 'Postal Code' },
      { key: 'client_country', label: 'Country' },
      { key: 'client_email', label: 'Email' },
    ],
  },
  {
    label: 'Document',
    items: [
      { key: 'doc_number', label: 'Document Number' },
      { key: 'doc_date', label: 'Document Date' },
      { key: 'doc_title', label: 'Title' },
      { key: 'valid_until', label: 'Valid Until' },
      { key: 'subtotal', label: 'Subtotal' },
      { key: 'vat_amount', label: 'VAT Amount' },
      { key: 'total', label: 'Total' },
    ],
  },
];

export function EditorToolbar({
  viewMode,
  onViewModeChange,
  zoom,
  onZoomChange,
  inspectorOpen,
  onToggleInspector,
  activeBlock,
  onChangeBlockType,
  onDeleteBlock,
  templateMode,
  onInsertVariable,
}: EditorToolbarProps) {
  return (
    <div className="flex items-center gap-4 px-4 py-2 bg-white border-b text-sm">
      <ViewModeToggle viewMode={viewMode} onChange={onViewModeChange} />

      <ZoomControls zoom={zoom} onChange={onZoomChange} />

      {activeBlock && (
        <BlockTypeSelector
          value={activeBlock.type}
          onChange={onChangeBlockType}
        />
      )}

      {activeBlock && onDeleteBlock && (
        <button
          onClick={onDeleteBlock}
          className="px-2 py-1 rounded text-xs text-red-500 hover:bg-red-50 hover:text-red-700"
          title="Delete block"
        >
          <Trash2 className="h-3.5 w-3.5" />
        </button>
      )}

      {templateMode && onInsertVariable && (
        <VariableDropdown onInsert={onInsertVariable} />
      )}

      <div className="flex-1" />

      <button
        onClick={onToggleInspector}
        className={`px-3 py-1 rounded text-xs ${
          inspectorOpen ? 'bg-gray-200' : 'hover:bg-gray-100'
        }`}
      >
        Inspector
      </button>
    </div>
  );
}

function ViewModeToggle({
  viewMode,
  onChange,
}: {
  viewMode: ViewMode;
  onChange: (mode: ViewMode) => void;
}) {
  return (
    <div className="flex gap-1 bg-gray-100 rounded-lg p-0.5">
      {VIEW_MODES.map((mode) => (
        <button
          key={mode}
          onClick={() => onChange(mode)}
          className={`px-3 py-1 rounded-md text-xs capitalize ${
            viewMode === mode
              ? 'bg-white shadow text-gray-900'
              : 'text-gray-500 hover:text-gray-700'
          }`}
        >
          {mode}
        </button>
      ))}
    </div>
  );
}

function ZoomControls({
  zoom,
  onChange,
}: {
  zoom: number;
  onChange: (zoom: number) => void;
}) {
  return (
    <div className="flex items-center gap-2">
      <button
        onClick={() => onChange(Math.max(0.25, zoom - 0.1))}
        className="text-gray-500 hover:text-gray-700"
      >
        -
      </button>
      <span className="text-xs text-gray-600 w-12 text-center">
        {Math.round(zoom * 100)}%
      </span>
      <button
        onClick={() => onChange(Math.min(2, zoom + 0.1))}
        className="text-gray-500 hover:text-gray-700"
      >
        +
      </button>
    </div>
  );
}

function BlockTypeSelector({
  value,
  onChange,
}: {
  value: BlockType;
  onChange: (type: BlockType) => void;
}) {
  return (
    <select
      value={value}
      onChange={(e) => onChange(e.target.value as BlockType)}
      className="text-xs border rounded px-2 py-1 bg-white"
    >
      {BLOCK_TYPE_OPTIONS.map((opt) => (
        <option key={opt.value} value={opt.value}>
          {opt.label}
        </option>
      ))}
    </select>
  );
}

function VariableDropdown({ onInsert }: { onInsert: (variable: string) => void }) {
  return (
    <div className="relative group">
      <button className="px-2 py-1 rounded text-xs bg-amber-50 text-amber-700 hover:bg-amber-100 font-mono">
        {'{{}}'}
      </button>
      <div className="absolute top-full left-0 mt-1 w-48 bg-white border rounded-lg shadow-lg hidden group-hover:block z-50 max-h-64 overflow-y-auto">
        {PLACEHOLDER_GROUPS.map((group) => (
          <div key={group.label}>
            <div className="px-3 py-1 text-[10px] font-semibold text-gray-400 uppercase tracking-wider bg-gray-50">
              {group.label}
            </div>
            {group.items.map((item) => (
              <button
                key={item.key}
                className="w-full text-left px-3 py-1.5 text-xs hover:bg-blue-50 text-gray-700"
                onMouseDown={(e) => {
                  e.preventDefault();
                  onInsert(item.key);
                }}
              >
                {item.label}
              </button>
            ))}
          </div>
        ))}
      </div>
    </div>
  );
}
