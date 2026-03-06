import { useState } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Dialog, DialogContent, DialogHeader, DialogTitle,
} from '@/components/ui/dialog';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import { useCurrencies, useCreateCurrency, useUpdateCurrency } from '@/hooks/useApi';
import { toast } from 'sonner';
import { Plus, Pencil } from 'lucide-react';
import { useI18n } from '@/i18n';
import { StickyToolbar, type ToolbarAction } from '@/components/ui/sticky-toolbar';
import type { Currency } from '@/types/currency';

export function CurrenciesPage() {
  const { t } = useI18n();
  const { data: currencies, isLoading } = useCurrencies();
  const createCurrency = useCreateCurrency();
  const updateCurrency = useUpdateCurrency();

  const [createOpen, setCreateOpen] = useState(false);
  const [createForm, setCreateForm] = useState({ code: '', name: '', symbol: '' });

  const [editCur, setEditCur] = useState<Currency | null>(null);
  const [editForm, setEditForm] = useState({ code: '', name: '', symbol: '' });

  function openEdit(c: Currency) {
    setEditCur(c);
    setEditForm({ code: c.code, name: c.name, symbol: c.symbol });
  }

  function handleCreate() {
    createCurrency.mutate(createForm, {
      onSuccess: () => { toast.success('Currency created'); setCreateOpen(false); setCreateForm({ code: '', name: '', symbol: '' }); },
      onError: () => toast.error('Failed to create currency'),
    });
  }

  function handleUpdate() {
    if (!editCur) return;
    updateCurrency.mutate(
      { id: editCur.id, data: editForm },
      {
        onSuccess: () => { toast.success('Currency updated'); setEditCur(null); },
        onError: () => toast.error('Failed to update'),
      },
    );
  }

  const list = currencies ?? [];

  return (
    <div className="space-y-4">
      <StickyToolbar
        actions={[
          { icon: <Plus className="h-4 w-4" />, label: t('currencies.add', 'Add Currency'), onClick: () => setCreateOpen(true), primary: true },
        ] satisfies ToolbarAction[]}
      >
        <Badge variant="secondary">{list.length} {t('currencies.title', 'Currencies')}</Badge>
      </StickyToolbar>

      <Card>
        <CardContent className="p-0">
          {isLoading ? (
            <div className="space-y-2 p-4">
              {Array.from({ length: 3 }).map((_, i) => <Skeleton key={i} className="h-10 w-full" />)}
            </div>
          ) : list.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Code</TableHead>
                  <TableHead>Name</TableHead>
                  <TableHead>Symbol</TableHead>
                  <TableHead>Primary</TableHead>
                  <TableHead className="w-16">Edit</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {list.map((c) => (
                  <TableRow key={c.id}>
                    <TableCell className="font-mono font-medium">{c.code}</TableCell>
                    <TableCell>{c.name}</TableCell>
                    <TableCell className="font-mono">{c.symbol}</TableCell>
                    <TableCell>
                      {c.is_primary && <Badge variant="default">Primary</Badge>}
                    </TableCell>
                    <TableCell>
                      <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => openEdit(c)} title="Edit">
                        <Pencil className="h-3.5 w-3.5" />
                      </Button>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">No currencies configured.</p>
          )}
        </CardContent>
      </Card>

      {/* Create Dialog */}
      <Dialog open={createOpen} onOpenChange={setCreateOpen}>
        <DialogContent className="max-w-sm">
          <DialogHeader><DialogTitle>New Currency</DialogTitle></DialogHeader>
          <div className="space-y-4">
            <div><Label>Code</Label><Input value={createForm.code} onChange={(e) => setCreateForm({ ...createForm, code: e.target.value })} placeholder="e.g. USD" /></div>
            <div><Label>Name</Label><Input value={createForm.name} onChange={(e) => setCreateForm({ ...createForm, name: e.target.value })} placeholder="e.g. US Dollar" /></div>
            <div><Label>Symbol</Label><Input value={createForm.symbol} onChange={(e) => setCreateForm({ ...createForm, symbol: e.target.value })} placeholder="e.g. $" /></div>
            <Button onClick={handleCreate} className="w-full" disabled={createCurrency.isPending || !createForm.code || !createForm.name}>Create</Button>
          </div>
        </DialogContent>
      </Dialog>

      {/* Edit Dialog */}
      <Dialog open={!!editCur} onOpenChange={(open) => !open && setEditCur(null)}>
        <DialogContent className="max-w-sm">
          <DialogHeader><DialogTitle>Edit Currency: {editCur?.code}</DialogTitle></DialogHeader>
          <div className="space-y-4">
            <div><Label>Code</Label><Input value={editForm.code} onChange={(e) => setEditForm({ ...editForm, code: e.target.value })} /></div>
            <div><Label>Name</Label><Input value={editForm.name} onChange={(e) => setEditForm({ ...editForm, name: e.target.value })} /></div>
            <div><Label>Symbol</Label><Input value={editForm.symbol} onChange={(e) => setEditForm({ ...editForm, symbol: e.target.value })} /></div>
            <Button onClick={handleUpdate} className="w-full" disabled={updateCurrency.isPending}>Update</Button>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  );
}
