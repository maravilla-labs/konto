import { useEditor, EditorContent } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import Placeholder from '@tiptap/extension-placeholder';
import { Markdown } from 'tiptap-markdown';
import { Bold, Italic, List, ListOrdered } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useEffect, useRef, useState } from 'react';

interface RichTextEditorProps {
  value: string;
  onChange: (md: string) => void;
  placeholder?: string;
  minimal?: boolean;
  className?: string;
}

export function RichTextEditor({
  value,
  onChange,
  placeholder,
  minimal = false,
  className,
}: RichTextEditorProps) {
  const onChangeRef = useRef(onChange);
  onChangeRef.current = onChange;
  const [focused, setFocused] = useState(false);

  const editor = useEditor({
    extensions: [
      StarterKit.configure({
        heading: minimal ? false : { levels: [1, 2, 3] },
        blockquote: minimal ? false : undefined,
        codeBlock: false,
        horizontalRule: false,
      }),
      Placeholder.configure({ placeholder: placeholder ?? '' }),
      Markdown,
    ],
    content: value || '',
    onUpdate: ({ editor }) => {
      const md = (editor.storage as Record<string, any>).markdown.getMarkdown() as string;
      onChangeRef.current(md);
    },
    onFocus: () => setFocused(true),
    onBlur: () => setFocused(false),
  });

  // Sync external value changes (e.g. form reset)
  const lastValue = useRef(value);
  useEffect(() => {
    if (editor && value !== lastValue.current) {
      const currentMd = (editor.storage as Record<string, any>).markdown.getMarkdown() as string;
      if (value !== currentMd) {
        editor.commands.setContent(value || '');
      }
      lastValue.current = value;
    }
  }, [value, editor]);

  if (!editor) return null;

  return (
    <div
      className={cn(
        'border-input focus-within:border-ring focus-within:ring-ring/50 rounded-md border bg-transparent shadow-xs transition-[color,box-shadow] focus-within:ring-[3px]',
        minimal ? 'min-h-9' : 'min-h-16',
        className,
      )}
    >
      {focused && !minimal && (
        <div className="flex items-center gap-0.5 border-b px-2 py-1">
          <ToolbarButton
            active={editor.isActive('bold')}
            onClick={() => editor.chain().focus().toggleBold().run()}
          >
            <Bold className="h-3.5 w-3.5" />
          </ToolbarButton>
          <ToolbarButton
            active={editor.isActive('italic')}
            onClick={() => editor.chain().focus().toggleItalic().run()}
          >
            <Italic className="h-3.5 w-3.5" />
          </ToolbarButton>
          <ToolbarButton
            active={editor.isActive('bulletList')}
            onClick={() => editor.chain().focus().toggleBulletList().run()}
          >
            <List className="h-3.5 w-3.5" />
          </ToolbarButton>
          <ToolbarButton
            active={editor.isActive('orderedList')}
            onClick={() => editor.chain().focus().toggleOrderedList().run()}
          >
            <ListOrdered className="h-3.5 w-3.5" />
          </ToolbarButton>
        </div>
      )}

      <EditorContent
        editor={editor}
        className={cn(
          'prose prose-sm dark:prose-invert max-w-none px-3 py-2 text-base md:text-sm',
          '[&_.tiptap]:outline-none [&_.tiptap]:min-h-[inherit]',
          '[&_.tiptap_p.is-editor-empty:first-child::before]:text-muted-foreground [&_.tiptap_p.is-editor-empty:first-child::before]:content-[attr(data-placeholder)] [&_.tiptap_p.is-editor-empty:first-child::before]:float-left [&_.tiptap_p.is-editor-empty:first-child::before]:h-0 [&_.tiptap_p.is-editor-empty:first-child::before]:pointer-events-none',
        )}
      />
    </div>
  );
}

function ToolbarButton({
  active,
  onClick,
  children,
}: {
  active: boolean;
  onClick: () => void;
  children: React.ReactNode;
}) {
  return (
    <button
      type="button"
      onMouseDown={(e) => {
        e.preventDefault(); // Prevent editor blur
        onClick();
      }}
      className={cn(
        'rounded p-1.5 text-muted-foreground hover:bg-accent hover:text-accent-foreground',
        active && 'bg-accent text-accent-foreground',
      )}
    >
      {children}
    </button>
  );
}
