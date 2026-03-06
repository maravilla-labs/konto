import { useState } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import {
  useShareholders,
  useCreateShareholder,
  useUpdateShareholder,
  useDeleteShareholder,
} from '@/hooks/useAnnualReportApi';
import { toast } from 'sonner';
import { Plus, Pencil, Trash2 } from 'lucide-react';
import { useI18n } from '@/i18n';
import { Badge } from '@/components/ui/badge';
import { StickyToolbar, type ToolbarAction } from '@/components/ui/sticky-toolbar';
import type { Shareholder } from '@/types/annual-report';

export function ShareholdersPage() {
  const { t } = useI18n();
  const { data: shareholders, isLoading } = useShareholders();
  const createShareholder = useCreateShareholder();
  const updateShareholder = useUpdateShareholder();
  const deleteShareholder = useDeleteShareholder();

  const [dialogOpen, setDialogOpen] = useState(false);
  const [editing, setEditing] = useState<Shareholder | null>(null);
  const [form, setForm] = useState({
    name: '',
    city: '',
    role: '',
    signing_rights: '',
    sort_order: 0,
  });

  function openCreate() {
    setEditing(null);
    setForm({ name: '', city: '', role: '', signing_rights: '', sort_order: 0 });
    setDialogOpen(true);
  }

  function openEdit(sh: Shareholder) {
    setEditing(sh);
    setForm({
      name: sh.name,
      city: sh.city,
      role: sh.role,
      signing_rights: sh.signing_rights ?? '',
      sort_order: sh.sort_order,
    });
    setDialogOpen(true);
  }

  function handleSave() {
    const data = {
      ...form,
      signing_rights: form.signing_rights || null,
    };

    if (editing) {
      updateShareholder.mutate(
        { id: editing.id, data },
        {
          onSuccess: () => { toast.success('Shareholder updated'); setDialogOpen(false); },
          onError: () => toast.error('Failed to update'),
        },
      );
    } else {
      createShareholder.mutate(data, {
        onSuccess: () => { toast.success('Shareholder created'); setDialogOpen(false); },
        onError: () => toast.error('Failed to create'),
      });
    }
  }

  function handleDelete(id: string) {
    deleteShareholder.mutate(id, {
      onSuccess: () => toast.success('Shareholder deleted'),
      onError: () => toast.error('Failed to delete'),
    });
  }

  const list = shareholders ?? [];

  return (
    <div className="space-y-4">
      <StickyToolbar
        actions={[
          { icon: <Plus className="h-4 w-4" />, label: t('shareholders.add', 'Add Shareholder'), onClick: openCreate, primary: true },
        ] satisfies ToolbarAction[]}
      >
        <Badge variant="secondary">{list.length} {t('shareholders.title', 'Shareholders')}</Badge>
      </StickyToolbar>

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
                  <TableHead>Name</TableHead>
                  <TableHead>City</TableHead>
                  <TableHead className="hidden sm:table-cell">Role</TableHead>
                  <TableHead className="hidden md:table-cell">Signing Rights</TableHead>
                  <TableHead className="w-20">Actions</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {list.map((sh) => (
                  <TableRow key={sh.id}>
                    <TableCell className="font-medium">{sh.name}</TableCell>
                    <TableCell>{sh.city}</TableCell>
                    <TableCell className="hidden sm:table-cell">{sh.role}</TableCell>
                    <TableCell className="hidden md:table-cell">
                      {sh.signing_rights ?? '-'}
                    </TableCell>
                    <TableCell>
                      <div className="flex gap-1">
                        <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => openEdit(sh)}>
                          <Pencil className="h-3.5 w-3.5" />
                        </Button>
                        <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => handleDelete(sh.id)}>
                          <Trash2 className="h-3.5 w-3.5" />
                        </Button>
                      </div>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">
              No shareholders yet.
            </p>
          )}
        </CardContent>
      </Card>

      <Dialog open={dialogOpen} onOpenChange={setDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{editing ? 'Edit Shareholder' : 'New Shareholder'}</DialogTitle>
            <DialogDescription>
              {editing ? 'Update shareholder details' : 'Add a new shareholder'}
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label>Name</Label>
              <Input
                value={form.name}
                onChange={(e) => setForm({ ...form, name: e.target.value })}
                placeholder="e.g. Doe, John"
              />
            </div>
            <div>
              <Label>City</Label>
              <Input
                value={form.city}
                onChange={(e) => setForm({ ...form, city: e.target.value })}
                placeholder="e.g. Basel"
              />
            </div>
            <div>
              <Label>Role</Label>
              <Input
                value={form.role}
                onChange={(e) => setForm({ ...form, role: e.target.value })}
                placeholder="e.g. Gesellschafter und Geschäftsführer"
              />
            </div>
            <div>
              <Label>Signing Rights</Label>
              <Input
                value={form.signing_rights}
                onChange={(e) => setForm({ ...form, signing_rights: e.target.value })}
                placeholder="e.g. Einzelunterschrift"
              />
            </div>
            <div>
              <Label>Sort Order</Label>
              <Input
                type="number"
                value={form.sort_order}
                onChange={(e) => setForm({ ...form, sort_order: parseInt(e.target.value) || 0 })}
              />
            </div>
            <Button onClick={handleSave} className="w-full" disabled={!form.name || !form.city || !form.role}>
              {editing ? 'Update' : 'Create'}
            </Button>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  );
}
