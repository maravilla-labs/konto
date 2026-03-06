import { useEffect, useMemo, useRef } from 'react';
import type {
  Block,
  BlockType,
  TextBlockData,
  TableBlockData,
  SpacerData,
} from '../../lib/editor/types';
import { TableBlock } from './TableBlock';
import { SignatureBlock } from './SignatureBlock';
import { ImageBlock } from './ImageBlock';
import { DividerBlock } from './DividerBlock';
import { PlaceholderBlock } from './PlaceholderBlock';
import { ContactInfoBlock } from './ContactInfoBlock';
import { DocMetaBlock } from './DocMetaBlock';
import { InvoiceTableBlock } from './InvoiceTableBlock';

export interface EditorBlockProps {
  block: Block;
  onChange: (block: Block) => void;
  onKeyDown: (e: React.KeyboardEvent, block: Block) => void;
  onFocus: (blockId: string) => void;
  isActive: boolean;
  templateMode?: boolean;
  blockRef: (el: HTMLDivElement | null) => void;
}

function isTextBlock(type: BlockType): boolean {
  return ['h1', 'h2', 'h3', 'p', 'blockquote'].includes(type);
}

const TAG_CLASSES: Record<string, string> = {
  h1: 'text-3xl font-bold',
  h2: 'text-2xl font-semibold',
  h3: 'text-xl font-medium',
  p: 'text-base',
  blockquote: 'text-base border-l-4 border-gray-300 pl-4 italic',
};

const PLACEHOLDERS: Record<string, string> = {
  h1: 'Title',
  h2: 'Heading',
  h3: 'Subheading',
  p: 'Type something...',
  blockquote: 'Quote...',
};

export function EditorBlock({
  block,
  onChange,
  onKeyDown,
  onFocus,
  isActive,
  templateMode,
  blockRef,
}: EditorBlockProps) {
  const contentRef = useRef<HTMLDivElement>(null);
  const isLocked = block.meta.locked && !templateMode;
  const textBlock = isTextBlock(block.type);

  // Apply font/alignment/size styles from meta
  const style = useMemo(
    () => ({
      textAlign: block.meta.align as React.CSSProperties['textAlign'],
      lineHeight: block.meta.lineHeight,
      ...(block.meta.fontSize
        ? { fontSize: `${block.meta.fontSize}pt` }
        : {}),
      fontFamily:
        block.meta.font === 'serif'
          ? 'Georgia, serif'
          : block.meta.font === 'mono'
            ? 'monospace'
            : 'inherit',
    }),
    [block.meta.align, block.meta.lineHeight, block.meta.fontSize, block.meta.font],
  );

  // Migrate legacy blockquote data shape (QuoteBlockData → TextBlockData)
  useEffect(() => {
    if (block.type === 'blockquote') {
      const d = block.data as unknown as Record<string, unknown>;
      if ('quote' in d && !('text' in d)) {
        onChange({
          ...block,
          data: {
            text: (d.quote as string) || '',
            _html: (d._quoteHtml as string) || '',
          } as TextBlockData,
        });
      }
    }
  }, [block.id]); // eslint-disable-line react-hooks/exhaustive-deps

  // Sync HTML on mount and when block identity changes externally
  useEffect(() => {
    if (contentRef.current && textBlock) {
      const data = block.data as TextBlockData;
      if (contentRef.current.innerHTML !== data._html) {
        contentRef.current.innerHTML = data._html;
      }
    }
  }, [block.id]); // eslint-disable-line react-hooks/exhaustive-deps

  function handleInput() {
    if (!contentRef.current) return;
    const data = block.data as TextBlockData;
    onChange({
      ...block,
      data: {
        ...data,
        text: contentRef.current.innerText,
        _html: contentRef.current.innerHTML,
      },
    });
  }

  // Compute inner content based on block type
  let innerContent: React.ReactNode;

  if (textBlock) {
    const tagClass = TAG_CLASSES[block.type] || 'text-base';
    const placeholder = PLACEHOLDERS[block.type];
    innerContent = (
      <div
        ref={contentRef}
        contentEditable={!isLocked}
        suppressContentEditableWarning
        className={`${tagClass} outline-none py-1 empty:before:content-[attr(data-placeholder)] empty:before:text-gray-400`}
        style={style}
        data-placeholder={placeholder}
        data-block-id={block.id}
        onInput={handleInput}
        onKeyDown={(e) => onKeyDown(e, block)}
        onFocus={() => onFocus(block.id)}
      />
    );
  } else if (block.type === 'contact_info') {
    innerContent = (
      <ContactInfoBlock block={block} onChange={onChange} locked={isLocked} />
    );
  } else if (block.type === 'doc_meta') {
    innerContent = (
      <DocMetaBlock block={block} onChange={onChange} locked={isLocked} />
    );
  } else if (block.type === 'table') {
    const tableData = block.data as TableBlockData;
    if (tableData.mode === 'invoice') {
      innerContent = (
        <InvoiceTableBlock block={block} onChange={onChange} locked={isLocked} />
      );
    } else {
      innerContent = (
        <TableBlock block={block} onChange={onChange} locked={isLocked} />
      );
    }
  } else if (block.type === 'signature') {
    innerContent = (
      <SignatureBlock block={block} onChange={onChange} locked={isLocked} />
    );
  } else if (block.type === 'image') {
    innerContent = (
      <ImageBlock block={block} onChange={onChange} locked={isLocked} />
    );
  } else if (block.type === 'divider') {
    innerContent = <DividerBlock />;
  } else if (block.type === 'placeholder') {
    innerContent = (
      <PlaceholderBlock block={block} onChange={onChange} />
    );
  } else if (block.type === 'spacer') {
    const spacerData = block.data as SpacerData;
    innerContent = (
      <div
        style={{ height: spacerData.height }}
        className="border border-dashed border-gray-200 flex items-center justify-center"
      >
        <span className="text-xs text-gray-300 select-none">spacer</span>
      </div>
    );
  }

  // Universal wrapper for ALL blocks
  const wrapperClass = [
    'relative',
    isLocked ? 'opacity-80' : '',
    isActive ? 'ring-2 ring-blue-200 rounded' : '',
  ]
    .filter(Boolean)
    .join(' ');

  return (
    <div
      ref={blockRef}
      className={wrapperClass}
      onClick={() => onFocus(block.id)}
      tabIndex={textBlock ? undefined : 0}
      onKeyDown={textBlock ? undefined : (e) => onKeyDown(e, block)}
    >
      {isLocked && (
        <div className="absolute inset-0 cursor-not-allowed z-10" />
      )}
      {innerContent}
    </div>
  );
}
