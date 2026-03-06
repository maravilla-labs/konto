import { useEffect, useState } from 'react';
import type { Block, BlockType } from '../../lib/editor/types';
import { createInvoiceTable } from '../../lib/editor/block-factory';

interface SlashMenuProps {
  position: { top: number; left: number };
  onSelect: (type: BlockType, initialBlock?: Block) => void;
  onClose: () => void;
}

interface MenuItem {
  type: BlockType;
  label: string;
  icon: string;
  description: string;
  initialBlock?: () => Block;
}

const MENU_ITEMS: MenuItem[] = [
  { type: 'h1', label: 'Heading 1', icon: 'H1', description: 'Large heading' },
  { type: 'h2', label: 'Heading 2', icon: 'H2', description: 'Medium heading' },
  { type: 'h3', label: 'Heading 3', icon: 'H3', description: 'Small heading' },
  { type: 'p', label: 'Paragraph', icon: 'P', description: 'Plain text' },
  { type: 'blockquote', label: 'Quote', icon: '"', description: 'Quote block' },
  { type: 'table', label: 'Table', icon: '#', description: 'Data table' },
  {
    type: 'table',
    label: 'Invoice Table',
    icon: '$',
    description: 'Line items table',
    initialBlock: createInvoiceTable,
  },
  { type: 'contact_info', label: 'Client Info', icon: '@', description: 'Client address block' },
  { type: 'doc_meta', label: 'Document Info', icon: 'i', description: 'Dates & project' },
  { type: 'image', label: 'Image', icon: 'Img', description: 'Upload image' },
  { type: 'divider', label: 'Divider', icon: '--', description: 'Horizontal line' },
  { type: 'signature', label: 'Signature', icon: 'Sig', description: 'Signature block' },
  { type: 'spacer', label: 'Spacer', icon: '||', description: 'Vertical space' },
  { type: 'placeholder', label: 'Variable', icon: '{ }', description: 'Template variable' },
];

export function SlashMenu({ position, onSelect, onClose }: SlashMenuProps) {
  const [selected, setSelected] = useState(0);

  useEffect(() => {
    function onKeyDown(e: KeyboardEvent) {
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        setSelected((s) => (s + 1) % MENU_ITEMS.length);
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        setSelected(
          (s) => (s - 1 + MENU_ITEMS.length) % MENU_ITEMS.length,
        );
      } else if (e.key === 'Enter') {
        e.preventDefault();
        const item = MENU_ITEMS[selected];
        onSelect(item.type, item.initialBlock?.());
      } else if (e.key === 'Escape') {
        onClose();
      }
    }

    document.addEventListener('keydown', onKeyDown);
    return () => document.removeEventListener('keydown', onKeyDown);
  }, [selected, onSelect, onClose]);

  return (
    <div
      className="fixed z-50 bg-white border rounded-lg shadow-xl w-64 max-h-72 overflow-y-auto"
      style={{ top: position.top, left: position.left }}
    >
      {MENU_ITEMS.map((item, i) => (
        <button
          key={`${item.type}-${item.label}`}
          className={`w-full flex items-center gap-3 px-3 py-2 text-left hover:bg-gray-100 ${
            i === selected ? 'bg-gray-100' : ''
          }`}
          onClick={() => onSelect(item.type, item.initialBlock?.())}
        >
          <span className="w-8 h-8 flex items-center justify-center bg-gray-100 rounded text-sm font-mono">
            {item.icon}
          </span>
          <div>
            <div className="text-sm font-medium">{item.label}</div>
            <div className="text-xs text-gray-500">{item.description}</div>
          </div>
        </button>
      ))}
    </div>
  );
}
