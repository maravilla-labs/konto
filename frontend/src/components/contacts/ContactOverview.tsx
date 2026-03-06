import { useState, useEffect } from 'react';
import { useUpdateContact } from '@/hooks/useApi';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { RichTextEditor } from '@/components/ui/rich-text-editor';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Save } from 'lucide-react';
import { toast } from 'sonner';
import { useI18n } from '@/i18n';
import { SUPPORTED_LANGUAGES } from '@/lib/language';
import { VatModeSelector } from '@/components/contacts/VatModeSelector';
import type { Contact } from '@/types/contacts';

interface Props {
  contact: Contact;
}

export function ContactOverview({ contact }: Props) {
  const { t } = useI18n();
  const updateContact = useUpdateContact();

  const isCompany =
    contact.category === 'company' || contact.contact_type === 'company';

  // Core fields
  const [name1, setName1] = useState(contact.name1 ?? '');
  const [name2, setName2] = useState(contact.name2 ?? '');
  const [contactType, setContactType] = useState(contact.contact_type ?? 'customer');
  const [email, setEmail] = useState(contact.email ?? '');
  const [phone, setPhone] = useState(contact.phone ?? '');
  const [address, setAddress] = useState(contact.address ?? '');
  const [postalCode, setPostalCode] = useState(contact.postal_code ?? '');
  const [city, setCity] = useState(contact.city ?? '');
  const [country, setCountry] = useState(contact.country ?? '');
  const [website, setWebsite] = useState(contact.website ?? '');
  const [vatNumber, setVatNumber] = useState(contact.vat_number ?? '');
  const [language, setLanguage] = useState(contact.language ?? '');
  const [vatMode, setVatMode] = useState(contact.vat_mode ?? 'auto');

  // Secondary fields
  const [email2, setEmail2] = useState(contact.email2 ?? '');
  const [phone2, setPhone2] = useState(contact.phone2 ?? '');
  const [mobile, setMobile] = useState(contact.mobile ?? '');
  const [fax, setFax] = useState(contact.fax ?? '');
  const [notes, setNotes] = useState(contact.notes ?? '');

  // Person-specific
  const [salutation, setSalutation] = useState(contact.salutation ?? '');
  const [title, setTitle] = useState(contact.title ?? '');
  const [salutationForm, setSalutationForm] = useState(contact.salutation_form ?? '');
  const [birthday, setBirthday] = useState(contact.birthday ?? '');

  // Company-specific
  const [industry, setIndustry] = useState(contact.industry ?? '');
  const [employeeCount, setEmployeeCount] = useState(contact.employee_count?.toString() ?? '');
  const [tradeRegisterNumber, setTradeRegisterNumber] = useState(contact.trade_register_number ?? '');
  const [customerNumber, setCustomerNumber] = useState(contact.customer_number ?? '');

  useEffect(() => {
    setName1(contact.name1 ?? '');
    setName2(contact.name2 ?? '');
    setContactType(contact.contact_type ?? 'customer');
    setEmail(contact.email ?? '');
    setPhone(contact.phone ?? '');
    setAddress(contact.address ?? '');
    setPostalCode(contact.postal_code ?? '');
    setCity(contact.city ?? '');
    setCountry(contact.country ?? '');
    setWebsite(contact.website ?? '');
    setVatNumber(contact.vat_number ?? '');
    setLanguage(contact.language ?? '');
    setVatMode(contact.vat_mode ?? 'auto');
    setEmail2(contact.email2 ?? '');
    setPhone2(contact.phone2 ?? '');
    setMobile(contact.mobile ?? '');
    setFax(contact.fax ?? '');
    setNotes(contact.notes ?? '');
    setSalutation(contact.salutation ?? '');
    setTitle(contact.title ?? '');
    setSalutationForm(contact.salutation_form ?? '');
    setBirthday(contact.birthday ?? '');
    setIndustry(contact.industry ?? '');
    setEmployeeCount(contact.employee_count?.toString() ?? '');
    setTradeRegisterNumber(contact.trade_register_number ?? '');
    setCustomerNumber(contact.customer_number ?? '');
  }, [contact]);

  function handleSave() {
    updateContact.mutate(
      {
        id: contact.id,
        data: {
          name1,
          name2: name2 || null,
          contact_type: contactType,
          email: email || null,
          phone: phone || null,
          address: address || null,
          postal_code: postalCode || null,
          city: city || null,
          country: country || null,
          website: website || null,
          vat_number: vatNumber || null,
          language: language || null,
          vat_mode: vatMode || null,
          email2: email2 || null,
          phone2: phone2 || null,
          mobile: mobile || null,
          fax: fax || null,
          notes: notes || null,
          salutation: salutation || null,
          title: title || null,
          salutation_form: salutationForm || null,
          birthday: birthday || null,
          industry: industry || null,
          employee_count: employeeCount ? Number(employeeCount) : null,
          trade_register_number: tradeRegisterNumber || null,
          customer_number: customerNumber || null,
        },
      },
      {
        onSuccess: () => toast.success(t('common.saved', 'Saved')),
        onError: () => toast.error(t('common.save_failed', 'Failed to save')),
      },
    );
  }

  return (
    <div className="space-y-4">
      {/* Core info */}
      <Card>
        <CardHeader>
          <CardTitle className="text-sm">
            {isCompany ? t('contacts.company_info', 'Company Information') : t('contacts.person_info', 'Person Information')}
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-3">
          {isCompany ? (
            <>
              <div>
                <Label>{t('contacts.company_name', 'Company Name')}</Label>
                <Input value={name1} onChange={(e) => setName1(e.target.value)} />
              </div>
              <div>
                <Label>{t('contacts.name2_suffix', 'Name Suffix')}</Label>
                <Input value={name2} onChange={(e) => setName2(e.target.value)} />
              </div>
            </>
          ) : (
            <>
              <div className="grid grid-cols-3 gap-3">
                <div>
                  <Label>{t('contacts.salutation', 'Salutation')}</Label>
                  <Select
                    value={salutation || '__none__'}
                    onValueChange={(v) => setSalutation(v === '__none__' ? '' : v)}
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
                  <Input value={title} onChange={(e) => setTitle(e.target.value)} placeholder="Dr., Prof." />
                </div>
                <div>
                  <Label>{t('contacts.salutation_form', 'Salutation Form')}</Label>
                  <Input value={salutationForm} onChange={(e) => setSalutationForm(e.target.value)} />
                </div>
              </div>
              <div className="grid grid-cols-2 gap-3">
                <div>
                  <Label>{t('contacts.first_name', 'First Name')}</Label>
                  <Input value={name1} onChange={(e) => setName1(e.target.value)} />
                </div>
                <div>
                  <Label>{t('contacts.last_name', 'Last Name')}</Label>
                  <Input value={name2} onChange={(e) => setName2(e.target.value)} />
                </div>
              </div>
              <div>
                <Label>{t('contacts.birthday', 'Birthday')}</Label>
                <Input type="date" value={birthday} onChange={(e) => setBirthday(e.target.value)} />
              </div>
            </>
          )}

          <div>
            <Label>{t('contacts.customer_number', 'Customer Number')}</Label>
            <Input value={customerNumber} onChange={(e) => setCustomerNumber(e.target.value)} placeholder={t('common.optional', 'optional')} />
          </div>

          <div className="grid grid-cols-2 gap-3">
            <div>
              <Label>{t('common.type', 'Type')}</Label>
              <Select value={contactType} onValueChange={setContactType}>
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
                value={language || '__auto__'}
                onValueChange={(v) => setLanguage(v === '__auto__' ? '' : v)}
              >
                <SelectTrigger><SelectValue placeholder={t('common.automatic', 'Automatic')} /></SelectTrigger>
                <SelectContent>
                  <SelectItem value="__auto__">{t('common.automatic', 'Automatic')}</SelectItem>
                  {SUPPORTED_LANGUAGES.map((lang) => (
                    <SelectItem key={lang.code} value={lang.code}>{lang.label}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Contact details */}
      <Card>
        <CardHeader>
          <CardTitle className="text-sm">{t('contacts.contact_details', 'Contact Details')}</CardTitle>
        </CardHeader>
        <CardContent className="space-y-3">
          <div className="grid grid-cols-2 gap-3">
            <div>
              <Label>{t('common.email', 'Email')}</Label>
              <Input value={email} onChange={(e) => setEmail(e.target.value)} type="email" />
            </div>
            <div>
              <Label>{t('contacts.email2', 'Email 2')}</Label>
              <Input value={email2} onChange={(e) => setEmail2(e.target.value)} type="email" />
            </div>
          </div>
          <div className="grid grid-cols-2 gap-3">
            <div>
              <Label>{t('contacts.phone', 'Phone')}</Label>
              <Input value={phone} onChange={(e) => setPhone(e.target.value)} />
            </div>
            <div>
              <Label>{t('contacts.phone2', 'Phone 2')}</Label>
              <Input value={phone2} onChange={(e) => setPhone2(e.target.value)} />
            </div>
          </div>
          <div className="grid grid-cols-2 gap-3">
            <div>
              <Label>{t('contacts.mobile', 'Mobile')}</Label>
              <Input value={mobile} onChange={(e) => setMobile(e.target.value)} />
            </div>
            <div>
              <Label>{t('contacts.fax', 'Fax')}</Label>
              <Input value={fax} onChange={(e) => setFax(e.target.value)} />
            </div>
          </div>
          {isCompany && (
            <div>
              <Label>{t('contacts.website', 'Website')}</Label>
              <Input value={website} onChange={(e) => setWebsite(e.target.value)} placeholder="https://example.com" />
            </div>
          )}
        </CardContent>
      </Card>

      {/* Address */}
      <Card>
        <CardHeader>
          <CardTitle className="text-sm">{t('contacts.address', 'Address')}</CardTitle>
        </CardHeader>
        <CardContent className="space-y-3">
          <div>
            <Label>{t('contacts.street', 'Street')}</Label>
            <Input value={address} onChange={(e) => setAddress(e.target.value)} />
          </div>
          <div className="grid grid-cols-3 gap-3">
            <div>
              <Label>{t('contacts.postal_code', 'Postal Code')}</Label>
              <Input value={postalCode} onChange={(e) => setPostalCode(e.target.value)} />
            </div>
            <div>
              <Label>{t('contacts.city', 'City')}</Label>
              <Input value={city} onChange={(e) => setCity(e.target.value)} />
            </div>
            <div>
              <Label>{t('contacts.country', 'Country')}</Label>
              <Input value={country} onChange={(e) => setCountry(e.target.value)} />
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Company-specific */}
      {isCompany && (
        <Card>
          <CardHeader>
            <CardTitle className="text-sm">{t('contacts.business_details', 'Business Details')}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            <div className="grid grid-cols-2 gap-3">
              <div>
                <Label>{t('contacts.industry', 'Industry')}</Label>
                <Input value={industry} onChange={(e) => setIndustry(e.target.value)} />
              </div>
              <div>
                <Label>{t('contacts.employee_count', 'Employees')}</Label>
                <Input type="number" value={employeeCount} onChange={(e) => setEmployeeCount(e.target.value)} />
              </div>
            </div>
            <div className="grid grid-cols-2 gap-3">
              <div>
                <Label>{t('contacts.trade_register_number', 'Trade Register No.')}</Label>
                <Input value={tradeRegisterNumber} onChange={(e) => setTradeRegisterNumber(e.target.value)} placeholder="CHE-123.456.789" />
              </div>
              <div>
                <Label>{t('contacts.vat_number', 'VAT Number')}</Label>
                <Input value={vatNumber} onChange={(e) => setVatNumber(e.target.value)} placeholder="CHE-123.456.789 MWST" />
              </div>
            </div>
          </CardContent>
        </Card>
      )}

      {/* VAT Mode */}
      <Card>
        <CardHeader>
          <CardTitle className="text-sm">{t('contact_vat.mode', 'VAT Mode')}</CardTitle>
        </CardHeader>
        <CardContent>
          <VatModeSelector value={vatMode} onChange={setVatMode} />
        </CardContent>
      </Card>

      {/* Notes */}
      <Card>
        <CardHeader>
          <CardTitle className="text-sm">{t('common.notes', 'Notes')}</CardTitle>
        </CardHeader>
        <CardContent className="space-y-3">
          <RichTextEditor
            value={notes}
            onChange={(md) => setNotes(md)}
            placeholder={t('common.notes', 'Notes')}
          />
        </CardContent>
      </Card>

      {/* Save button */}
      <div className="flex justify-end">
        <Button onClick={handleSave} disabled={updateContact.isPending}>
          <Save className="mr-1 h-4 w-4" /> {t('common.save', 'Save')}
        </Button>
      </div>
    </div>
  );
}
