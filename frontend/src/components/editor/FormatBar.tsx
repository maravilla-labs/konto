import { useEffect, useState } from 'react';
import {
  toggleBold,
  toggleItalic,
  toggleUnderline,
  toggleStrikethrough,
  hasSelection,
  getSelectionRect,
  isFormatActive,
} from '../../lib/editor/commands';

interface FormatState {
  bold: boolean;
  italic: boolean;
  underline: boolean;
  strikethrough: boolean;
}

interface FormatButtonProps {
  label: string;
  active: boolean;
  className?: string;
  onClick: () => void;
}

function FormatButton({ label, active, className, onClick }: FormatButtonProps) {
  return (
    <button
      className={`w-7 h-7 flex items-center justify-center rounded text-sm hover:bg-gray-700 ${
        active ? 'bg-gray-600' : ''
      } ${className ?? ''}`}
      onMouseDown={(e) => {
        e.preventDefault();
        onClick();
      }}
    >
      {label}
    </button>
  );
}

export function FormatBar() {
  const [position, setPosition] = useState<{
    top: number;
    left: number;
  } | null>(null);
  const [formats, setFormats] = useState<FormatState>({
    bold: false,
    italic: false,
    underline: false,
    strikethrough: false,
  });

  useEffect(() => {
    function onSelectionChange() {
      const rect = getSelectionRect();
      if (rect && hasSelection()) {
        // Find the editor scroll container for proper positioning
        const scrollContainer = document.querySelector('[data-editor-scroll]');
        if (scrollContainer) {
          const containerRect = scrollContainer.getBoundingClientRect();
          setPosition({
            top: rect.top - containerRect.top + scrollContainer.scrollTop - 44,
            left: rect.left - containerRect.left + rect.width / 2,
          });
        } else {
          // Fallback to fixed positioning
          setPosition({
            top: rect.top - 44 + window.scrollY,
            left: rect.left + rect.width / 2,
          });
        }
        setFormats({
          bold: isFormatActive('bold'),
          italic: isFormatActive('italic'),
          underline: isFormatActive('underline'),
          strikethrough: isFormatActive('strikeThrough'),
        });
      } else {
        setPosition(null);
      }
    }

    document.addEventListener('selectionchange', onSelectionChange);
    return () =>
      document.removeEventListener('selectionchange', onSelectionChange);
  }, []);

  if (!position) return null;

  // Check if we have a scroll container for absolute positioning
  const scrollContainer = document.querySelector('[data-editor-scroll]');
  const positionStyle = scrollContainer
    ? { position: 'absolute' as const, top: position.top, left: position.left }
    : { position: 'fixed' as const, top: position.top, left: position.left };

  return (
    <div
      className="z-50 flex gap-1 bg-gray-900 text-white rounded-lg shadow-xl px-2 py-1 -translate-x-1/2"
      style={positionStyle}
    >
      <FormatButton
        label="B"
        active={formats.bold}
        className="font-bold"
        onClick={toggleBold}
      />
      <FormatButton
        label="I"
        active={formats.italic}
        className="italic"
        onClick={toggleItalic}
      />
      <FormatButton
        label="U"
        active={formats.underline}
        className="underline"
        onClick={toggleUnderline}
      />
      <FormatButton
        label="S"
        active={formats.strikethrough}
        className="line-through"
        onClick={toggleStrikethrough}
      />
      <div className="w-px bg-gray-600 mx-0.5" />
      <FormatButton
        label="✕"
        active={false}
        onClick={() => document.execCommand('removeFormat')}
      />
    </div>
  );
}
