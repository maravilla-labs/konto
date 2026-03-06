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
import { useEmployees, useCreateEmployee, useUpdateEmployee, useDeleteEmployee, useRoles } from '@/hooks/useApi';
import { toast } from 'sonner';
import { Plus, Pencil, Trash2 } from 'lucide-react';
import { useI18n } from '@/i18n';
import { StickyToolbar, type ToolbarAction } from '@/components/ui/sticky-toolbar';
import type { Employee, CreateEmployeeResponse } from '@/types/employee';
import { TempPasswordDialog } from '@/components/employees/TempPasswordDialog';

const emptyForm = {
  first_name: '', last_name: '', email: '', phone: '',
  ahv_number: '', date_of_birth: '', nationality: 'CH',
  street: '', postal_code: '', city: '', country: 'CH',
  iban: '', bic: '', bank_name: '',
  employment_start: '', position: '', department: '',
  employment_percentage: 100, gross_monthly_salary: 0, annual_salary_13th: false,
  has_children: false, number_of_children: 0,
  child_allowance_amount: 215, education_allowance_amount: 268,
  bvg_insured: true, uvg_insured: true, ktg_insured: true,
  is_quellensteuer: false, quellensteuer_tariff: '', quellensteuer_rate: 0,
  marital_status: 'single', canton: 'BS', status: 'active', notes: '',
};

export function EmployeesPage() {
  const { t } = useI18n();
  const { data: employees, isLoading } = useEmployees();
  const createEmployee = useCreateEmployee();
  const updateEmployee = useUpdateEmployee();
  const deleteEmployee = useDeleteEmployee();

  const { data: roles } = useRoles();

  const [createOpen, setCreateOpen] = useState(false);
  const [createForm, setCreateForm] = useState({ ...emptyForm });
  const [createUser, setCreateUser] = useState(false);
  const [userRoleId, setUserRoleId] = useState('');
  const [tempPasswordInfo, setTempPasswordInfo] = useState<{ userId: string; password: string } | null>(null);

  const [editEmp, setEditEmp] = useState<Employee | null>(null);
  const [editForm, setEditForm] = useState({ ...emptyForm });

  function openEdit(e: Employee) {
    setEditEmp(e);
    setEditForm({
      first_name: e.first_name, last_name: e.last_name,
      email: e.email ?? '', phone: e.phone ?? '',
      ahv_number: e.ahv_number, date_of_birth: e.date_of_birth,
      nationality: e.nationality,
      street: e.street, postal_code: e.postal_code, city: e.city, country: e.country,
      iban: e.iban, bic: e.bic ?? '', bank_name: e.bank_name ?? '',
      employment_start: e.employment_start,
      position: e.position ?? '', department: e.department ?? '',
      employment_percentage: e.employment_percentage,
      gross_monthly_salary: e.gross_monthly_salary,
      annual_salary_13th: e.annual_salary_13th,
      has_children: e.has_children, number_of_children: e.number_of_children,
      child_allowance_amount: e.child_allowance_amount,
      education_allowance_amount: e.education_allowance_amount,
      bvg_insured: e.bvg_insured, uvg_insured: e.uvg_insured, ktg_insured: e.ktg_insured,
      is_quellensteuer: e.is_quellensteuer,
      quellensteuer_tariff: e.quellensteuer_tariff ?? '',
      quellensteuer_rate: e.quellensteuer_rate ?? 0,
      marital_status: e.marital_status, canton: e.canton,
      status: e.status, notes: e.notes ?? '',
    });
  }

  function handleCreate() {
    createEmployee.mutate(
      {
        first_name: createForm.first_name,
        last_name: createForm.last_name,
        email: createForm.email || undefined,
        phone: createForm.phone || undefined,
        ahv_number: createForm.ahv_number,
        date_of_birth: createForm.date_of_birth,
        nationality: createForm.nationality,
        street: createForm.street,
        postal_code: createForm.postal_code,
        city: createForm.city,
        country: createForm.country || 'CH',
        iban: createForm.iban,
        bic: createForm.bic || undefined,
        bank_name: createForm.bank_name || undefined,
        employment_start: createForm.employment_start,
        position: createForm.position || undefined,
        department: createForm.department || undefined,
        employment_percentage: createForm.employment_percentage,
        gross_monthly_salary: createForm.gross_monthly_salary,
        annual_salary_13th: createForm.annual_salary_13th,
        has_children: createForm.has_children,
        number_of_children: createForm.number_of_children,
        child_allowance_amount: createForm.child_allowance_amount,
        education_allowance_amount: createForm.education_allowance_amount,
        bvg_insured: createForm.bvg_insured,
        uvg_insured: createForm.uvg_insured,
        ktg_insured: createForm.ktg_insured,
        is_quellensteuer: createForm.is_quellensteuer,
        quellensteuer_tariff: createForm.quellensteuer_tariff || undefined,
        quellensteuer_rate: createForm.is_quellensteuer ? createForm.quellensteuer_rate : undefined,
        marital_status: createForm.marital_status,
        canton: createForm.canton,
        notes: createForm.notes || undefined,
        create_user: createUser || undefined,
        user_role_id: createUser && userRoleId ? userRoleId : undefined,
      },
      {
        onSuccess: (res) => {
          toast.success(t('employees.created'));
          setCreateOpen(false);
          setCreateForm({ ...emptyForm });
          setCreateUser(false);
          setUserRoleId('');
          const data = res.data as CreateEmployeeResponse;
          if (data.provisioned_user) {
            setTempPasswordInfo({
              userId: data.provisioned_user.user_id,
              password: data.provisioned_user.temp_password,
            });
          }
        },
        onError: () => toast.error(t('employees.create_failed')),
      },
    );
  }

  function handleUpdate() {
    if (!editEmp) return;
    updateEmployee.mutate(
      {
        id: editEmp.id,
        data: {
          first_name: editForm.first_name,
          last_name: editForm.last_name,
          email: editForm.email || null,
          phone: editForm.phone || null,
          ahv_number: editForm.ahv_number,
          date_of_birth: editForm.date_of_birth,
          nationality: editForm.nationality,
          street: editForm.street,
          postal_code: editForm.postal_code,
          city: editForm.city,
          country: editForm.country,
          iban: editForm.iban,
          bic: editForm.bic || null,
          bank_name: editForm.bank_name || null,
          employment_start: editForm.employment_start,
          employment_end: null,
          position: editForm.position || null,
          department: editForm.department || null,
          employment_percentage: editForm.employment_percentage,
          gross_monthly_salary: editForm.gross_monthly_salary,
          annual_salary_13th: editForm.annual_salary_13th,
          has_children: editForm.has_children,
          number_of_children: editForm.number_of_children,
          child_allowance_amount: editForm.child_allowance_amount,
          education_allowance_amount: editForm.education_allowance_amount,
          bvg_insured: editForm.bvg_insured,
          uvg_insured: editForm.uvg_insured,
          ktg_insured: editForm.ktg_insured,
          is_quellensteuer: editForm.is_quellensteuer,
          quellensteuer_tariff: editForm.quellensteuer_tariff || null,
          quellensteuer_rate: editForm.is_quellensteuer ? editForm.quellensteuer_rate : null,
          marital_status: editForm.marital_status,
          canton: editForm.canton,
          status: editForm.status,
          notes: editForm.notes || null,
        },
      },
      {
        onSuccess: () => { toast.success(t('employees.updated')); setEditEmp(null); },
        onError: () => toast.error(t('employees.update_failed')),
      },
    );
  }

  function handleDelete(id: string) {
    if (!confirm(t('employees.confirm_delete'))) return;
    deleteEmployee.mutate(id, {
      onSuccess: () => toast.success(t('employees.deleted')),
      onError: () => toast.error(t('employees.delete_failed')),
    });
  }

  const list = employees ?? [];

  return (
    <div className="space-y-4">
      <StickyToolbar
        actions={[
          { icon: <Plus className="h-4 w-4" />, label: t('employees.add'), onClick: () => setCreateOpen(true), primary: true },
        ] satisfies ToolbarAction[]}
      >
        <Badge variant="secondary">{list.length} {t('employees.title')}</Badge>
      </StickyToolbar>

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
                  <TableHead>{t('employees.number', 'No.')}</TableHead>
                  <TableHead>{t('employees.name')}</TableHead>
                  <TableHead>{t('employees.position')}</TableHead>
                  <TableHead className="hidden md:table-cell">{t('employees.ahv_number')}</TableHead>
                  <TableHead className="text-right">{t('employees.employment_percentage', 'Employment %')}</TableHead>
                  <TableHead className="text-right hidden sm:table-cell">{t('employees.salary')}</TableHead>
                  <TableHead>{t('common.status')}</TableHead>
                  <TableHead className="w-24">{t('common.actions')}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {list.map((e) => (
                  <TableRow key={e.id}>
                    <TableCell className="font-mono text-sm text-muted-foreground">{e.number || '—'}</TableCell>
                    <TableCell className="font-medium">{e.first_name} {e.last_name}</TableCell>
                    <TableCell className="text-muted-foreground">{e.position ?? '—'}</TableCell>
                    <TableCell className="hidden md:table-cell font-mono text-sm">{e.ahv_number}</TableCell>
                    <TableCell className="text-right font-mono">{e.employment_percentage}%</TableCell>
                    <TableCell className="text-right font-mono hidden sm:table-cell">
                      {e.gross_monthly_salary.toLocaleString('de-CH', { minimumFractionDigits: 2 })}
                    </TableCell>
                    <TableCell>
                      <Badge variant={e.status === 'active' ? 'default' : 'secondary'}>
                        {e.status === 'active' ? t('employees.status_active') : t('employees.status_terminated', 'Terminated')}
                      </Badge>
                    </TableCell>
                    <TableCell>
                      <div className="flex gap-1">
                        <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => openEdit(e)} title={t('common.edit')}>
                          <Pencil className="h-3.5 w-3.5" />
                        </Button>
                        <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => handleDelete(e.id)} title={t('common.delete')}>
                          <Trash2 className="h-3.5 w-3.5" />
                        </Button>
                      </div>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">{t('employees.no_results')}</p>
          )}
        </CardContent>
      </Card>

      {/* Create Dialog */}
      <Dialog open={createOpen} onOpenChange={setCreateOpen}>
        <DialogContent className="max-w-2xl max-h-[85vh] overflow-y-auto">
          <DialogHeader><DialogTitle>{t('employees.new')}</DialogTitle></DialogHeader>
          <EmployeeFormFields form={createForm} setForm={setCreateForm} t={t} />

          {/* Grant system access */}
          {createForm.email && (
            <div className="space-y-3 rounded-lg border p-4">
              <div className="flex items-center gap-2">
                <Switch checked={createUser} onCheckedChange={setCreateUser} />
                <Label>{t('employees.grant_system_access', 'Grant system access')}</Label>
              </div>
              <p className="text-xs text-muted-foreground">{t('employees.create_user_info', 'Creates a user account for this employee with a temporary password.')}</p>
              {createUser && roles && (
                <div>
                  <Label>{t('common.role', 'Role')}</Label>
                  <Select value={userRoleId || '__none__'} onValueChange={(v) => setUserRoleId(v === '__none__' ? '' : v)}>
                    <SelectTrigger><SelectValue placeholder={t('common.select', 'Select...')} /></SelectTrigger>
                    <SelectContent>
                      <SelectItem value="__none__">—</SelectItem>
                      {roles.map((r) => (
                        <SelectItem key={r.id} value={r.id}>{r.name}</SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
              )}
            </div>
          )}

          <Button onClick={handleCreate} className="w-full" disabled={createEmployee.isPending || !createForm.first_name || !createForm.last_name || !createForm.ahv_number || !createForm.date_of_birth}>
            {t('common.create')}
          </Button>
        </DialogContent>
      </Dialog>

      {/* Edit Dialog */}
      <Dialog open={!!editEmp} onOpenChange={(open) => !open && setEditEmp(null)}>
        <DialogContent className="max-w-2xl max-h-[85vh] overflow-y-auto">
          <DialogHeader><DialogTitle>{t('employees.edit')}: {editEmp?.first_name} {editEmp?.last_name}</DialogTitle></DialogHeader>
          <EmployeeFormFields form={editForm} setForm={setEditForm} t={t} showStatus />
          <Button onClick={handleUpdate} className="w-full" disabled={updateEmployee.isPending}>
            {t('common.update')}
          </Button>
        </DialogContent>
      </Dialog>

      {/* Temp Password Dialog */}
      <TempPasswordDialog
        open={!!tempPasswordInfo}
        onClose={() => setTempPasswordInfo(null)}
        password={tempPasswordInfo?.password ?? ''}
      />
    </div>
  );
}

function EmployeeFormFields({ form, setForm, t, showStatus }: {
  form: typeof emptyForm;
  setForm: (f: typeof emptyForm) => void;
  t: (k: string, fallback?: string) => string;
  showStatus?: boolean;
}) {
  return (
    <div className="space-y-6">
      {/* Personal */}
      <div>
        <h3 className="text-sm font-medium mb-3">{t('employees.section_personal')}</h3>
        <div className="grid gap-4 sm:grid-cols-2">
          <div><Label>{t('employees.first_name')}</Label><Input value={form.first_name} onChange={(e) => setForm({ ...form, first_name: e.target.value })} /></div>
          <div><Label>{t('employees.last_name')}</Label><Input value={form.last_name} onChange={(e) => setForm({ ...form, last_name: e.target.value })} /></div>
          <div><Label>{t('employees.email')}</Label><Input type="email" value={form.email} onChange={(e) => setForm({ ...form, email: e.target.value })} /></div>
          <div><Label>{t('employees.phone')}</Label><Input value={form.phone} onChange={(e) => setForm({ ...form, phone: e.target.value })} /></div>
          <div><Label>{t('employees.date_of_birth')}</Label><Input type="date" value={form.date_of_birth} onChange={(e) => setForm({ ...form, date_of_birth: e.target.value })} /></div>
          <div><Label>{t('employees.ahv_number')}</Label><Input value={form.ahv_number} onChange={(e) => setForm({ ...form, ahv_number: e.target.value })} placeholder="756.XXXX.XXXX.XX" /></div>
          <div><Label>{t('employees.nationality')}</Label><Input value={form.nationality} onChange={(e) => setForm({ ...form, nationality: e.target.value })} /></div>
          <div><Label>{t('employees.canton')}</Label><Input value={form.canton} onChange={(e) => setForm({ ...form, canton: e.target.value })} placeholder="e.g. BS" /></div>
          <div>
            <Label>{t('employees.marital_status')}</Label>
            <Select value={form.marital_status} onValueChange={(v) => setForm({ ...form, marital_status: v })}>
              <SelectTrigger><SelectValue /></SelectTrigger>
              <SelectContent>
                <SelectItem value="single">{t('employees.ms_single')}</SelectItem>
                <SelectItem value="married">{t('employees.ms_married')}</SelectItem>
                <SelectItem value="divorced">{t('employees.ms_divorced')}</SelectItem>
                <SelectItem value="widowed">{t('employees.ms_widowed')}</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </div>
      </div>

      {/* Address */}
      <div>
        <h3 className="text-sm font-medium mb-3">{t('employees.section_address', 'Address')}</h3>
        <div className="grid gap-4 sm:grid-cols-2">
          <div className="sm:col-span-2"><Label>{t('employees.street', 'Street')}</Label><Input value={form.street} onChange={(e) => setForm({ ...form, street: e.target.value })} /></div>
          <div><Label>{t('employees.postal_code', 'Postal Code')}</Label><Input value={form.postal_code} onChange={(e) => setForm({ ...form, postal_code: e.target.value })} /></div>
          <div><Label>{t('employees.city', 'City')}</Label><Input value={form.city} onChange={(e) => setForm({ ...form, city: e.target.value })} /></div>
          <div><Label>{t('employees.country', 'Country')}</Label><Input value={form.country} onChange={(e) => setForm({ ...form, country: e.target.value })} /></div>
        </div>
      </div>

      {/* Employment */}
      <div>
        <h3 className="text-sm font-medium mb-3">{t('employees.section_employment')}</h3>
        <div className="grid gap-4 sm:grid-cols-2">
          <div><Label>{t('employees.employment_start')}</Label><Input type="date" value={form.employment_start} onChange={(e) => setForm({ ...form, employment_start: e.target.value })} /></div>
          <div><Label>{t('employees.position')}</Label><Input value={form.position} onChange={(e) => setForm({ ...form, position: e.target.value })} /></div>
          <div><Label>{t('employees.department')}</Label><Input value={form.department} onChange={(e) => setForm({ ...form, department: e.target.value })} /></div>
          <div><Label>{t('employees.employment_percentage', 'Employment %')}</Label><Input type="number" min={0} max={100} value={form.employment_percentage} onChange={(e) => setForm({ ...form, employment_percentage: parseInt(e.target.value) || 0 })} /></div>
        </div>
      </div>

      {/* Salary & Banking */}
      <div>
        <h3 className="text-sm font-medium mb-3">{t('employees.section_salary')}</h3>
        <div className="grid gap-4 sm:grid-cols-2">
          <div><Label>{t('employees.gross_monthly_salary')}</Label><Input type="number" step="0.01" value={form.gross_monthly_salary} onChange={(e) => setForm({ ...form, gross_monthly_salary: parseFloat(e.target.value) || 0 })} /></div>
          <div className="flex items-center gap-2 pt-6">
            <Switch checked={form.annual_salary_13th} onCheckedChange={(v) => setForm({ ...form, annual_salary_13th: v })} />
            <Label>{t('employees.annual_salary_13th')}</Label>
          </div>
          <div><Label>{t('employees.iban')}</Label><Input value={form.iban} onChange={(e) => setForm({ ...form, iban: e.target.value })} placeholder="CH..." /></div>
          <div><Label>{t('employees.bank_name')}</Label><Input value={form.bank_name} onChange={(e) => setForm({ ...form, bank_name: e.target.value })} /></div>
          <div><Label>{t('employees.bic', 'BIC')}</Label><Input value={form.bic} onChange={(e) => setForm({ ...form, bic: e.target.value })} /></div>
        </div>
      </div>

      {/* Children & Allowances */}
      <div>
        <h3 className="text-sm font-medium mb-3">{t('employees.section_children', 'Children & Allowances')}</h3>
        <div className="grid gap-4 sm:grid-cols-2">
          <div className="flex items-center gap-2">
            <Switch checked={form.has_children} onCheckedChange={(v) => setForm({ ...form, has_children: v })} />
            <Label>{t('employees.has_children', 'Has Children')}</Label>
          </div>
          {form.has_children && (
            <>
              <div><Label>{t('employees.number_of_children', 'Number of Children')}</Label><Input type="number" min={0} value={form.number_of_children} onChange={(e) => setForm({ ...form, number_of_children: parseInt(e.target.value) || 0 })} /></div>
              <div><Label>{t('employees.child_allowance', 'Child Allowance (CHF)')}</Label><Input type="number" step="0.01" value={form.child_allowance_amount} onChange={(e) => setForm({ ...form, child_allowance_amount: parseFloat(e.target.value) || 0 })} /></div>
              <div><Label>{t('employees.education_allowance', 'Education Allowance (CHF)')}</Label><Input type="number" step="0.01" value={form.education_allowance_amount} onChange={(e) => setForm({ ...form, education_allowance_amount: parseFloat(e.target.value) || 0 })} /></div>
            </>
          )}
        </div>
      </div>

      {/* Insurance */}
      <div>
        <h3 className="text-sm font-medium mb-3">{t('employees.section_insurance')}</h3>
        <div className="grid gap-4 sm:grid-cols-2">
          <div className="flex items-center gap-2">
            <Switch checked={form.bvg_insured} onCheckedChange={(v) => setForm({ ...form, bvg_insured: v })} />
            <Label>{t('employees.bvg_insured')}</Label>
          </div>
          <div className="flex items-center gap-2">
            <Switch checked={form.uvg_insured} onCheckedChange={(v) => setForm({ ...form, uvg_insured: v })} />
            <Label>{t('employees.uvg_insured')}</Label>
          </div>
          <div className="flex items-center gap-2">
            <Switch checked={form.ktg_insured} onCheckedChange={(v) => setForm({ ...form, ktg_insured: v })} />
            <Label>{t('employees.ktg_insured')}</Label>
          </div>
        </div>
      </div>

      {/* Withholding Tax */}
      <div>
        <h3 className="text-sm font-medium mb-3">{t('employees.section_tax', 'Withholding Tax')}</h3>
        <div className="grid gap-4 sm:grid-cols-2">
          <div className="flex items-center gap-2">
            <Switch checked={form.is_quellensteuer} onCheckedChange={(v) => setForm({ ...form, is_quellensteuer: v })} />
            <Label>{t('employees.quellensteuer')}</Label>
          </div>
          {form.is_quellensteuer && (
            <>
              <div><Label>{t('employees.quellensteuer_tariff', 'Tariff Code')}</Label><Input value={form.quellensteuer_tariff} onChange={(e) => setForm({ ...form, quellensteuer_tariff: e.target.value })} /></div>
              <div><Label>{t('employees.quellensteuer_rate', 'Rate %')}</Label><Input type="number" step="0.01" value={form.quellensteuer_rate} onChange={(e) => setForm({ ...form, quellensteuer_rate: parseFloat(e.target.value) || 0 })} /></div>
            </>
          )}
        </div>
      </div>

      {/* Notes */}
      <div>
        <Label>{t('employees.notes')}</Label>
        <Input value={form.notes} onChange={(e) => setForm({ ...form, notes: e.target.value })} />
      </div>

      {/* Status */}
      {showStatus && (
        <div>
          <Label>{t('common.status')}</Label>
          <Select value={form.status} onValueChange={(v) => setForm({ ...form, status: v })}>
            <SelectTrigger><SelectValue /></SelectTrigger>
            <SelectContent>
              <SelectItem value="active">{t('employees.status_active')}</SelectItem>
              <SelectItem value="terminated">{t('employees.status_terminated', 'Terminated')}</SelectItem>
            </SelectContent>
          </Select>
        </div>
      )}
    </div>
  );
}
