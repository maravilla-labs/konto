/** Toggle bold formatting on the current selection */
export function toggleBold(): void {
  document.execCommand('bold');
}

/** Toggle italic formatting on the current selection */
export function toggleItalic(): void {
  document.execCommand('italic');
}

/** Toggle underline formatting on the current selection */
export function toggleUnderline(): void {
  document.execCommand('underline');
}

/** Toggle strikethrough formatting on the current selection */
export function toggleStrikethrough(): void {
  document.execCommand('strikethrough');
}

/** Wrap the current selection in a link */
export function createLink(url: string): void {
  document.execCommand('createLink', false, url);
}

/** Remove link from the current selection */
export function removeLink(): void {
  document.execCommand('unlink');
}

/** Check if a formatting command is currently active */
export function isFormatActive(command: string): boolean {
  return document.queryCommandState(command);
}

/** Set font size on the current selection */
export function setFontSize(size: string): void {
  document.execCommand('fontSize', false, size);
}

/** Check if any text is currently selected */
export function hasSelection(): boolean {
  const sel = window.getSelection();
  return !!sel && !sel.isCollapsed;
}

/** Get the bounding rect of the current selection */
export function getSelectionRect(): DOMRect | null {
  const sel = window.getSelection();
  if (!sel || sel.rangeCount === 0 || sel.isCollapsed) return null;
  return sel.getRangeAt(0).getBoundingClientRect();
}
