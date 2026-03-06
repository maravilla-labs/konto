import { useState } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import { Switch } from '@/components/ui/switch';
import {
  Dialog, DialogContent, DialogHeader, DialogTitle,
} from '@/components/ui/dialog';
import {
  Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
} from '@/components/ui/select';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import {
  useExpenseCategories, useCreateExpenseCategory,
  useUpdateExpenseCategory, useDeleteExpenseCategory, useAccounts,
} from '@/hooks/useApi';
import { toast } from 'sonner';
import { Plus, Pencil, Trash2 } from 'lucide-react';
import type { ExpenseCategory } from '@/types/expense';

export function ExpenseCategoriesPage() {
  const { data: categories, isLoading } = useExpenseCategories();
  const { data: accountsData } = useAccounts({ per_page: 500 });
  const createCategory = useCreateExpenseCategory();
  const updateCategory = useUpdateExpenseCategory();
  const deleteCategory = useDeleteExpenseCategory();

  const expenseAccounts = (accountsData?.data ?? []).filter(
    (a) => a.number >= 4000 && a.number <= 6999,
  );

  const [createOpen, setCreateOpen] = useState(false);
  const [editCat, setEditCat] = useState<ExpenseCategory | null>(null);

  const [createForm, setCreateForm] = useState({ name: '', account_id: '' });
  const [editForm, setEditForm] = useState({
    name: '', account_id: '', is_active: true,
  });

  function openCreate() {
    setCreateForm({ name: '', account_id: '' });
    setCreateOpen(true);
  }

  function openEdit(cat: ExpenseCategory) {
    setEditCat(cat);
    setEditForm({
      name: cat.name,
      account_id: cat.account_id ?? '',
      is_active: cat.is_active,
    });
  }

  function handleCreate() {
    createCategory.mutate(
      {
        name: createForm.name,
        account_id: createForm.account_id || undefined,
      },
      {
        onSuccess: () => { toast.success('Category created'); setCreateOpen(false); },
        onError: () => toast.error('Failed to create category'),
      },
    );
  }

  function handleUpdate() {
    if (!editCat) return;
    updateCategory.mutate(
      {
        id: editCat.id,
        data: {
          name: editForm.name,
          account_id: editForm.account_id || undefined,
          is_active: editForm.is_active,
        },
      },
      {
        onSuccess: () => { toast.success('Category updated'); setEditCat(null); },
        onError: () => toast.error('Failed to update category'),
      },
    );
  }

  function handleDelete(cat: ExpenseCategory) {
    if (!confirm(`Delete category "${cat.name}"?`)) return;
    deleteCategory.mutate(cat.id, {
      onSuccess: () => toast.success('Category deleted'),
      onError: () => toast.error('Failed to delete category'),
    });
  }

  const list = categories ?? [];

  return (
    <div className="space-y-4">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-lg font-semibold">Expense Categories</h2>
          <p className="text-sm text-muted-foreground">
            Manage expense categories and their linked accounts
          </p>
        </div>
        <Button size="sm" onClick={openCreate}>
          <Plus className="mr-1 h-4 w-4" /> Add Category
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
                  <TableHead>Name</TableHead>
                  <TableHead className="hidden sm:table-cell">Linked Account</TableHead>
                  <TableHead>Status</TableHead>
                  <TableHead className="w-28">Actions</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {list.map((c) => {
                  const acct = expenseAccounts.find((a) => a.id === c.account_id);
                  return (
                    <TableRow key={c.id}>
                      <TableCell className="font-medium">{c.name}</TableCell>
                      <TableCell className="hidden sm:table-cell text-sm text-muted-foreground">
                        {acct ? `${acct.number} — ${acct.name}` : '—'}
                      </TableCell>
                      <TableCell>
                        <Badge variant={c.is_active ? 'default' : 'secondary'}>
                          {c.is_active ? 'Active' : 'Inactive'}
                        </Badge>
                      </TableCell>
                      <TableCell>
                        <div className="flex gap-1">
                          <Button
                            variant="ghost" size="icon" className="h-8 w-8"
                            onClick={() => openEdit(c)} title="Edit category"
                          >
                            <Pencil className="h-3.5 w-3.5" />
                          </Button>
                          <Button
                            variant="ghost" size="icon" className="h-8 w-8"
                            onClick={() => handleDelete(c)} title="Delete category"
                          >
                            <Trash2 className="h-3.5 w-3.5" />
                          </Button>
                        </div>
                      </TableCell>
                    </TableRow>
                  );
                })}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">
              No expense categories yet.
            </p>
          )}
        </CardContent>
      </Card>

      {/* Create Dialog */}
      <Dialog open={createOpen} onOpenChange={setCreateOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>New Expense Category</DialogTitle>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label>Name</Label>
              <Input
                value={createForm.name}
                onChange={(e) => setCreateForm({ ...createForm, name: e.target.value })}
              />
            </div>
            <div>
              <Label>Linked Account (optional)</Label>
              <Select
                value={createForm.account_id}
                onValueChange={(v) => setCreateForm({ ...createForm, account_id: v })}
              >
                <SelectTrigger>
                  <SelectValue placeholder="Select expense account" />
                </SelectTrigger>
                <SelectContent>
                  {expenseAccounts.map((a) => (
                    <SelectItem key={a.id} value={a.id}>
                      {a.number} — {a.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <Button
              onClick={handleCreate} className="w-full"
              disabled={createCategory.isPending || !createForm.name}
            >
              Create Category
            </Button>
          </div>
        </DialogContent>
      </Dialog>

      {/* Edit Dialog */}
      <Dialog open={!!editCat} onOpenChange={(open) => !open && setEditCat(null)}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Edit Category</DialogTitle>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label>Name</Label>
              <Input
                value={editForm.name}
                onChange={(e) => setEditForm({ ...editForm, name: e.target.value })}
              />
            </div>
            <div>
              <Label>Linked Account (optional)</Label>
              <Select
                value={editForm.account_id}
                onValueChange={(v) => setEditForm({ ...editForm, account_id: v })}
              >
                <SelectTrigger>
                  <SelectValue placeholder="Select expense account" />
                </SelectTrigger>
                <SelectContent>
                  {expenseAccounts.map((a) => (
                    <SelectItem key={a.id} value={a.id}>
                      {a.number} — {a.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
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
              disabled={updateCategory.isPending || !editForm.name}
            >
              Update Category
            </Button>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  );
}
