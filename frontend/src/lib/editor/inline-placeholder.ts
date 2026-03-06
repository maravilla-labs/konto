import type { PlaceholderContext } from './placeholder-resolver';

/**
 * Insert an inline placeholder span at the current caret position.
 * The span is contentEditable=false so it behaves as a single "chip".
 */
export function insertInlinePlaceholder(variable: string): void {
  const sel = window.getSelection();
  if (!sel || sel.rangeCount === 0) return;

  const range = sel.getRangeAt(0);
  range.deleteContents();

  const span = document.createElement('span');
  span.contentEditable = 'false';
  span.className = 'inline-placeholder';
  span.dataset.variable = variable;
  span.textContent = `{{${variable}}}`;
  span.style.cssText =
    'background:#fef3c7;color:#92400e;font-family:monospace;font-size:0.85em;' +
    'padding:1px 4px;border-radius:3px;cursor:default;user-select:all;';

  range.insertNode(span);

  // Move caret after the inserted span
  const afterRange = document.createRange();
  afterRange.setStartAfter(span);
  afterRange.collapse(true);
  sel.removeAllRanges();
  sel.addRange(afterRange);
}

/**
 * Resolve inline placeholder spans in an HTML string.
 * Replaces <span class="inline-placeholder" data-variable="x">{{x}}</span>
 * with the resolved value from context, or leaves the raw text if not resolved.
 */
export function resolveInlinePlaceholders(
  html: string,
  ctx: PlaceholderContext,
): string {
  // Match inline placeholder spans and replace with resolved values
  return html.replace(
    /<span[^>]*class="inline-placeholder"[^>]*data-variable="(\w+)"[^>]*>[^<]*<\/span>/g,
    (_match, variable: string) => {
      return ctx[variable] ?? `{{${variable}}}`;
    },
  );
}
