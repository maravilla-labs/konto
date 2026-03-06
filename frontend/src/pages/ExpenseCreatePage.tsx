import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { RichTextEditor } from '@/components/ui/rich-text-editor';
import {
  Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
} from '@/components/ui/select';
import {
  useCreateExpense, useContacts, useProjects, useExpenseCategories,
  useCurrencies, useVatRates,
} from '@/hooks/useApi';
import { toast } from 'sonner';
import { extractErrorMessage } from '@/api/client';

function today(): string {
  return new Date().toISOString().split('T')[0];
}

export function ExpenseCreatePage() {
  const navigate = useNavigate();
  const createExpense = useCreateExpense();
  const { data: contactsData } = useContacts({ per_page: 500 });
  const { data: projectsData } = useProjects({ per_page: 500 });
  const { data: categories } = useExpenseCategories();
  const { data: currencies } = useCurrencies();
  const { data: vatRates } = useVatRates();

  const contacts = contactsData?.data ?? [];
  const projects = projectsData?.data ?? [];
  const defaultCurrencyId = (currencies ?? []).find((c) => c.code === 'CHF')?.id ?? '';

  const [form, setForm] = useState({
    contact_id: '',
    category_id: '',
    description: '',
    amount: '',
    currency_id: '',
    vat_rate_id: '',
    expense_date: today(),
    due_date: '',
    project_id: '',
  });

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    createExpense.mutate(
      {
        contact_id: form.contact_id || undefined,
        category_id: form.category_id || undefined,
        description: form.description,
        amount: parseFloat(form.amount) || 0,
        currency_id: form.currency_id || defaultCurrencyId,
        vat_rate_id: form.vat_rate_id || undefined,
        expense_date: form.expense_date,
        due_date: form.due_date || undefined,
        project_id: form.project_id || undefined,
      },
      {
        onSuccess: (res) => {
          toast.success('Expense created');
          navigate(`/expenses/${res.data.id}`);
        },
        onError: (err) => toast.error(extractErrorMessage(err)),
      },
    );
  }

  return (
    <div className="space-y-4">
      <div>
        <h2 className="text-lg font-semibold">New Expense</h2>
        <p className="text-sm text-muted-foreground">Record a new expense or supplier bill</p>
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
                placeholder="Describe the expense..."
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
                  placeholder="0.00"
                  required
                />
              </div>
              <div>
                <Label>Currency</Label>
                <Select
                  value={form.currency_id || defaultCurrencyId}
                  onValueChange={(v) => setForm({ ...form, currency_id: v })}
                >
                  <SelectTrigger><SelectValue placeholder="Select currency" /></SelectTrigger>
                  <SelectContent>
                    {(currencies ?? []).map((c) => (
                      <SelectItem key={c.id} value={c.id}>{c.code} — {c.name}</SelectItem>
                    ))}
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

            <div className="grid gap-4 sm:grid-cols-3">
              <div>
                <Label>VAT Rate (optional)</Label>
                <Select
                  value={form.vat_rate_id}
                  onValueChange={(v) => setForm({ ...form, vat_rate_id: v })}
                >
                  <SelectTrigger><SelectValue placeholder="No VAT" /></SelectTrigger>
                  <SelectContent>
                    {(vatRates ?? []).filter((v) => v.is_active).map((v) => (
                      <SelectItem key={v.id} value={v.id}>{v.code} ({v.rate}%)</SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
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
              <Button type="submit" disabled={createExpense.isPending}>
                Save Expense
              </Button>
              <Button type="button" variant="outline" onClick={() => navigate('/expenses')}>
                Cancel
              </Button>
            </div>
          </form>
        </CardContent>
      </Card>
    </div>
  );
}
