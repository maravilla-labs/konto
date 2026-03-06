import { cn } from '@/lib/utils';

interface MarkdownPreviewProps {
  content: string;
  className?: string;
}

/**
 * Renders Markdown as HTML for read-only display.
 * Handles: bold, italic, bullet/ordered lists, line breaks.
 */
export function MarkdownPreview({ content, className }: MarkdownPreviewProps) {
  if (!content) return null;

  const html = mdToHtml(content);
  return (
    <div
      className={cn('prose prose-sm dark:prose-invert max-w-none', className)}
      dangerouslySetInnerHTML={{ __html: html }}
    />
  );
}

/**
 * Strip Markdown formatting and return plain text (for table cell previews).
 */
export function stripMarkdown(md: string): string {
  return md
    .replace(/\*\*(.+?)\*\*/g, '$1')
    .replace(/\*(.+?)\*/g, '$1')
    .replace(/^[-*+]\s+/gm, '')
    .replace(/^\d+\.\s+/gm, '')
    .replace(/\n/g, ' ')
    .trim();
}

function mdToHtml(md: string): string {
  const lines = md.split('\n');
  const result: string[] = [];
  let inUl = false;
  let inOl = false;

  for (const line of lines) {
    const trimmed = line.trim();

    // Bullet list
    if (/^[-*+]\s+/.test(trimmed)) {
      if (!inUl) { closeLists(); inUl = true; result.push('<ul>'); }
      result.push(`<li>${inlineMd(trimmed.replace(/^[-*+]\s+/, ''))}</li>`);
      continue;
    }

    // Ordered list
    if (/^\d+\.\s+/.test(trimmed)) {
      if (!inOl) { closeLists(); inOl = true; result.push('<ol>'); }
      result.push(`<li>${inlineMd(trimmed.replace(/^\d+\.\s+/, ''))}</li>`);
      continue;
    }

    closeLists();

    if (trimmed === '') {
      result.push('<br />');
    } else {
      result.push(`<p>${inlineMd(trimmed)}</p>`);
    }
  }

  closeLists();
  return result.join('');

  function closeLists() {
    if (inUl) { result.push('</ul>'); inUl = false; }
    if (inOl) { result.push('</ol>'); inOl = false; }
  }
}

function inlineMd(text: string): string {
  return escapeHtml(text)
    .replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
    .replace(/\*(.+?)\*/g, '<em>$1</em>');
}

function escapeHtml(s: string): string {
  return s
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}
