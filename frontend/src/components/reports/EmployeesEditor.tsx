import { useState, useEffect } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Pencil, Plus, Check, X } from 'lucide-react';

export function EmployeesEditor({
  entries: initialEntries,
  onSave,
}: {
  entries: Array<{ location: string; count: number }>;
  onSave: (entries: Array<{ location: string; count: number }>) => void;
}) {
  const [editing, setEditing] = useState(false);
  const [entries, setEntries] = useState(initialEntries);

  useEffect(() => {
    setEntries(initialEntries);
  }, [initialEntries]);

  function updateEntry(
    idx: number,
    field: 'location' | 'count',
    value: string | number,
  ) {
    const updated = [...entries];
    updated[idx] = { ...updated[idx], [field]: value };
    setEntries(updated);
  }

  function save() {
    onSave(entries);
    setEditing(false);
  }

  if (!editing) {
    return (
      <div
        className="group relative cursor-pointer rounded-lg px-3 py-2.5 transition-colors hover:bg-muted/50"
        onClick={() => setEditing(true)}
      >
        {entries.length === 0 ? (
          <p className="text-sm italic text-muted-foreground/50">
            Click to add employee locations...
          </p>
        ) : (
          <table className="w-full text-sm">
            <thead>
              <tr className="border-b text-left text-xs text-muted-foreground">
                <th className="pb-1.5 font-medium">Standort</th>
                <th className="pb-1.5 text-right font-medium">Anzahl</th>
              </tr>
            </thead>
            <tbody>
              {entries.map((e, i) => (
                <tr key={i} className="border-b border-muted/50 last:border-0">
                  <td className="py-1.5">{e.location || '—'}</td>
                  <td className="py-1.5 text-right tabular-nums">{e.count}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
        <Pencil className="absolute right-2 top-2 h-3.5 w-3.5 text-muted-foreground/40 opacity-0 transition-opacity group-hover:opacity-100" />
      </div>
    );
  }

  return (
    <div className="space-y-2 rounded-lg border border-border/50 p-3">
      {entries.map((entry, idx) => (
        <div key={idx} className="flex items-center gap-2">
          <Input
            value={entry.location}
            onChange={(e) => updateEntry(idx, 'location', e.target.value)}
            placeholder="Standort"
            className="h-8 flex-1 text-sm"
          />
          <Input
            type="number"
            value={entry.count}
            onChange={(e) =>
              updateEntry(idx, 'count', parseInt(e.target.value) || 0)
            }
            className="h-8 w-20 text-right text-sm"
          />
          <Button
            variant="ghost"
            size="sm"
            className="h-7 w-7 shrink-0 p-0 text-muted-foreground hover:text-destructive"
            onClick={() => setEntries(entries.filter((_, i) => i !== idx))}
          >
            <X className="h-3.5 w-3.5" />
          </Button>
        </div>
      ))}
      <div className="flex items-center gap-1.5 pt-1">
        <Button
          variant="outline"
          size="sm"
          className="h-7 text-xs"
          onClick={() => setEntries([...entries, { location: '', count: 0 }])}
        >
          <Plus className="mr-1 h-3 w-3" /> Add
        </Button>
        <Button size="sm" className="h-7 text-xs" onClick={save}>
          <Check className="mr-1 h-3 w-3" /> Save
        </Button>
        <Button
          variant="ghost"
          size="sm"
          className="h-7 text-xs"
          onClick={() => {
            setEntries(initialEntries);
            setEditing(false);
          }}
        >
          Cancel
        </Button>
      </div>
    </div>
  );
}
