import { useState, useRef, useEffect } from 'react';
import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Textarea } from '@/components/ui/textarea';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from '@/components/ui/dialog';
import {
  useAnnualReportNotes,
  useUpdateNote,
  useCreateNote,
  useDeleteNote,
} from '@/hooks/useAnnualReportApi';
import { toast } from 'sonner';
import { Pencil, Plus, Trash2, Check, X, Info } from 'lucide-react';
import type { AnnualReportNote } from '@/types/annual-report';
import { EmployeesEditor } from './EmployeesEditor';

export function AnnualReportNotesEditor({
  fiscalYearId,
}: {
  fiscalYearId: string;
}) {
  const { data: notes, isLoading } = useAnnualReportNotes(fiscalYearId);
  const updateNote = useUpdateNote();
  const createNote = useCreateNote();
  const deleteNote = useDeleteNote();
  const [addDialogOpen, setAddDialogOpen] = useState(false);
  const [newLabel, setNewLabel] = useState('');

  function handleAddSection() {
    if (!newLabel.trim()) return;
    createNote.mutate(
      { fiscalYearId, data: { label: newLabel.trim() } },
      {
        onSuccess: () => {
          toast.success('Section added');
          setNewLabel('');
          setAddDialogOpen(false);
        },
        onError: () => toast.error('Failed to add section'),
      },
    );
  }

  function handleDeleteSection(sectionKey: string) {
    if (!confirm('Delete this custom section?')) return;
    deleteNote.mutate(
      { fiscalYearId, sectionKey },
      {
        onSuccess: () => toast.success('Section deleted'),
        onError: () => toast.error('Failed to delete section'),
      },
    );
  }

  if (isLoading) {
    return (
      <div className="space-y-3">
        {Array.from({ length: 4 }).map((_, i) => (
          <Skeleton key={i} className="h-24 w-full rounded-xl" />
        ))}
      </div>
    );
  }

  const sortedNotes = [...(notes ?? [])].sort(
    (a, b) => a.sort_order - b.sort_order,
  );

  return (
    <div className="space-y-3">
      {sortedNotes.map((note, idx) => (
        <NoteSection
          key={note.section_key}
          note={note}
          displayNumber={idx + 1}
          onSave={(content, label) => {
            updateNote.mutate(
              {
                fiscalYearId,
                section: note.section_key,
                data: { content, label },
              },
              {
                onSuccess: () => toast.success('Saved'),
                onError: () => toast.error('Failed to save'),
              },
            );
          }}
          onDelete={
            note.section_key.startsWith('custom_')
              ? () => handleDeleteSection(note.section_key)
              : undefined
          }
        />
      ))}

      <button
        className="flex w-full items-center justify-center gap-2 rounded-xl border-2 border-dashed border-muted-foreground/20 py-4 text-sm text-muted-foreground transition-colors hover:border-primary/40 hover:text-primary"
        onClick={() => setAddDialogOpen(true)}
      >
        <Plus className="h-4 w-4" /> Add Section
      </button>

      <Dialog open={addDialogOpen} onOpenChange={setAddDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Add Anhang Section</DialogTitle>
          </DialogHeader>
          <div className="space-y-2">
            <Label>Section Title</Label>
            <Input
              value={newLabel}
              onChange={(e) => setNewLabel(e.target.value)}
              placeholder="e.g. Leasingverpflichtungen"
              onKeyDown={(e) => e.key === 'Enter' && handleAddSection()}
              autoFocus
            />
          </div>
          <DialogFooter>
            <Button variant="outline" onClick={() => setAddDialogOpen(false)}>
              Cancel
            </Button>
            <Button
              onClick={handleAddSection}
              disabled={!newLabel.trim() || createNote.isPending}
            >
              Add
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}

function NoteSection({
  note,
  displayNumber,
  onSave,
  onDelete,
}: {
  note: AnnualReportNote;
  displayNumber: number;
  onSave: (content: Record<string, unknown>, label?: string) => void;
  onDelete?: () => void;
}) {
  const isCustom = note.section_key.startsWith('custom_');
  const isTextSection = note.section_type === 'text' || isCustom;
  const isEmployees = note.section_type === 'employees';
  const isAuto =
    note.section_type === 'auto_company_info' ||
    note.section_type === 'auto_fx_rates';

  return (
    <Card className="overflow-hidden rounded-xl">
      <div className="flex items-start justify-between px-5 pt-4 pb-1">
        <SectionTitle
          displayNumber={displayNumber}
          label={note.label}
          isCustom={isCustom}
          onSave={(label) => onSave(note.content, label)}
        />
        {onDelete && (
          <Button
            variant="ghost"
            size="sm"
            className="h-7 w-7 shrink-0 p-0 text-muted-foreground/50 hover:text-destructive"
            onClick={onDelete}
          >
            <Trash2 className="h-3.5 w-3.5" />
          </Button>
        )}
      </div>

      <div className="px-5 pb-4">
        {isTextSection && (
          <InlineTextEditor
            note={note}
            onSave={(content) => onSave(content, isCustom ? undefined : undefined)}
          />
        )}

        {isAuto && (
          <div className="flex items-center gap-2 rounded-lg bg-muted/50 px-3 py-2.5 text-xs text-muted-foreground">
            <Info className="h-3.5 w-3.5 shrink-0" />
            {note.section_type === 'auto_company_info'
              ? 'Automatisch aus Firmeneinstellungen und Gesellschaftern.'
              : 'Automatisch aus Wechselkursen per Bilanzstichtag.'}
          </div>
        )}

        {isEmployees && (
          <EmployeesEditor
            entries={
              (note.content.entries as Array<{
                location: string;
                count: number;
              }>) ?? []
            }
            onSave={(entries) => onSave({ ...note.content, entries })}
          />
        )}
      </div>
    </Card>
  );
}

function SectionTitle({
  displayNumber,
  label,
  isCustom,
  onSave,
}: {
  displayNumber: number;
  label: string;
  isCustom: boolean;
  onSave: (label: string) => void;
}) {
  const [editing, setEditing] = useState(false);
  const [draft, setDraft] = useState(label);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (editing) inputRef.current?.focus();
  }, [editing]);

  if (editing && isCustom) {
    return (
      <div className="flex items-center gap-1.5">
        <span className="text-sm font-semibold text-muted-foreground">
          {displayNumber}.
        </span>
        <Input
          ref={inputRef}
          value={draft}
          onChange={(e) => setDraft(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === 'Enter') {
              onSave(draft);
              setEditing(false);
            }
            if (e.key === 'Escape') {
              setDraft(label);
              setEditing(false);
            }
          }}
          className="h-7 text-sm font-semibold"
        />
        <Button
          variant="ghost"
          size="sm"
          className="h-6 w-6 p-0"
          onClick={() => {
            onSave(draft);
            setEditing(false);
          }}
        >
          <Check className="h-3.5 w-3.5" />
        </Button>
        <Button
          variant="ghost"
          size="sm"
          className="h-6 w-6 p-0"
          onClick={() => {
            setDraft(label);
            setEditing(false);
          }}
        >
          <X className="h-3.5 w-3.5" />
        </Button>
      </div>
    );
  }

  return (
    <div className="group flex items-center gap-1.5">
      <h3 className="text-sm font-semibold">
        {displayNumber}. {label}
      </h3>
      {isCustom && (
        <button
          className="opacity-0 transition-opacity group-hover:opacity-100"
          onClick={() => setEditing(true)}
        >
          <Pencil className="h-3 w-3 text-muted-foreground" />
        </button>
      )}
    </div>
  );
}

function InlineTextEditor({
  note,
  onSave,
}: {
  note: AnnualReportNote;
  onSave: (content: Record<string, unknown>) => void;
}) {
  const textKey = note.section_key === 'extraordinary' ? 'explanation' : 'text';
  const currentText = (note.content[textKey] as string) ?? '';
  const [editing, setEditing] = useState(false);
  const [draft, setDraft] = useState(currentText);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    setDraft(currentText);
  }, [currentText]);

  useEffect(() => {
    if (editing && textareaRef.current) {
      textareaRef.current.focus();
      textareaRef.current.style.height = 'auto';
      textareaRef.current.style.height =
        textareaRef.current.scrollHeight + 'px';
    }
  }, [editing]);

  function save() {
    onSave({ ...note.content, [textKey]: draft });
    setEditing(false);
  }

  function cancel() {
    setDraft(currentText);
    setEditing(false);
  }

  if (editing) {
    return (
      <div className="space-y-2">
        <Textarea
          ref={textareaRef}
          value={draft}
          onChange={(e) => {
            setDraft(e.target.value);
            e.target.style.height = 'auto';
            e.target.style.height = e.target.scrollHeight + 'px';
          }}
          onKeyDown={(e) => {
            if (e.key === 'Escape') cancel();
          }}
          className="min-h-[80px] resize-none text-sm leading-relaxed"
        />
        <div className="flex gap-1.5">
          <Button size="sm" variant="default" className="h-7 text-xs" onClick={save}>
            <Check className="mr-1 h-3 w-3" /> Save
          </Button>
          <Button size="sm" variant="ghost" className="h-7 text-xs" onClick={cancel}>
            Cancel
          </Button>
        </div>
      </div>
    );
  }

  const isEmpty = !currentText.trim();

  return (
    <div
      className="group relative cursor-pointer rounded-lg px-3 py-2.5 transition-colors hover:bg-muted/50"
      onClick={() => setEditing(true)}
    >
      {isEmpty ? (
        <p className="text-sm italic text-muted-foreground/50">
          Click to add text...
        </p>
      ) : (
        <p className="whitespace-pre-wrap text-sm leading-relaxed text-foreground/80">
          {currentText}
        </p>
      )}
      <Pencil className="absolute right-2 top-2 h-3.5 w-3.5 text-muted-foreground/40 opacity-0 transition-opacity group-hover:opacity-100" />
    </div>
  );
}

