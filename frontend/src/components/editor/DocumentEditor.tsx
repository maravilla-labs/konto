import { useState, useRef, useEffect, useCallback } from 'react';
import type {
  Block,
  BlockType,
  DocumentModel,
  TextBlockData,
  ViewMode,
} from '../../lib/editor/types';
import { createBlock } from '../../lib/editor/block-factory';
import {
  setCaretStart,
  setCaretEnd,
  caretAtStart,
  caretOnFirstLine,
  caretOnLastLine,
} from '../../lib/editor/caret';
import { toggleBold, toggleItalic, toggleUnderline } from '../../lib/editor/commands';
import { insertInlinePlaceholder } from '../../lib/editor/inline-placeholder';
import { EditorToolbar } from './EditorToolbar';
import { BlockInspector, type EditorContext } from './BlockInspector';
import { FormatBar } from './FormatBar';
import { SlashMenu } from './SlashMenu';
import { EditorBlock } from './EditorBlock';
import { PageRenderer } from './PageRenderer';
import { HeaderFooterEditor } from './HeaderFooterEditor';

const TEXT_TYPES = new Set(['h1', 'h2', 'h3', 'p', 'blockquote']);

interface DocumentEditorProps {
  value: DocumentModel;
  onChange: (doc: DocumentModel) => void;
  readOnly?: boolean;
  templateMode?: boolean;
  editorContext?: EditorContext;
}

export function DocumentEditor({
  value,
  onChange,
  readOnly,
  templateMode,
  editorContext,
}: DocumentEditorProps) {
  const [activeBlockId, setActiveBlockId] = useState<string | null>(null);
  const [blockHeights, setBlockHeights] = useState<Map<string, number>>(
    new Map(),
  );
  const [viewMode, setViewMode] = useState<ViewMode>('pages');
  const [zoom, setZoom] = useState(1);
  const [inspectorOpen, setInspectorOpen] = useState(true);
  const [slashMenu, setSlashMenu] = useState<{
    top: number;
    left: number;
  } | null>(null);
  const blockRefs = useRef<Map<string, HTMLDivElement>>(new Map());
  const measureRef = useRef<ResizeObserver | null>(null);

  useEffect(() => {
    const observer = new ResizeObserver((entries) => {
      setBlockHeights((prev) => {
        const next = new Map(prev);
        for (const entry of entries) {
          const el = entry.target as HTMLElement;
          const id = el.dataset.measureId;
          if (id) next.set(id, entry.contentRect.height);
        }
        return next;
      });
    });
    measureRef.current = observer;
    return () => observer.disconnect();
  }, []);

  const setBlockRef = useCallback(
    (blockId: string) => (el: HTMLDivElement | null) => {
      const observer = measureRef.current;
      const prev = blockRefs.current.get(blockId);
      if (prev && observer) observer.unobserve(prev);
      if (el) {
        el.dataset.measureId = blockId;
        blockRefs.current.set(blockId, el);
        if (observer) observer.observe(el);
      } else {
        blockRefs.current.delete(blockId);
      }
    },
    [],
  );

  function updateBlock(updated: Block) {
    onChange({
      ...value,
      blocks: value.blocks.map((b) => (b.id === updated.id ? updated : b)),
    });
  }

  function focusBlock(blockId: string, position: 'start' | 'end') {
    const el = blockRefs.current.get(blockId);
    if (!el) return;
    const editable = el.querySelector(
      '[contenteditable]',
    ) as HTMLElement | null;
    if (editable) {
      editable.focus();
      if (position === 'start') setCaretStart(editable);
      else setCaretEnd(editable);
    } else {
      // Non-text block — focus the wrapper itself (has tabIndex=0)
      el.focus();
    }
    setActiveBlockId(blockId);
  }

  function insertBlockAfter(afterId: string, type: BlockType, initialBlock?: Block) {
    const idx = value.blocks.findIndex((b) => b.id === afterId);
    const newBlock = initialBlock ?? createBlock(type);
    const blocks = [...value.blocks];
    blocks.splice(idx + 1, 0, newBlock);
    onChange({ ...value, blocks });
    setTimeout(() => focusBlock(newBlock.id, 'start'), 0);
  }

  function deleteBlock(id: string) {
    if (value.blocks.length <= 1) return;
    const idx = value.blocks.findIndex((b) => b.id === id);
    const prevBlock = idx > 0 ? value.blocks[idx - 1] : null;
    onChange({ ...value, blocks: value.blocks.filter((b) => b.id !== id) });
    if (prevBlock) {
      setTimeout(() => focusBlock(prevBlock.id, 'end'), 0);
    }
  }

  function changeBlockType(type: BlockType) {
    if (!activeBlockId) return;
    const block = value.blocks.find((b) => b.id === activeBlockId);
    if (!block) return;
    const newBlock = createBlock(type);
    onChange({
      ...value,
      blocks: value.blocks.map((b) =>
        b.id === activeBlockId ? { ...newBlock, id: b.id } : b,
      ),
    });
  }

  function handleDeleteActiveBlock() {
    if (activeBlockId) {
      deleteBlock(activeBlockId);
    }
  }

  function handleBlockKeyDown(e: React.KeyboardEvent, block: Block) {
    if (readOnly) return;

    const isText = TEXT_TYPES.has(block.type);

    // Non-text block keyboard handling
    if (!isText) {
      if (e.key === 'Backspace' || e.key === 'Delete') {
        e.preventDefault();
        deleteBlock(block.id);
        return;
      }
      if (e.key === 'Enter') {
        e.preventDefault();
        insertBlockAfter(block.id, 'p');
        return;
      }
      if (e.key === 'ArrowUp') {
        e.preventDefault();
        const idx = value.blocks.findIndex((b) => b.id === block.id);
        if (idx > 0) focusBlock(value.blocks[idx - 1].id, 'end');
        return;
      }
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        const idx = value.blocks.findIndex((b) => b.id === block.id);
        if (idx < value.blocks.length - 1) {
          focusBlock(value.blocks[idx + 1].id, 'start');
        }
        return;
      }
      return; // Ignore other keys for non-text blocks
    }

    // Text block keyboard handling
    const mod = e.metaKey || e.ctrlKey;
    if (mod && e.key === 'b') {
      e.preventDefault();
      toggleBold();
      return;
    }
    if (mod && e.key === 'i') {
      e.preventDefault();
      toggleItalic();
      return;
    }
    if (mod && e.key === 'u') {
      e.preventDefault();
      toggleUnderline();
      return;
    }

    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      insertBlockAfter(block.id, 'p');
      return;
    }

    if (e.key === 'Backspace') {
      const el = e.currentTarget as HTMLElement;
      if (caretAtStart(el) && el.innerText.length === 0) {
        e.preventDefault();
        deleteBlock(block.id);
        return;
      }
    }

    if (e.key === 'ArrowUp') {
      const el = e.currentTarget as HTMLElement;
      if (caretOnFirstLine(el)) {
        e.preventDefault();
        const idx = value.blocks.findIndex((b) => b.id === block.id);
        if (idx > 0) focusBlock(value.blocks[idx - 1].id, 'end');
      }
    }

    if (e.key === 'ArrowDown') {
      const el = e.currentTarget as HTMLElement;
      if (caretOnLastLine(el)) {
        e.preventDefault();
        const idx = value.blocks.findIndex((b) => b.id === block.id);
        if (idx < value.blocks.length - 1) {
          focusBlock(value.blocks[idx + 1].id, 'start');
        }
      }
    }

    if (e.key === '/' && block.type === 'p') {
      const data = block.data as TextBlockData;
      if (!data.text || data.text.trim() === '') {
        e.preventDefault();
        const el = e.currentTarget as HTMLElement;
        const rect = el.getBoundingClientRect();
        setSlashMenu({ top: rect.bottom + 4, left: rect.left });
      }
    }
  }

  function handleSlashSelect(type: BlockType, initialBlock?: Block) {
    if (activeBlockId) {
      if (initialBlock) {
        onChange({
          ...value,
          blocks: value.blocks.map((b) =>
            b.id === activeBlockId ? { ...initialBlock, id: b.id } : b,
          ),
        });
      } else {
        const newBlock = createBlock(type);
        onChange({
          ...value,
          blocks: value.blocks.map((b) =>
            b.id === activeBlockId ? { ...newBlock, id: b.id } : b,
          ),
        });
      }
      // Focus the replaced block after render
      setTimeout(() => focusBlock(activeBlockId, 'start'), 0);
    }
    setSlashMenu(null);
  }

  function handleScrollContainerClick(e: React.MouseEvent) {
    if (e.target === e.currentTarget) {
      setActiveBlockId(null);
    }
  }

  const activeBlock =
    value.blocks.find((b) => b.id === activeBlockId) ?? null;

  return (
    <div className="flex flex-col h-full">
      <EditorToolbar
        viewMode={viewMode}
        onViewModeChange={setViewMode}
        zoom={zoom}
        onZoomChange={setZoom}
        inspectorOpen={inspectorOpen}
        onToggleInspector={() => setInspectorOpen(!inspectorOpen)}
        activeBlock={activeBlock}
        onChangeBlockType={changeBlockType}
        onDeleteBlock={handleDeleteActiveBlock}
        templateMode={templateMode}
        onInsertVariable={templateMode ? insertInlinePlaceholder : undefined}
      />

      <div className="flex flex-1 overflow-hidden">
        <div
          className="flex-1 overflow-y-auto bg-gray-100 p-8 relative"
          data-editor-scroll
          onClick={handleScrollContainerClick}
        >
          <PageRenderer
            blocks={value.blocks}
            pageSetup={value.pageSetup}
            header={value.header}
            footer={value.footer}
            blockHeights={blockHeights}
            viewMode={viewMode}
            zoom={zoom}
            renderBlock={(block) => (
              <EditorBlock
                key={block.id}
                block={block}
                onChange={updateBlock}
                onKeyDown={handleBlockKeyDown}
                onFocus={setActiveBlockId}
                isActive={activeBlockId === block.id}
                templateMode={templateMode}
                blockRef={setBlockRef(block.id)}
              />
            )}
            renderHeader={renderHeader}
            renderFooter={renderFooter}
          />
          <FormatBar />
        </div>

        {inspectorOpen && (
          <BlockInspector
            block={activeBlock}
            onChange={updateBlock}
            templateMode={templateMode}
            editorContext={editorContext}
          />
        )}
      </div>

      {slashMenu && (
        <SlashMenu
          position={slashMenu}
          onSelect={handleSlashSelect}
          onClose={() => setSlashMenu(null)}
        />
      )}
    </div>
  );

  function renderHeader(_pageIdx: number, _totalPages: number) {
    if (templateMode && !readOnly) {
      return (
        <HeaderFooterEditor
          blocks={value.header}
          onChange={(blocks) => onChange({ ...value, header: blocks })}
          zone="header"
        />
      );
    }
    return (
      <div className="text-xs text-gray-400 flex justify-between">
        {value.header.map((b) => (
          <span key={b.id}>{(b.data as TextBlockData).text}</span>
        ))}
      </div>
    );
  }

  function renderFooter(pageIdx: number, totalPages: number) {
    if (templateMode && !readOnly) {
      return (
        <HeaderFooterEditor
          blocks={value.footer}
          onChange={(blocks) => onChange({ ...value, footer: blocks })}
          zone="footer"
        />
      );
    }
    return (
      <div className="text-xs text-gray-400 flex justify-between">
        {value.footer.map((b) => (
          <span key={b.id}>{(b.data as TextBlockData).text}</span>
        ))}
        <span>Page {pageIdx + 1} / {totalPages}</span>
      </div>
    );
  }
}
