import { useState, useRef, useEffect, useMemo } from 'react';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import { useAccountTreeWithBalances } from '@/hooks/useApi';
import type { AccountTreeWithBalance } from '@/types/accounts';

const typeColors: Record<string, string> = {
  asset: 'bg-blue-100 text-blue-800',
  liability: 'bg-red-100 text-red-800',
  equity: 'bg-purple-100 text-purple-800',
  revenue: 'bg-green-100 text-green-800',
  expense: 'bg-orange-100 text-orange-800',
};

function flattenTree(nodes: AccountTreeWithBalance[]): AccountTreeWithBalance[] {
  const result: AccountTreeWithBalance[] = [];
  for (const n of nodes) {
    result.push(n);
    if (n.children.length > 0) result.push(...flattenTree(n.children));
  }
  return result;
}

interface Props {
  value: string;
  onChange: (accountId: string) => void;
  placeholder?: string;
  className?: string;
  autoFocus?: boolean;
  onKeyDown?: (e: React.KeyboardEvent) => void;
}

export function AccountSelect({ value, onChange, placeholder, className, autoFocus, onKeyDown }: Props) {
  const { data: tree } = useAccountTreeWithBalances();
  const allAccounts = useMemo(() => (tree ? flattenTree(tree) : []), [tree]);
  const [search, setSearch] = useState('');
  const [open, setOpen] = useState(false);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);
  const listRef = useRef<HTMLDivElement>(null);

  const selected = allAccounts.find((a) => a.id === value);

  const filtered = useMemo(() => {
    if (!search) return allAccounts.slice(0, 20);
    const lower = search.toLowerCase();
    return allAccounts
      .filter(
        (a) =>
          a.number.toString().includes(search) ||
          a.name.toLowerCase().includes(lower)
      )
      .slice(0, 20);
  }, [allAccounts, search]);

  useEffect(() => {
    setSelectedIndex(0);
  }, [filtered]);

  useEffect(() => {
    if (selected && !open) {
      setSearch(`${selected.number} ${selected.name}`);
    }
  }, [selected, open]);

  function handleSelect(account: AccountTreeWithBalance) {
    onChange(account.id);
    setSearch(`${account.number} ${account.name}`);
    setOpen(false);
  }

  function handleKeyDown(e: React.KeyboardEvent) {
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      setSelectedIndex((i) => Math.min(i + 1, filtered.length - 1));
      scrollToItem(selectedIndex + 1);
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      setSelectedIndex((i) => Math.max(i - 1, 0));
      scrollToItem(selectedIndex - 1);
    } else if (e.key === 'Enter') {
      e.preventDefault();
      if (filtered[selectedIndex]) {
        handleSelect(filtered[selectedIndex]);
      }
    } else if (e.key === 'Escape') {
      setOpen(false);
    }
    onKeyDown?.(e);
  }

  function scrollToItem(index: number) {
    const list = listRef.current;
    if (!list) return;
    const item = list.children[index] as HTMLElement;
    if (item) item.scrollIntoView({ block: 'nearest' });
  }

  return (
    <div className="relative">
      <Input
        ref={inputRef}
        value={search}
        onChange={(e) => {
          setSearch(e.target.value);
          setOpen(true);
        }}
        onFocus={() => {
          setOpen(true);
          setSearch('');
        }}
        onBlur={() => {
          setTimeout(() => setOpen(false), 150);
        }}
        onKeyDown={handleKeyDown}
        placeholder={placeholder ?? 'Type account # or name...'}
        className={className}
        autoFocus={autoFocus}
      />
      {open && filtered.length > 0 && (
        <div
          ref={listRef}
          className="absolute z-50 mt-1 max-h-48 w-full overflow-auto rounded-md border bg-popover shadow-md"
        >
          {filtered.map((account, i) => (
            <button
              key={account.id}
              type="button"
              className={`flex w-full items-center gap-2 px-3 py-1.5 text-left text-sm hover:bg-muted ${
                i === selectedIndex ? 'bg-muted' : ''
              }`}
              onMouseDown={(e) => {
                e.preventDefault();
                handleSelect(account);
              }}
            >
              <span className="font-mono text-muted-foreground">{account.number}</span>
              <span className="flex-1 truncate">{account.name}</span>
              <Badge variant="secondary" className={`text-[10px] ${typeColors[account.account_type] ?? ''}`}>
                {account.account_type}
              </Badge>
            </button>
          ))}
        </div>
      )}
    </div>
  );
}
