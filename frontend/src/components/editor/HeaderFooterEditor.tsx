import { useEffect, useRef } from 'react';
import type { Block, TextBlockData } from '../../lib/editor/types';
import { createBlock } from '../../lib/editor/block-factory';

interface HeaderFooterEditorProps {
  blocks: Block[];
  onChange: (blocks: Block[]) => void;
  zone: 'header' | 'footer';
  locked?: boolean;
}

function HFBlockEditor({
  block,
  onUpdate,
  locked,
  zone,
}: {
  block: Block;
  onUpdate: (b: Block) => void;
  locked?: boolean;
  zone: string;
}) {
  const ref = useRef<HTMLDivElement>(null);

  // Sync HTML on mount and block identity change
  useEffect(() => {
    if (ref.current) {
      const data = block.data as TextBlockData;
      if (ref.current.innerHTML !== data._html) {
        ref.current.innerHTML = data._html;
      }
    }
  }, [block.id]); // eslint-disable-line react-hooks/exhaustive-deps

  return (
    <div
      ref={ref}
      contentEditable={!locked}
      suppressContentEditableWarning
      className="outline-none py-0.5 empty:before:content-[attr(data-placeholder)] empty:before:text-gray-300"
      data-placeholder={`Add ${zone} text...`}
      onInput={() => {
        if (!ref.current) return;
        onUpdate({
          ...block,
          data: {
            text: ref.current.innerText,
            _html: ref.current.innerHTML,
          } as TextBlockData,
        });
      }}
    />
  );
}

export function HeaderFooterEditor({
  blocks,
  onChange,
  zone,
  locked,
}: HeaderFooterEditorProps) {
  function updateBlock(updated: Block) {
    onChange(blocks.map((b) => (b.id === updated.id ? updated : b)));
  }

  function addBlock() {
    onChange([...blocks, createBlock('p')]);
  }

  const borderClass =
    zone === 'header'
      ? 'border-b border-gray-200 pb-2'
      : 'border-t border-gray-200 pt-2';

  return (
    <div className={`text-xs ${borderClass}`}>
      {blocks.map((block) => (
        <HFBlockEditor
          key={block.id}
          block={block}
          onUpdate={updateBlock}
          locked={locked}
          zone={zone}
        />
      ))}
      {!locked && blocks.length === 0 && (
        <button
          onClick={addBlock}
          className="text-gray-400 hover:text-gray-600 text-xs"
        >
          + Add {zone}
        </button>
      )}
    </div>
  );
}
