/** Get character offset of caret within an element */
export function getCaretOffset(el: HTMLElement): number {
  const sel = window.getSelection();
  if (!sel || sel.rangeCount === 0) return 0;

  const range = sel.getRangeAt(0);
  const preRange = range.cloneRange();
  preRange.selectNodeContents(el);
  preRange.setEnd(range.startContainer, range.startOffset);
  return preRange.toString().length;
}

/** Set caret to a specific character offset */
export function setCaretOffset(el: HTMLElement, offset: number): void {
  const sel = window.getSelection();
  if (!sel) return;

  const range = document.createRange();
  let currentOffset = 0;

  function walk(node: Node): boolean {
    if (node.nodeType === Node.TEXT_NODE) {
      const len = (node.textContent || '').length;
      if (currentOffset + len >= offset) {
        range.setStart(node, offset - currentOffset);
        range.collapse(true);
        return true;
      }
      currentOffset += len;
    } else {
      for (const child of Array.from(node.childNodes)) {
        if (walk(child)) return true;
      }
    }
    return false;
  }

  if (!walk(el)) {
    // offset beyond content, place at end
    range.selectNodeContents(el);
    range.collapse(false);
  }

  sel.removeAllRanges();
  sel.addRange(range);
}

/** Set caret to the start of an element */
export function setCaretStart(el: HTMLElement): void {
  setCaretOffset(el, 0);
}

/** Set caret to the end of an element */
export function setCaretEnd(el: HTMLElement): void {
  const sel = window.getSelection();
  if (!sel) return;

  const range = document.createRange();
  range.selectNodeContents(el);
  range.collapse(false);
  sel.removeAllRanges();
  sel.addRange(range);
}

/** Check if the caret is at the very start of the element */
export function caretAtStart(el: HTMLElement): boolean {
  return getCaretOffset(el) === 0;
}

/** Check if the caret is at the very end of the element */
export function caretAtEnd(el: HTMLElement): boolean {
  return getCaretOffset(el) >= (el.innerText || '').length;
}

/** Find the first text node in the element tree */
function findFirstTextNode(el: Node): Text | null {
  if (el.nodeType === Node.TEXT_NODE) return el as Text;
  for (const child of Array.from(el.childNodes)) {
    const found = findFirstTextNode(child);
    if (found) return found;
  }
  return null;
}

/** Find the last text node in the element tree */
function findLastTextNode(el: Node): Text | null {
  if (el.nodeType === Node.TEXT_NODE) return el as Text;
  const children = Array.from(el.childNodes);
  for (let i = children.length - 1; i >= 0; i--) {
    const found = findLastTextNode(children[i]);
    if (found) return found;
  }
  return null;
}

/** Check if caret is on the first visual line using first character Y comparison */
export function caretOnFirstLine(el: HTMLElement): boolean {
  const sel = window.getSelection();
  if (!sel || sel.rangeCount === 0) return true;

  const caretRange = sel.getRangeAt(0);
  const caretRect = caretRange.getBoundingClientRect();

  // Get the Y of the first character
  const firstText = findFirstTextNode(el);
  if (!firstText || !firstText.textContent?.length) return true;

  const testRange = document.createRange();
  testRange.setStart(firstText, 0);
  testRange.setEnd(firstText, Math.min(1, firstText.textContent.length));
  const firstCharRect = testRange.getBoundingClientRect();

  return Math.abs(caretRect.top - firstCharRect.top) < 5;
}

/** Check if caret is on the last visual line using last character Y comparison */
export function caretOnLastLine(el: HTMLElement): boolean {
  const sel = window.getSelection();
  if (!sel || sel.rangeCount === 0) return true;

  const caretRange = sel.getRangeAt(0);
  const caretRect = caretRange.getBoundingClientRect();

  // Get the Y of the last character
  const lastText = findLastTextNode(el);
  if (!lastText || !lastText.textContent?.length) return true;

  const testRange = document.createRange();
  const len = lastText.textContent.length;
  testRange.setStart(lastText, Math.max(0, len - 1));
  testRange.setEnd(lastText, len);
  const lastCharRect = testRange.getBoundingClientRect();

  return Math.abs(caretRect.bottom - lastCharRect.bottom) < 5;
}
