import { useState } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { RichTextEditor } from '@/components/ui/rich-text-editor';
import { Switch } from '@/components/ui/switch';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Dialog, DialogContent, DialogHeader, DialogTitle,
} from '@/components/ui/dialog';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import { useDunningLevels, useUpdateDunningLevel, useRunDunning } from '@/hooks/useApi';
import { toast } from 'sonner';
import { Pencil, Play } from 'lucide-react';
import type { DunningLevel } from '@/types/dunning';

export function DunningSettingsPage() {
  const { data: levels, isLoading } = useDunningLevels();
  const updateLevel = useUpdateDunningLevel();
  const runDunning = useRunDunning();

  const [editLevel, setEditLevel] = useState<DunningLevel | null>(null);
  const [editForm, setEditForm] = useState({
    days_after_due: 0,
    fee_amount: 0,
    subject_template: '',
    body_template: '',
    is_active: true,
  });

  function openEdit(level: DunningLevel) {
    setEditLevel(level);
    setEditForm({
      days_after_due: level.days_after_due,
      fee_amount: parseFloat(level.fee_amount),
      subject_template: level.subject_template,
      body_template: level.body_template,
      is_active: level.is_active,
    });
  }

  function handleUpdate() {
    if (!editLevel) return;
    updateLevel.mutate(
      { id: editLevel.id, data: editForm },
      {
        onSuccess: () => { toast.success('Dunning level updated'); setEditLevel(null); },
        onError: () => toast.error('Failed to update level'),
      },
    );
  }

  function handleRunDunning() {
    runDunning.mutate(undefined, {
      onSuccess: (res) => {
        const r = res.data;
        toast.success(`Dunning run complete: ${r.reminders_sent} reminders sent`);
        if (r.errors.length > 0) {
          toast.error(`${r.errors.length} errors occurred`);
        }
      },
      onError: () => toast.error('Failed to run dunning'),
    });
  }

  const list = levels ?? [];

  return (
    <div className="space-y-4">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-lg font-semibold">Payment Reminders</h2>
          <p className="text-sm text-muted-foreground">
            Configure dunning levels and run payment reminders
          </p>
        </div>
        <Button size="sm" onClick={handleRunDunning} disabled={runDunning.isPending}>
          <Play className="mr-1 h-4 w-4" /> Run Dunning
        </Button>
      </div>

      <Card>
        <CardContent className="p-0">
          {isLoading ? (
            <div className="space-y-2 p-4">
              {Array.from({ length: 3 }).map((_, i) => (
                <Skeleton key={i} className="h-10 w-full" />
              ))}
            </div>
          ) : list.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Level</TableHead>
                  <TableHead>Days After Due</TableHead>
                  <TableHead>Fee</TableHead>
                  <TableHead className="hidden sm:table-cell">Subject</TableHead>
                  <TableHead>Status</TableHead>
                  <TableHead className="w-16">Edit</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {list.map((l) => (
                  <TableRow key={l.id}>
                    <TableCell className="font-medium">Level {l.level}</TableCell>
                    <TableCell>{l.days_after_due} days</TableCell>
                    <TableCell className="font-mono">
                      CHF {parseFloat(l.fee_amount).toFixed(2)}
                    </TableCell>
                    <TableCell className="hidden sm:table-cell max-w-[200px] truncate text-sm text-muted-foreground">
                      {l.subject_template}
                    </TableCell>
                    <TableCell>
                      <Badge variant={l.is_active ? 'default' : 'secondary'}>
                        {l.is_active ? 'Active' : 'Inactive'}
                      </Badge>
                    </TableCell>
                    <TableCell>
                      <Button
                        variant="ghost" size="icon" className="h-8 w-8"
                        onClick={() => openEdit(l)} title="Edit level"
                      >
                        <Pencil className="h-3.5 w-3.5" />
                      </Button>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">
              No dunning levels configured.
            </p>
          )}
        </CardContent>
      </Card>

      <Card>
        <CardContent className="py-3">
          <p className="text-xs text-muted-foreground">
            Template variables: {'{{company_name}}'}, {'{{contact_name}}'}, {'{{invoice_number}}'}, {'{{amount}}'}, {'{{due_date}}'}
          </p>
        </CardContent>
      </Card>

      {/* Edit Dialog */}
      <Dialog open={!!editLevel} onOpenChange={(open) => !open && setEditLevel(null)}>
        <DialogContent className="max-w-lg">
          <DialogHeader>
            <DialogTitle>Edit Level {editLevel?.level}</DialogTitle>
          </DialogHeader>
          <div className="space-y-4">
            <div className="grid gap-4 sm:grid-cols-2">
              <div>
                <Label>Days After Due</Label>
                <Input
                  type="number" min="1"
                  value={editForm.days_after_due}
                  onChange={(e) => setEditForm({ ...editForm, days_after_due: parseInt(e.target.value) || 0 })}
                />
              </div>
              <div>
                <Label>Fee Amount (CHF)</Label>
                <Input
                  type="number" step="0.01" min="0"
                  value={editForm.fee_amount}
                  onChange={(e) => setEditForm({ ...editForm, fee_amount: parseFloat(e.target.value) || 0 })}
                />
              </div>
            </div>
            <div>
              <Label>Subject Template</Label>
              <Input
                value={editForm.subject_template}
                onChange={(e) => setEditForm({ ...editForm, subject_template: e.target.value })}
              />
            </div>
            <div>
              <Label>Body Template</Label>
              <RichTextEditor
                value={editForm.body_template}
                onChange={(md) => setEditForm({ ...editForm, body_template: md })}
              />
            </div>
            <div className="flex items-center gap-2">
              <Switch
                checked={editForm.is_active}
                onCheckedChange={(v) => setEditForm({ ...editForm, is_active: v })}
              />
              <Label>Active</Label>
            </div>
            <Button
              onClick={handleUpdate} className="w-full"
              disabled={updateLevel.isPending}
            >
              Update Level
            </Button>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  );
}
