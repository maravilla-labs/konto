import { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { RichTextEditor } from '@/components/ui/rich-text-editor';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
} from '@/components/ui/select';
import {
  useExpense, useUpdateExpense, useContacts, useProjects, useExpenseCategories,
} from '@/hooks/useApi';
import { toast } from 'sonner';

export function ExpenseEditPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { data, isLoading } = useExpense(id);
  const updateExpense = useUpdateExpense();
  const { data: contactsData } = useContacts({ per_page: 500 });
  const { data: projectsData } = useProjects({ per_page: 500 });
  const { data: categories } = useExpenseCategories();

  const contacts = contactsData?.data ?? [];
  const projects = projectsData?.data ?? [];

  const [form, setForm] = useState({
    contact_id: '',
    category_id: '',
    description: '',
    amount: '',
    currency_id: '',
    vat_rate_id: '',
    expense_date: '',
    due_date: '',
    project_id: '',
  });

  useEffect(() => {
    if (data) {
      setForm({
        contact_id: data.contact_id ?? '',
        category_id: data.category_id ?? '',
        description: data.description,
        amount: data.amount,
        currency_id: data.currency_id,
        vat_rate_id: data.vat_rate_id ?? '',
        expense_date: data.expense_date,
        due_date: data.due_date ?? '',
        project_id: data.project_id ?? '',
      });
    }
  }, [data]);

  if (isLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-64 w-full" />
      </div>
    );
  }

  if (!data) {
    return <p className="text-center text-muted-foreground">Expense not found.</p>;
  }

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    updateExpense.mutate(
      {
        id: id!,
        data: {
          contact_id: form.contact_id || undefined,
          category_id: form.category_id || undefined,
          description: form.description,
          amount: parseFloat(form.amount) || 0,
          currency_id: form.currency_id || 'CHF',
          vat_rate_id: form.vat_rate_id || undefined,
          expense_date: form.expense_date,
          due_date: form.due_date || undefined,
          project_id: form.project_id || undefined,
        },
      },
      {
        onSuccess: () => {
          toast.success('Expense updated');
          navigate(`/expenses/${id}`);
        },
        onError: () => toast.error('Failed to update expense'),
      },
    );
  }

  return (
    <div className="space-y-4">
      <div>
        <h2 className="text-lg font-semibold">Edit Expense {data.expense_number}</h2>
        <p className="text-sm text-muted-foreground">Update expense details</p>
      </div>
      <Card>
        <CardHeader>
          <CardTitle className="text-base">Expense Details</CardTitle>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleSubmit} className="space-y-4">
            <div className="grid gap-4 sm:grid-cols-2">
              <div>
                <Label>Category</Label>
                <Select
                  value={form.category_id}
                  onValueChange={(v) => setForm({ ...form, category_id: v })}
                >
                  <SelectTrigger><SelectValue placeholder="Select category" /></SelectTrigger>
                  <SelectContent>
                    {(categories ?? []).filter((c) => c.is_active).map((c) => (
                      <SelectItem key={c.id} value={c.id}>{c.name}</SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              <div>
                <Label>Supplier (optional)</Label>
                <Select
                  value={form.contact_id}
                  onValueChange={(v) => setForm({ ...form, contact_id: v })}
                >
                  <SelectTrigger><SelectValue placeholder="Select supplier" /></SelectTrigger>
                  <SelectContent>
                    {contacts.map((c) => (
                      <SelectItem key={c.id} value={c.id}>{c.name1}</SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            </div>

            <div>
              <Label>Description</Label>
              <RichTextEditor
                value={form.description}
                onChange={(md) => setForm({ ...form, description: md })}
              />
            </div>

            <div className="grid gap-4 sm:grid-cols-3">
              <div>
                <Label>Amount</Label>
                <Input
                  type="number"
                  step="0.01"
                  min="0"
                  value={form.amount}
                  onChange={(e) => setForm({ ...form, amount: e.target.value })}
                  required
                />
              </div>
              <div>
                <Label>Currency</Label>
                <Select
                  value={form.currency_id || 'CHF'}
                  onValueChange={(v) => setForm({ ...form, currency_id: v })}
                >
                  <SelectTrigger><SelectValue /></SelectTrigger>
                  <SelectContent>
                    <SelectItem value="CHF">CHF</SelectItem>
                    <SelectItem value="EUR">EUR</SelectItem>
                    <SelectItem value="USD">USD</SelectItem>
                  </SelectContent>
                </Select>
              </div>
              <div>
                <Label>Expense Date</Label>
                <Input
                  type="date"
                  value={form.expense_date}
                  onChange={(e) => setForm({ ...form, expense_date: e.target.value })}
                  required
                />
              </div>
            </div>

            <div className="grid gap-4 sm:grid-cols-2">
              <div>
                <Label>Due Date (optional)</Label>
                <Input
                  type="date"
                  value={form.due_date}
                  onChange={(e) => setForm({ ...form, due_date: e.target.value })}
                />
              </div>
              <div>
                <Label>Project (optional)</Label>
                <Select
                  value={form.project_id}
                  onValueChange={(v) => setForm({ ...form, project_id: v })}
                >
                  <SelectTrigger><SelectValue placeholder="Select project" /></SelectTrigger>
                  <SelectContent>
                    {projects.map((p) => (
                      <SelectItem key={p.id} value={p.id}>{p.name}</SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            </div>

            <div className="flex gap-2">
              <Button type="submit" disabled={updateExpense.isPending}>
                Update Expense
              </Button>
              <Button type="button" variant="outline" onClick={() => navigate(`/expenses/${id}`)}>
                Cancel
              </Button>
            </div>
          </form>
        </CardContent>
      </Card>
    </div>
  );
}
