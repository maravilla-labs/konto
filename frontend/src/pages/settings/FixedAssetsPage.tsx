import { useState } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import { RichTextEditor } from '@/components/ui/rich-text-editor';
import {
  Dialog, DialogContent, DialogHeader, DialogTitle,
} from '@/components/ui/dialog';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import {
  Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
} from '@/components/ui/select';
import {
  useFixedAssets, useCreateFixedAsset, useUpdateFixedAsset, useDeleteFixedAsset,
  useDepreciationSchedule, useRunDepreciation,
} from '@/hooks/useFixedAssets';
import { useFiscalYears, useAccountTree } from '@/hooks/useApi';
import { toast } from 'sonner';
import { Plus, Pencil, Trash2, Calendar, Play } from 'lucide-react';
import { useI18n } from '@/i18n';
import type { FixedAsset } from '@/types/fixed-asset';

const defaultCreate = {
  name: '', description: '', account_id: '', depreciation_account_id: '',
  acquisition_date: '', acquisition_cost: 0, residual_value: 0,
  useful_life_years: 5, depreciation_method: 'straight_line', declining_rate: 0,
};

export function FixedAssetsPage() {
  const { t } = useI18n();
  const { data: assets, isLoading } = useFixedAssets();
  const { data: fiscalYears } = useFiscalYears();
  const { data: accounts } = useAccountTree();
  const createAsset = useCreateFixedAsset();
  const updateAsset = useUpdateFixedAsset();
  const deleteAsset = useDeleteFixedAsset();
  const runDepreciation = useRunDepreciation();

  const [createOpen, setCreateOpen] = useState(false);
  const [createForm, setCreateForm] = useState(defaultCreate);

  const [editAsset, setEditAsset] = useState<FixedAsset | null>(null);
  const [editForm, setEditForm] = useState<any>({});

  const [scheduleAsset, setScheduleAsset] = useState<string | null>(null);
  const { data: schedule } = useDepreciationSchedule(scheduleAsset ?? '');

  const [runOpen, setRunOpen] = useState(false);
  const [selectedFy, setSelectedFy] = useState('');

  function flatAccounts(nodes: any[]): { id: string; number: string; name: string }[] {
    const result: any[] = [];
    for (const n of nodes) {
      result.push({ id: n.id, number: n.number, name: n.name });
      if (n.children) result.push(...flatAccounts(n.children));
    }
    return result;
  }

  const accountList = accounts ? flatAccounts(accounts) : [];

  function openEdit(a: FixedAsset) {
    setEditAsset(a);
    setEditForm({
      name: a.name, description: a.description ?? '', account_id: a.account_id,
      depreciation_account_id: a.depreciation_account_id, acquisition_date: a.acquisition_date,
      acquisition_cost: a.acquisition_cost, residual_value: a.residual_value,
      useful_life_years: a.useful_life_years, depreciation_method: a.depreciation_method,
      declining_rate: a.declining_rate ?? 0, status: a.status, disposed_date: a.disposed_date ?? '',
    });
  }

  function handleCreate() {
    createAsset.mutate(
      { ...createForm, description: createForm.description || undefined, declining_rate: createForm.depreciation_method === 'declining_balance' ? createForm.declining_rate : undefined },
      {
        onSuccess: () => { toast.success(t('fixed_assets.created')); setCreateOpen(false); setCreateForm(defaultCreate); },
        onError: () => toast.error(t('fixed_assets.create_failed')),
      },
    );
  }

  function handleUpdate() {
    if (!editAsset) return;
    updateAsset.mutate(
      {
        id: editAsset.id,
        data: { ...editForm, description: editForm.description || undefined, declining_rate: editForm.depreciation_method === 'declining_balance' ? editForm.declining_rate : undefined, disposed_date: editForm.disposed_date || undefined },
      },
      {
        onSuccess: () => { toast.success(t('fixed_assets.updated')); setEditAsset(null); },
        onError: () => toast.error(t('fixed_assets.update_failed')),
      },
    );
  }

  function handleDelete(id: string) {
    if (!confirm(t('fixed_assets.confirm_delete'))) return;
    deleteAsset.mutate(id, {
      onSuccess: () => toast.success(t('fixed_assets.deleted')),
      onError: () => toast.error(t('fixed_assets.delete_failed')),
    });
  }

  function handleRunDepreciation() {
    if (!selectedFy) return;
    runDepreciation.mutate(
      { fiscal_year_id: selectedFy },
      {
        onSuccess: (res) => {
          const count = res.data?.length ?? 0;
          toast.success(t('fixed_assets.depreciation_run_success').replace('{n}', String(count)));
          setRunOpen(false);
        },
        onError: () => toast.error(t('fixed_assets.depreciation_run_failed')),
      },
    );
  }

  const statusColor = (s: string) => {
    switch (s) {
      case 'active': return 'default';
      case 'fully_depreciated': return 'secondary';
      case 'disposed': return 'outline';
      default: return 'default';
    }
  };

  const list = assets ?? [];
  const fyList = fiscalYears?.data ?? fiscalYears ?? [];

  return (
    <div className="space-y-4">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-lg font-semibold">{t('fixed_assets.title')}</h2>
          <p className="text-sm text-muted-foreground">{t('fixed_assets.subtitle')}</p>
        </div>
        <div className="flex gap-2">
          <Button size="sm" variant="outline" onClick={() => setRunOpen(true)}>
            <Play className="mr-1 h-4 w-4" /> {t('fixed_assets.run_depreciation')}
          </Button>
          <Button size="sm" onClick={() => setCreateOpen(true)}>
            <Plus className="mr-1 h-4 w-4" /> {t('fixed_assets.add')}
          </Button>
        </div>
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
                  <TableHead>{t('fixed_assets.name')}</TableHead>
                  <TableHead className="text-right">{t('fixed_assets.acquisition_cost')}</TableHead>
                  <TableHead className="text-right">{t('fixed_assets.residual_value')}</TableHead>
                  <TableHead>{t('fixed_assets.method')}</TableHead>
                  <TableHead className="text-right">{t('fixed_assets.useful_life')}</TableHead>
                  <TableHead>{t('common.status')}</TableHead>
                  <TableHead className="w-32">{t('common.actions')}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {list.map((a) => (
                  <TableRow key={a.id}>
                    <TableCell className="font-medium">{a.name}</TableCell>
                    <TableCell className="text-right font-mono">{a.acquisition_cost.toFixed(2)}</TableCell>
                    <TableCell className="text-right font-mono">{a.residual_value.toFixed(2)}</TableCell>
                    <TableCell>
                      <Badge variant="outline">
                        {a.depreciation_method === 'straight_line' ? t('fixed_assets.straight_line') : t('fixed_assets.declining_balance')}
                      </Badge>
                    </TableCell>
                    <TableCell className="text-right">{a.useful_life_years} {t('fixed_assets.years')}</TableCell>
                    <TableCell>
                      <Badge variant={statusColor(a.status) as any}>{a.status}</Badge>
                    </TableCell>
                    <TableCell>
                      <div className="flex gap-1">
                        <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => setScheduleAsset(a.id)} title={t('fixed_assets.schedule')}>
                          <Calendar className="h-3.5 w-3.5" />
                        </Button>
                        <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => openEdit(a)} title={t('common.update')}>
                          <Pencil className="h-3.5 w-3.5" />
                        </Button>
                        <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => handleDelete(a.id)} title={t('common.delete')}>
                          <Trash2 className="h-3.5 w-3.5" />
                        </Button>
                      </div>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">{t('fixed_assets.no_results')}</p>
          )}
        </CardContent>
      </Card>

      {/* Create Dialog */}
      <Dialog open={createOpen} onOpenChange={setCreateOpen}>
        <DialogContent className="max-w-lg max-h-[90vh] overflow-y-auto">
          <DialogHeader><DialogTitle>{t('fixed_assets.new')}</DialogTitle></DialogHeader>
          <div className="space-y-4">
            <div><Label>{t('fixed_assets.name')}</Label><Input value={createForm.name} onChange={(e) => setCreateForm({ ...createForm, name: e.target.value })} /></div>
            <div><Label>{t('common.description')}</Label><RichTextEditor value={createForm.description} onChange={(md) => setCreateForm({ ...createForm, description: md })} /></div>
            <div className="grid gap-4 sm:grid-cols-2">
              <div>
                <Label>{t('fixed_assets.asset_account')}</Label>
                <Select value={createForm.account_id} onValueChange={(v) => setCreateForm({ ...createForm, account_id: v })}>
                  <SelectTrigger><SelectValue placeholder={t('fixed_assets.select_account')} /></SelectTrigger>
                  <SelectContent>{accountList.map((a) => <SelectItem key={a.id} value={a.id}>{a.number} — {a.name}</SelectItem>)}</SelectContent>
                </Select>
              </div>
              <div>
                <Label>{t('fixed_assets.depreciation_account')}</Label>
                <Select value={createForm.depreciation_account_id} onValueChange={(v) => setCreateForm({ ...createForm, depreciation_account_id: v })}>
                  <SelectTrigger><SelectValue placeholder={t('fixed_assets.select_account')} /></SelectTrigger>
                  <SelectContent>{accountList.map((a) => <SelectItem key={a.id} value={a.id}>{a.number} — {a.name}</SelectItem>)}</SelectContent>
                </Select>
              </div>
            </div>
            <div className="grid gap-4 sm:grid-cols-2">
              <div><Label>{t('fixed_assets.acquisition_date')}</Label><Input type="date" value={createForm.acquisition_date} onChange={(e) => setCreateForm({ ...createForm, acquisition_date: e.target.value })} /></div>
              <div><Label>{t('fixed_assets.acquisition_cost')}</Label><Input type="number" step="0.01" value={createForm.acquisition_cost} onChange={(e) => setCreateForm({ ...createForm, acquisition_cost: parseFloat(e.target.value) || 0 })} /></div>
            </div>
            <div className="grid gap-4 sm:grid-cols-2">
              <div><Label>{t('fixed_assets.residual_value')}</Label><Input type="number" step="0.01" value={createForm.residual_value} onChange={(e) => setCreateForm({ ...createForm, residual_value: parseFloat(e.target.value) || 0 })} /></div>
              <div><Label>{t('fixed_assets.useful_life')}</Label><Input type="number" value={createForm.useful_life_years} onChange={(e) => setCreateForm({ ...createForm, useful_life_years: parseInt(e.target.value) || 1 })} /></div>
            </div>
            <div className="grid gap-4 sm:grid-cols-2">
              <div>
                <Label>{t('fixed_assets.method')}</Label>
                <Select value={createForm.depreciation_method} onValueChange={(v) => setCreateForm({ ...createForm, depreciation_method: v })}>
                  <SelectTrigger><SelectValue /></SelectTrigger>
                  <SelectContent>
                    <SelectItem value="straight_line">{t('fixed_assets.straight_line')}</SelectItem>
                    <SelectItem value="declining_balance">{t('fixed_assets.declining_balance')}</SelectItem>
                  </SelectContent>
                </Select>
              </div>
              {createForm.depreciation_method === 'declining_balance' && (
                <div><Label>{t('fixed_assets.declining_rate')}</Label><Input type="number" step="0.01" value={createForm.declining_rate} onChange={(e) => setCreateForm({ ...createForm, declining_rate: parseFloat(e.target.value) || 0 })} /></div>
              )}
            </div>
            <Button onClick={handleCreate} className="w-full" disabled={createAsset.isPending || !createForm.name || !createForm.account_id}>{t('common.create')}</Button>
          </div>
        </DialogContent>
      </Dialog>

      {/* Edit Dialog */}
      <Dialog open={!!editAsset} onOpenChange={(open) => !open && setEditAsset(null)}>
        <DialogContent className="max-w-lg max-h-[90vh] overflow-y-auto">
          <DialogHeader><DialogTitle>{t('fixed_assets.edit')}: {editAsset?.name}</DialogTitle></DialogHeader>
          <div className="space-y-4">
            <div><Label>{t('fixed_assets.name')}</Label><Input value={editForm.name ?? ''} onChange={(e) => setEditForm({ ...editForm, name: e.target.value })} /></div>
            <div><Label>{t('common.description')}</Label><RichTextEditor value={editForm.description ?? ''} onChange={(md) => setEditForm({ ...editForm, description: md })} /></div>
            <div className="grid gap-4 sm:grid-cols-2">
              <div>
                <Label>{t('fixed_assets.asset_account')}</Label>
                <Select value={editForm.account_id ?? ''} onValueChange={(v) => setEditForm({ ...editForm, account_id: v })}>
                  <SelectTrigger><SelectValue /></SelectTrigger>
                  <SelectContent>{accountList.map((a) => <SelectItem key={a.id} value={a.id}>{a.number} — {a.name}</SelectItem>)}</SelectContent>
                </Select>
              </div>
              <div>
                <Label>{t('fixed_assets.depreciation_account')}</Label>
                <Select value={editForm.depreciation_account_id ?? ''} onValueChange={(v) => setEditForm({ ...editForm, depreciation_account_id: v })}>
                  <SelectTrigger><SelectValue /></SelectTrigger>
                  <SelectContent>{accountList.map((a) => <SelectItem key={a.id} value={a.id}>{a.number} — {a.name}</SelectItem>)}</SelectContent>
                </Select>
              </div>
            </div>
            <div className="grid gap-4 sm:grid-cols-2">
              <div><Label>{t('fixed_assets.acquisition_date')}</Label><Input type="date" value={editForm.acquisition_date ?? ''} onChange={(e) => setEditForm({ ...editForm, acquisition_date: e.target.value })} /></div>
              <div><Label>{t('fixed_assets.acquisition_cost')}</Label><Input type="number" step="0.01" value={editForm.acquisition_cost ?? 0} onChange={(e) => setEditForm({ ...editForm, acquisition_cost: parseFloat(e.target.value) || 0 })} /></div>
            </div>
            <div className="grid gap-4 sm:grid-cols-2">
              <div><Label>{t('fixed_assets.residual_value')}</Label><Input type="number" step="0.01" value={editForm.residual_value ?? 0} onChange={(e) => setEditForm({ ...editForm, residual_value: parseFloat(e.target.value) || 0 })} /></div>
              <div><Label>{t('fixed_assets.useful_life')}</Label><Input type="number" value={editForm.useful_life_years ?? 1} onChange={(e) => setEditForm({ ...editForm, useful_life_years: parseInt(e.target.value) || 1 })} /></div>
            </div>
            <div className="grid gap-4 sm:grid-cols-2">
              <div>
                <Label>{t('fixed_assets.method')}</Label>
                <Select value={editForm.depreciation_method ?? 'straight_line'} onValueChange={(v) => setEditForm({ ...editForm, depreciation_method: v })}>
                  <SelectTrigger><SelectValue /></SelectTrigger>
                  <SelectContent>
                    <SelectItem value="straight_line">{t('fixed_assets.straight_line')}</SelectItem>
                    <SelectItem value="declining_balance">{t('fixed_assets.declining_balance')}</SelectItem>
                  </SelectContent>
                </Select>
              </div>
              {editForm.depreciation_method === 'declining_balance' && (
                <div><Label>{t('fixed_assets.declining_rate')}</Label><Input type="number" step="0.01" value={editForm.declining_rate ?? 0} onChange={(e) => setEditForm({ ...editForm, declining_rate: parseFloat(e.target.value) || 0 })} /></div>
              )}
            </div>
            <div className="grid gap-4 sm:grid-cols-2">
              <div>
                <Label>{t('common.status')}</Label>
                <Select value={editForm.status ?? 'active'} onValueChange={(v) => setEditForm({ ...editForm, status: v })}>
                  <SelectTrigger><SelectValue /></SelectTrigger>
                  <SelectContent>
                    <SelectItem value="active">{t('fixed_assets.status_active')}</SelectItem>
                    <SelectItem value="fully_depreciated">{t('fixed_assets.status_fully_depreciated')}</SelectItem>
                    <SelectItem value="disposed">{t('fixed_assets.status_disposed')}</SelectItem>
                  </SelectContent>
                </Select>
              </div>
              {editForm.status === 'disposed' && (
                <div><Label>{t('fixed_assets.disposed_date')}</Label><Input type="date" value={editForm.disposed_date ?? ''} onChange={(e) => setEditForm({ ...editForm, disposed_date: e.target.value })} /></div>
              )}
            </div>
            <Button onClick={handleUpdate} className="w-full" disabled={updateAsset.isPending}>{t('common.update')}</Button>
          </div>
        </DialogContent>
      </Dialog>

      {/* Depreciation Schedule Dialog */}
      <Dialog open={!!scheduleAsset} onOpenChange={(open) => !open && setScheduleAsset(null)}>
        <DialogContent className="max-w-2xl max-h-[90vh] overflow-y-auto">
          <DialogHeader><DialogTitle>{t('fixed_assets.depreciation_schedule')}</DialogTitle></DialogHeader>
          {schedule && schedule.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t('fixed_assets.period')}</TableHead>
                  <TableHead className="text-right">{t('fixed_assets.dep_amount')}</TableHead>
                  <TableHead className="text-right">{t('fixed_assets.accumulated')}</TableHead>
                  <TableHead className="text-right">{t('fixed_assets.book_value')}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {schedule.map((e) => (
                  <TableRow key={e.id}>
                    <TableCell>{e.period_date}</TableCell>
                    <TableCell className="text-right font-mono">{e.amount.toFixed(2)}</TableCell>
                    <TableCell className="text-right font-mono">{e.accumulated.toFixed(2)}</TableCell>
                    <TableCell className="text-right font-mono">{e.book_value.toFixed(2)}</TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-4 text-center text-sm text-muted-foreground">{t('fixed_assets.no_depreciation_entries')}</p>
          )}
        </DialogContent>
      </Dialog>

      {/* Run Depreciation Dialog */}
      <Dialog open={runOpen} onOpenChange={setRunOpen}>
        <DialogContent className="max-w-md">
          <DialogHeader><DialogTitle>{t('fixed_assets.run_depreciation')}</DialogTitle></DialogHeader>
          <div className="space-y-4">
            <p className="text-sm text-muted-foreground">{t('fixed_assets.run_depreciation_desc')}</p>
            <div>
              <Label>{t('fixed_assets.fiscal_year')}</Label>
              <Select value={selectedFy} onValueChange={setSelectedFy}>
                <SelectTrigger><SelectValue placeholder={t('fixed_assets.select_fiscal_year')} /></SelectTrigger>
                <SelectContent>
                  {(Array.isArray(fyList) ? fyList : []).map((fy: any) => (
                    <SelectItem key={fy.id} value={fy.id}>{fy.name}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <Button onClick={handleRunDepreciation} className="w-full" disabled={runDepreciation.isPending || !selectedFy}>
              <Play className="mr-1 h-4 w-4" /> {t('fixed_assets.run_depreciation')}
            </Button>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  );
}
