import { useState } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Dialog, DialogContent, DialogHeader, DialogTitle,
} from '@/components/ui/dialog';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import {
  Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
} from '@/components/ui/select';
import { useVatRates, useCreateVatRate, useUpdateVatRate, useDeactivateVatRate } from '@/hooks/useApi';
import { toast } from 'sonner';
import { Plus, Pencil, XCircle } from 'lucide-react';
import { useI18n } from '@/i18n';
import type { VatRate } from '@/types/vat-rate';

export function VatRatesPage() {
  const { t } = useI18n();
  const { data: rates, isLoading } = useVatRates();
  const createRate = useCreateVatRate();
  const updateRate = useUpdateVatRate();
  const deactivateRate = useDeactivateVatRate();

  const [createOpen, setCreateOpen] = useState(false);
  const [createForm, setCreateForm] = useState({ code: '', name: '', rate: 0, vat_type: 'output', valid_from: '', valid_to: '' });

  const [editRate, setEditRate] = useState<VatRate | null>(null);
  const [editForm, setEditForm] = useState({ code: '', name: '', rate: 0, vat_type: 'output', is_active: true, valid_from: '', valid_to: '' });

  function openEdit(r: VatRate) {
    setEditRate(r);
    setEditForm({
      code: r.code, name: r.name, rate: r.rate, vat_type: r.vat_type, is_active: r.is_active,
      valid_from: r.valid_from ?? '', valid_to: r.valid_to ?? '',
    });
  }

  function handleCreate() {
    createRate.mutate(
      { ...createForm, valid_from: createForm.valid_from || undefined, valid_to: createForm.valid_to || undefined },
      {
        onSuccess: () => { toast.success('VAT rate created'); setCreateOpen(false); setCreateForm({ code: '', name: '', rate: 0, vat_type: 'output', valid_from: '', valid_to: '' }); },
        onError: () => toast.error('Failed to create VAT rate'),
      },
    );
  }

  function handleUpdate() {
    if (!editRate) return;
    updateRate.mutate(
      { id: editRate.id, data: { ...editForm, valid_from: editForm.valid_from || undefined, valid_to: editForm.valid_to || undefined } },
      {
        onSuccess: () => { toast.success('VAT rate updated'); setEditRate(null); },
        onError: () => toast.error('Failed to update'),
      },
    );
  }

  function handleDeactivate(id: string) {
    deactivateRate.mutate(id, {
      onSuccess: () => toast.success('VAT rate deactivated'),
      onError: () => toast.error('Failed to deactivate'),
    });
  }

  const list = rates ?? [];

  return (
    <div className="space-y-4">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-lg font-semibold">{t('nav.settings-vat-rates')}</h2>
          <p className="text-sm text-muted-foreground">Manage VAT / tax rate codes</p>
        </div>
        <Button size="sm" onClick={() => setCreateOpen(true)}>
          <Plus className="mr-1 h-4 w-4" /> Add VAT Rate
        </Button>
      </div>

      <Card>
        <CardContent className="p-0">
          {isLoading ? (
            <div className="space-y-2 p-4">
              {Array.from({ length: 4 }).map((_, i) => <Skeleton key={i} className="h-10 w-full" />)}
            </div>
          ) : list.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t('vat.code')}</TableHead>
                  <TableHead>{t('vat.name')}</TableHead>
                  <TableHead className="text-right">{t('vat.rate')} (%)</TableHead>
                  <TableHead>{t('vat.type')}</TableHead>
                  <TableHead className="hidden sm:table-cell">Valid From</TableHead>
                  <TableHead className="hidden sm:table-cell">Valid To</TableHead>
                  <TableHead>{t('common.status')}</TableHead>
                  <TableHead className="w-24">{t('common.actions')}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {list.map((r) => (
                  <TableRow key={r.id}>
                    <TableCell className="font-mono font-medium">{r.code}</TableCell>
                    <TableCell>{r.name}</TableCell>
                    <TableCell className="text-right font-mono">{r.rate.toFixed(2)}</TableCell>
                    <TableCell>
                      <Badge variant={r.vat_type === 'output' ? 'default' : 'outline'}>
                        {r.vat_type === 'output' ? t('vat.type_output') : t('vat.type_input')}
                      </Badge>
                    </TableCell>
                    <TableCell className="hidden sm:table-cell text-sm text-muted-foreground">{r.valid_from ?? '—'}</TableCell>
                    <TableCell className="hidden sm:table-cell text-sm text-muted-foreground">{r.valid_to ?? '—'}</TableCell>
                    <TableCell>
                      <Badge variant={r.is_active ? 'default' : 'secondary'}>
                        {r.is_active ? 'Active' : 'Inactive'}
                      </Badge>
                    </TableCell>
                    <TableCell>
                      <div className="flex gap-1">
                        <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => openEdit(r)} title="Edit">
                          <Pencil className="h-3.5 w-3.5" />
                        </Button>
                        {r.is_active && (
                          <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => handleDeactivate(r.id)} title="Deactivate">
                            <XCircle className="h-3.5 w-3.5" />
                          </Button>
                        )}
                      </div>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">No VAT rates configured.</p>
          )}
        </CardContent>
      </Card>

      {/* Create Dialog */}
      <Dialog open={createOpen} onOpenChange={setCreateOpen}>
        <DialogContent className="max-w-md">
          <DialogHeader><DialogTitle>New VAT Rate</DialogTitle></DialogHeader>
          <div className="space-y-4">
            <div className="grid gap-4 sm:grid-cols-2">
              <div><Label>{t('vat.code')}</Label><Input value={createForm.code} onChange={(e) => setCreateForm({ ...createForm, code: e.target.value })} placeholder="e.g. UN77" /></div>
              <div><Label>{t('vat.rate')} (%)</Label><Input type="number" step="0.01" value={createForm.rate} onChange={(e) => setCreateForm({ ...createForm, rate: parseFloat(e.target.value) || 0 })} /></div>
            </div>
            <div><Label>{t('vat.name')}</Label><Input value={createForm.name} onChange={(e) => setCreateForm({ ...createForm, name: e.target.value })} placeholder="e.g. Umsatzsteuer 7.7%" /></div>
            <div>
              <Label>{t('vat.type')}</Label>
              <Select value={createForm.vat_type} onValueChange={(v) => setCreateForm({ ...createForm, vat_type: v })}>
                <SelectTrigger><SelectValue /></SelectTrigger>
                <SelectContent>
                  <SelectItem value="output">{t('vat.type_output')}</SelectItem>
                  <SelectItem value="input">{t('vat.type_input')}</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div className="grid gap-4 sm:grid-cols-2">
              <div><Label>Valid From</Label><Input type="date" value={createForm.valid_from} onChange={(e) => setCreateForm({ ...createForm, valid_from: e.target.value })} /></div>
              <div><Label>Valid To</Label><Input type="date" value={createForm.valid_to} onChange={(e) => setCreateForm({ ...createForm, valid_to: e.target.value })} /></div>
            </div>
            <Button onClick={handleCreate} className="w-full" disabled={createRate.isPending || !createForm.code || !createForm.name}>{t('common.create')}</Button>
          </div>
        </DialogContent>
      </Dialog>

      {/* Edit Dialog */}
      <Dialog open={!!editRate} onOpenChange={(open) => !open && setEditRate(null)}>
        <DialogContent className="max-w-md">
          <DialogHeader><DialogTitle>Edit VAT Rate: {editRate?.code}</DialogTitle></DialogHeader>
          <div className="space-y-4">
            <div className="grid gap-4 sm:grid-cols-2">
              <div><Label>{t('vat.code')}</Label><Input value={editForm.code} onChange={(e) => setEditForm({ ...editForm, code: e.target.value })} /></div>
              <div><Label>{t('vat.rate')} (%)</Label><Input type="number" step="0.01" value={editForm.rate} onChange={(e) => setEditForm({ ...editForm, rate: parseFloat(e.target.value) || 0 })} /></div>
            </div>
            <div><Label>{t('vat.name')}</Label><Input value={editForm.name} onChange={(e) => setEditForm({ ...editForm, name: e.target.value })} /></div>
            <div>
              <Label>{t('vat.type')}</Label>
              <Select value={editForm.vat_type} onValueChange={(v) => setEditForm({ ...editForm, vat_type: v })}>
                <SelectTrigger><SelectValue /></SelectTrigger>
                <SelectContent>
                  <SelectItem value="output">{t('vat.type_output')}</SelectItem>
                  <SelectItem value="input">{t('vat.type_input')}</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div className="grid gap-4 sm:grid-cols-2">
              <div><Label>Valid From</Label><Input type="date" value={editForm.valid_from} onChange={(e) => setEditForm({ ...editForm, valid_from: e.target.value })} /></div>
              <div><Label>Valid To</Label><Input type="date" value={editForm.valid_to} onChange={(e) => setEditForm({ ...editForm, valid_to: e.target.value })} /></div>
            </div>
            <div className="flex items-center gap-2">
              <Switch checked={editForm.is_active} onCheckedChange={(v) => setEditForm({ ...editForm, is_active: v })} />
              <Label>Active</Label>
            </div>
            <Button onClick={handleUpdate} className="w-full" disabled={updateRate.isPending}>{t('common.update')}</Button>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  );
}
