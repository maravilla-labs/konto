import { useState } from 'react';
import { useContacts, useCreateContact } from '@/hooks/useApi';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Skeleton } from '@/components/ui/skeleton';
import { Badge } from '@/components/ui/badge';
import { StickyToolbar, type ToolbarAction, type ToolbarOverflowItem } from '@/components/ui/sticky-toolbar';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Plus, Search, Download, Building2, User } from 'lucide-react';
import { Link } from 'react-router-dom';
import { toast } from 'sonner';
import { downloadCsv } from '@/lib/export';
import { SUPPORTED_LANGUAGES } from '@/lib/language';
import { useI18n } from '@/i18n';

type CategoryFilter = 'all' | 'company' | 'person';
type CreateCategory = 'company' | 'person';

function getCategory(c: { category?: string; contact_type: string }): string {
  return c.category || (c.contact_type === 'company' ? 'company' : 'person');
}

const emptyForm = {
  name1: '',
  name2: '',
  contact_type: 'customer',
  category: 'person' as CreateCategory,
  email: '',
  phone: '',
  mobile: '',
  address: '',
  postal_code: '',
  city: '',
  country: 'CH',
  website: '',
  vat_number: '',
  language: '',
  salutation: '',
  title: '',
  industry: '',
  trade_register_number: '',
  birthday: '',
};

export function ContactsPage() {
  const { t } = useI18n();
  const [search, setSearch] = useState('');
  const [page, setPage] = useState(1);
  const [categoryFilter, setCategoryFilter] = useState<CategoryFilter>('all');
  const { data, isLoading } = useContacts({ search: search || undefined, page });
  const createContact = useCreateContact();
  const [createOpen, setCreateOpen] = useState(false);
  const [form, setForm] = useState({ ...emptyForm });

  function openCreate(category: CreateCategory) {
    setForm({ ...emptyForm, category });
    setCreateOpen(true);
  }

  function handleCreate() {
    const payload: Record<string, unknown> = {
      name1: form.name1,
      contact_type: form.contact_type,
      category: form.category,
      country: form.country || undefined,
      language: form.language || undefined,
    };
    // Common optional fields
    if (form.name2) payload.name2 = form.name2;
    if (form.email) payload.email = form.email;
    if (form.phone) payload.phone = form.phone;
    if (form.address) payload.address = form.address;
    if (form.postal_code) payload.postal_code = form.postal_code;
    if (form.city) payload.city = form.city;
    if (form.website) payload.website = form.website;
    if (form.vat_number) payload.vat_number = form.vat_number;
    // Person fields
    if (form.category === 'person') {
      if (form.salutation) payload.salutation = form.salutation;
      if (form.title) payload.title = form.title;
      if (form.mobile) payload.mobile = form.mobile;
      if (form.birthday) payload.birthday = form.birthday;
    }
    // Company fields
    if (form.category === 'company') {
      if (form.industry) payload.industry = form.industry;
      if (form.trade_register_number) payload.trade_register_number = form.trade_register_number;
    }

    createContact.mutate(payload as any, {
      onSuccess: () => {
        toast.success(t('contacts.created', 'Contact created'));
        setCreateOpen(false);
        setForm({ ...emptyForm });
      },
      onError: () => toast.error(t('contacts.create_failed', 'Failed to create contact')),
    });
  }

  const allContacts = data?.data ?? [];
  const contacts = categoryFilter === 'all'
    ? allContacts
    : allContacts.filter((c) => getCategory(c) === categoryFilter);

  const filterTabs: { key: CategoryFilter; label: string; icon: typeof Building2 }[] = [
    { key: 'all', label: t('contacts.filter.all', 'All'), icon: Search },
    { key: 'company', label: t('contacts.filter.companies', 'Companies'), icon: Building2 },
    { key: 'person', label: t('contacts.filter.persons', 'Persons'), icon: User },
  ];

  const isCompany = form.category === 'company';

  const actions: ToolbarAction[] = [
    {
      icon: <Plus className="h-4 w-4" />,
      label: t('contacts.add_contact', 'Add Contact'),
      onClick: () => openCreate('company'),
      primary: true,
    },
  ];

  const toolbarOverflow: ToolbarOverflowItem[] = [
    {
      icon: <Building2 className="h-4 w-4" />,
      label: t('contacts.new_company', 'New Company'),
      onClick: () => openCreate('company'),
    },
    {
      icon: <User className="h-4 w-4" />,
      label: t('contacts.new_person', 'New Person'),
      onClick: () => openCreate('person'),
    },
    {
      icon: <Download className="h-4 w-4" />,
      label: t('invoices.export_csv', 'Export CSV'),
      onClick: () => downloadCsv('/contacts'),
      separator: true,
    },
  ];

  return (
    <div className="space-y-4">
      <StickyToolbar actions={actions} overflow={toolbarOverflow}>
        <Badge variant="secondary">
          {t('contacts.subtitle', 'Manage customers and vendors')}
        </Badge>
      </StickyToolbar>

      {/* Create Contact Dialog */}
      <Dialog open={createOpen} onOpenChange={setCreateOpen}>
        <DialogContent className="max-w-lg">
          <DialogHeader>
            <DialogTitle className="flex items-center gap-2">
              {isCompany
                ? <><Building2 className="h-5 w-5" /> {t('contacts.create_company', 'New Company')}</>
                : <><User className="h-5 w-5" /> {t('contacts.create_person', 'New Person')}</>
              }
            </DialogTitle>
            <DialogDescription>
              {isCompany
                ? t('contacts.create_company_desc', 'Add a new company to your contacts.')
                : t('contacts.create_person_desc', 'Add a new individual person to your contacts.')
              }
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            {/* Name section */}
            {isCompany ? (
              <>
                <div>
                  <Label>{t('contacts.company_name', 'Company Name')} *</Label>
                  <Input
                    value={form.name1}
                    onChange={(e) => setForm({ ...form, name1: e.target.value })}
                    placeholder={t('contacts.company_name_placeholder', 'Acme GmbH')}
                  />
                </div>
                <div>
                  <Label>{t('contacts.name2_suffix', 'Name Suffix')}</Label>
                  <Input
                    value={form.name2}
                    onChange={(e) => setForm({ ...form, name2: e.target.value })}
                    placeholder={t('contacts.name2_suffix_placeholder', 'Department, branch, etc.')}
                  />
                </div>
              </>
            ) : (
              <>
                <div className="grid grid-cols-3 gap-3">
                  <div>
                    <Label>{t('contacts.salutation', 'Salutation')}</Label>
                    <Select
                      value={form.salutation || '__none__'}
                      onValueChange={(v) => setForm({ ...form, salutation: v === '__none__' ? '' : v })}
                    >
                      <SelectTrigger><SelectValue placeholder="—" /></SelectTrigger>
                      <SelectContent>
                        <SelectItem value="__none__">—</SelectItem>
                        <SelectItem value="Herr">{t('contacts.salutation_mr', 'Mr.')}</SelectItem>
                        <SelectItem value="Frau">{t('contacts.salutation_mrs', 'Mrs.')}</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                  <div>
                    <Label>{t('contacts.title_field', 'Title')}</Label>
                    <Input
                      value={form.title}
                      onChange={(e) => setForm({ ...form, title: e.target.value })}
                      placeholder="Dr., Prof."
                    />
                  </div>
                  <div className="col-span-1" />
                </div>
                <div className="grid grid-cols-2 gap-3">
                  <div>
                    <Label>{t('contacts.first_name', 'First Name')} *</Label>
                    <Input
                      value={form.name1}
                      onChange={(e) => setForm({ ...form, name1: e.target.value })}
                      placeholder={t('contacts.first_name_placeholder', 'John')}
                    />
                  </div>
                  <div>
                    <Label>{t('contacts.last_name', 'Last Name')}</Label>
                    <Input
                      value={form.name2}
                      onChange={(e) => setForm({ ...form, name2: e.target.value })}
                      placeholder={t('contacts.last_name_placeholder', 'Doe')}
                    />
                  </div>
                </div>
              </>
            )}

            {/* Type + Country */}
            <div className="grid grid-cols-2 gap-3">
              <div>
                <Label>{t('common.type', 'Type')}</Label>
                <Select
                  value={form.contact_type}
                  onValueChange={(v) => setForm({ ...form, contact_type: v })}
                >
                  <SelectTrigger><SelectValue /></SelectTrigger>
                  <SelectContent>
                    <SelectItem value="customer">{t('contacts.type.customer', 'Customer')}</SelectItem>
                    <SelectItem value="vendor">{t('contacts.type.vendor', 'Vendor')}</SelectItem>
                    <SelectItem value="both">{t('contacts.type.both', 'Both')}</SelectItem>
                  </SelectContent>
                </Select>
              </div>
              <div>
                <Label>{t('contacts.preferred_language', 'Language')}</Label>
                <Select
                  value={form.language || '__auto__'}
                  onValueChange={(v) => setForm({ ...form, language: v === '__auto__' ? '' : v })}
                >
                  <SelectTrigger><SelectValue placeholder={t('common.automatic', 'Automatic')} /></SelectTrigger>
                  <SelectContent>
                    <SelectItem value="__auto__">{t('common.automatic', 'Automatic')}</SelectItem>
                    {SUPPORTED_LANGUAGES.map((lang) => (
                      <SelectItem key={lang.code} value={lang.code}>
                        {lang.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            </div>

            {/* Contact info */}
            <div className="grid grid-cols-2 gap-3">
              <div>
                <Label>{t('common.email', 'Email')}</Label>
                <Input
                  value={form.email}
                  onChange={(e) => setForm({ ...form, email: e.target.value })}
                  placeholder="contact@example.com"
                  type="email"
                />
              </div>
              <div>
                <Label>{t('contacts.phone', 'Phone')}</Label>
                <Input
                  value={form.phone}
                  onChange={(e) => setForm({ ...form, phone: e.target.value })}
                  placeholder="+41 00 000 00 00"
                />
              </div>
            </div>

            {!isCompany && (
              <div className="grid grid-cols-2 gap-3">
                <div>
                  <Label>{t('contacts.mobile', 'Mobile')}</Label>
                  <Input
                    value={form.mobile}
                    onChange={(e) => setForm({ ...form, mobile: e.target.value })}
                    placeholder="+41 79 000 00 00"
                  />
                </div>
                <div>
                  <Label>{t('contacts.birthday', 'Birthday')}</Label>
                  <Input
                    type="date"
                    value={form.birthday}
                    onChange={(e) => setForm({ ...form, birthday: e.target.value })}
                  />
                </div>
              </div>
            )}

            {/* Address */}
            <div>
              <Label>{t('contacts.address', 'Address')}</Label>
              <Input
                value={form.address}
                onChange={(e) => setForm({ ...form, address: e.target.value })}
                placeholder={t('contacts.address_placeholder', 'Street and number')}
              />
            </div>
            <div className="grid grid-cols-3 gap-3">
              <div>
                <Label>{t('contacts.postal_code', 'Postal Code')}</Label>
                <Input
                  value={form.postal_code}
                  onChange={(e) => setForm({ ...form, postal_code: e.target.value })}
                  placeholder="8000"
                />
              </div>
              <div>
                <Label>{t('contacts.city', 'City')}</Label>
                <Input
                  value={form.city}
                  onChange={(e) => setForm({ ...form, city: e.target.value })}
                  placeholder="Zürich"
                />
              </div>
              <div>
                <Label>{t('contacts.country', 'Country')}</Label>
                <Input
                  value={form.country}
                  onChange={(e) => setForm({ ...form, country: e.target.value })}
                  placeholder="CH"
                />
              </div>
            </div>

            {/* Company-specific */}
            {isCompany && (
              <div className="grid grid-cols-2 gap-3">
                <div>
                  <Label>{t('contacts.website', 'Website')}</Label>
                  <Input
                    value={form.website}
                    onChange={(e) => setForm({ ...form, website: e.target.value })}
                    placeholder="https://example.com"
                  />
                </div>
                <div>
                  <Label>{t('contacts.vat_number', 'VAT Number')}</Label>
                  <Input
                    value={form.vat_number}
                    onChange={(e) => setForm({ ...form, vat_number: e.target.value })}
                    placeholder="CHE-123.456.789"
                  />
                </div>
              </div>
            )}
            {isCompany && (
              <div className="grid grid-cols-2 gap-3">
                <div>
                  <Label>{t('contacts.industry', 'Industry')}</Label>
                  <Input
                    value={form.industry}
                    onChange={(e) => setForm({ ...form, industry: e.target.value })}
                    placeholder={t('contacts.industry_placeholder', 'IT, Finance, ...')}
                  />
                </div>
                <div>
                  <Label>{t('contacts.trade_register_number', 'Trade Register No.')}</Label>
                  <Input
                    value={form.trade_register_number}
                    onChange={(e) => setForm({ ...form, trade_register_number: e.target.value })}
                    placeholder="CHE-123.456.789"
                  />
                </div>
              </div>
            )}

            <Button onClick={handleCreate} className="w-full" disabled={!form.name1 || createContact.isPending}>
              {isCompany
                ? t('contacts.create_company', 'New Company')
                : t('contacts.create_person', 'New Person')
              }
            </Button>
          </div>
        </DialogContent>
      </Dialog>

      <div className="flex items-center gap-2">
        <div className="relative flex-1">
          <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <Input
            placeholder={t('contacts.search_placeholder', 'Search contacts...')}
            value={search}
            onChange={(e) => { setSearch(e.target.value); setPage(1); }}
            className="pl-9"
          />
        </div>
        <div className="flex gap-1">
          {filterTabs.map((ft) => (
            <Button
              key={ft.key}
              variant={categoryFilter === ft.key ? 'default' : 'outline'}
              size="sm"
              onClick={() => { setCategoryFilter(ft.key); setPage(1); }}
            >
              <ft.icon className="mr-1 h-3.5 w-3.5" />
              {ft.label}
            </Button>
          ))}
        </div>
      </div>

      <Card>
        <CardContent className="p-0">
          {isLoading ? (
            <div className="space-y-2 p-4">
              {Array.from({ length: 5 }).map((_, i) => (
                <Skeleton key={i} className="h-10 w-full" />
              ))}
            </div>
          ) : contacts.length > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t('contacts.name', 'Name')}</TableHead>
                  <TableHead className="hidden lg:table-cell">{t('contacts.customer_number', 'Customer No.')}</TableHead>
                  <TableHead className="hidden sm:table-cell">{t('common.type', 'Type')}</TableHead>
                  <TableHead className="hidden md:table-cell">{t('common.email', 'Email')}</TableHead>
                  <TableHead className="hidden lg:table-cell">{t('contacts.phone', 'Phone')}</TableHead>
                  <TableHead className="hidden md:table-cell">{t('contacts.city', 'City')}</TableHead>
                  <TableHead className="hidden md:table-cell">{t('contacts.country', 'Country')}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {contacts.map((contact) => {
                  const cat = getCategory(contact);
                  return (
                    <TableRow key={contact.id}>
                      <TableCell className="font-medium">
                        <Link to={`/contacts/${contact.id}`} className="flex items-center gap-2 text-primary hover:underline">
                          {cat === 'company' ? (
                            <Building2 className="h-4 w-4 shrink-0 text-muted-foreground" />
                          ) : (
                            <User className="h-4 w-4 shrink-0 text-muted-foreground" />
                          )}
                          <div>
                            {contact.name1}
                            {contact.name2 && (
                              <span className="block text-xs text-muted-foreground">
                                {contact.name2}
                              </span>
                            )}
                          </div>
                        </Link>
                      </TableCell>
                      <TableCell className="hidden lg:table-cell font-mono text-sm text-muted-foreground">
                        {contact.customer_number || '—'}
                      </TableCell>
                      <TableCell className="hidden sm:table-cell">
                        <Badge variant="secondary">{t(`contacts.type.${contact.contact_type}`, contact.contact_type)}</Badge>
                      </TableCell>
                      <TableCell className="hidden md:table-cell text-sm text-muted-foreground">{contact.email}</TableCell>
                      <TableCell className="hidden lg:table-cell text-sm text-muted-foreground">{contact.phone}</TableCell>
                      <TableCell className="hidden md:table-cell text-sm text-muted-foreground">{contact.city}</TableCell>
                      <TableCell className="hidden md:table-cell text-sm text-muted-foreground">{contact.country}</TableCell>
                    </TableRow>
                  );
                })}
              </TableBody>
            </Table>
          ) : (
            <p className="py-8 text-center text-sm text-muted-foreground">
              {t('contacts.no_results', 'No contacts found. Add your first contact to get started.')}
            </p>
          )}
        </CardContent>
      </Card>

      {data?.total_pages && data.total_pages > 1 && (
        <div className="flex items-center justify-center gap-2">
          <Button
            variant="outline"
            size="sm"
            onClick={() => setPage((p) => Math.max(1, p - 1))}
            disabled={page <= 1}
          >
            {t('common.previous', 'Previous')}
          </Button>
          <span className="text-sm text-muted-foreground">
            {t('common.page', 'Page')} {data.page} {t('common.of', 'of')} {data.total_pages}
          </span>
          <Button
            variant="outline"
            size="sm"
            onClick={() => setPage((p) => p + 1)}
            disabled={page >= data.total_pages}
          >
            {t('common.next', 'Next')}
          </Button>
        </div>
      )}
    </div>
  );
}
