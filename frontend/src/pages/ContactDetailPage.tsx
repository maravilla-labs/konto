import { useState } from 'react';
import { useParams, Link } from 'react-router-dom';
import {
  useContact,
  useContactTags,
  useAssignContactTag,
  useRemoveContactTag,
  useContactInvoices,
  useContactDocuments,
  useContactTimeEntries,
  useContactVatInfo,
  useContactPersonsViaRelationships,
} from '@/hooks/useApi';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { ArrowLeft, X, Mail, Phone, MapPin, Globe, ReceiptText, User } from 'lucide-react';
import { toast } from 'sonner';
import { useI18n } from '@/i18n';
import { ContactOverview } from '@/components/contacts/ContactOverview';
import { ContactRelationships } from '@/components/contacts/ContactRelationships';
import {
  ContactInvoicesTab,
  ContactDocumentsTab,
  ContactTimeEntriesTab,
} from '@/components/contacts/ContactDetailTabs';

type Tab = 'overview' | 'relationships' | 'invoices' | 'documents' | 'time-entries';

export function ContactDetailPage() {
  const { t } = useI18n();
  const { id } = useParams<{ id: string }>();
  const { data: contact, isLoading } = useContact(id);
  const { data: tags } = useContactTags();
  const { data: vatInfo } = useContactVatInfo(id);
  const { data: relatedPersons } = useContactPersonsViaRelationships(id);
  const assignTag = useAssignContactTag();
  const removeTag = useRemoveContactTag();
  const [tab, setTab] = useState<Tab>('overview');

  const { data: invoicesData } = useContactInvoices(tab === 'invoices' ? id : undefined);
  const { data: docsData } = useContactDocuments(tab === 'documents' ? id : undefined);
  const { data: timeData } = useContactTimeEntries(tab === 'time-entries' ? id : undefined);

  if (isLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-64 w-full" />
      </div>
    );
  }

  if (!contact) {
    return <p className="text-center text-muted-foreground py-8">Contact not found.</p>;
  }

  function handleAssignTag(tagId: string) {
    assignTag.mutate({ contactId: id!, tagId }, {
      onSuccess: () => toast.success('Tag assigned'),
      onError: () => toast.error('Failed to assign tag'),
    });
  }

  function handleRemoveTag(tagId: string) {
    removeTag.mutate({ contactId: id!, tagId }, {
      onSuccess: () => toast.success('Tag removed'),
      onError: () => toast.error('Failed to remove tag'),
    });
  }

  const isCompany = contact.category === 'company';
  const personsList = relatedPersons ?? [];

  const tabs: { key: Tab; label: string }[] = [
    { key: 'overview', label: t('contacts.tab.overview') },
    { key: 'relationships', label: t('contacts.tab.relationships') },
    { key: 'invoices', label: t('contacts.tab.invoices') },
    { key: 'documents', label: t('contacts.tab.documents') },
    { key: 'time-entries', label: t('contacts.tab.time_entries') },
  ];

  const vatModeLabel: Record<string, string> = {
    auto: t('contact_vat.auto', 'Auto'),
    normal: t('contact_vat.normal', 'Normal'),
    reverse_charge: t('contact_vat.reverse_charge', 'Reverse Charge'),
    export_exempt: t('contact_vat.export_exempt', 'Export Exempt'),
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3">
        <Link to="/contacts">
          <Button variant="ghost" size="icon"><ArrowLeft className="h-4 w-4" /></Button>
        </Link>
        <div>
          <h2 className="text-lg font-semibold">{contact.name1}</h2>
          {contact.name2 && <p className="text-sm text-muted-foreground">{contact.name2}</p>}
        </div>
        <Badge variant="secondary" className="ml-2">{contact.contact_type}</Badge>
        {contact.customer_number && (
          <span className="ml-2 font-mono text-sm text-muted-foreground">{contact.customer_number}</span>
        )}
        {!contact.is_active && <Badge variant="destructive">Inactive</Badge>}
        <div className="ml-auto">
          <Link to={`/invoices/new?contact_id=${id}`}>
            <Button size="sm" variant="outline">
              <ReceiptText className="mr-1 h-4 w-4" /> Create Invoice
            </Button>
          </Link>
        </div>
      </div>

      <div className="flex flex-wrap gap-4 text-sm text-muted-foreground">
        {contact.email && <span className="flex items-center gap-1"><Mail className="h-3.5 w-3.5" />{contact.email}</span>}
        {contact.phone && <span className="flex items-center gap-1"><Phone className="h-3.5 w-3.5" />{contact.phone}</span>}
        {contact.mobile && <span className="flex items-center gap-1"><Phone className="h-3.5 w-3.5" />{contact.mobile}</span>}
        {(contact.address || contact.city) && (
          <span className="flex items-center gap-1">
            <MapPin className="h-3.5 w-3.5" />
            {[contact.address, contact.postal_code, contact.city, contact.country].filter(Boolean).join(', ')}
          </span>
        )}
        {contact.website && <span className="flex items-center gap-1"><Globe className="h-3.5 w-3.5" />{contact.website}</span>}
      </div>

      {vatInfo && (
        <div className="flex items-center gap-2 text-sm">
          <span className="text-muted-foreground">{t('contact_vat.mode', 'VAT Mode')}:</span>
          <Badge variant="outline">
            {vatModeLabel[contact.vat_mode || 'auto'] || contact.vat_mode}
          </Badge>
          {contact.vat_mode === 'auto' && vatInfo.resolved_mode !== 'auto' && (
            <span className="text-xs text-muted-foreground">
              ({t('contact_vat.resolved_as', 'Resolved as')} {vatModeLabel[vatInfo.resolved_mode] || vatInfo.resolved_mode})
            </span>
          )}
        </div>
      )}

      <div className="flex flex-wrap items-center gap-2">
        {tags?.map((tag) => (
          <Badge
            key={tag.id}
            style={{ backgroundColor: tag.color, color: '#fff' }}
            className="cursor-pointer gap-1"
            onClick={() => handleRemoveTag(tag.id)}
          >
            {tag.name} <X className="h-3 w-3" />
          </Badge>
        ))}
        {tags && tags.length > 0 && (
          <select
            className="rounded border px-2 py-1 text-xs"
            onChange={(e) => { if (e.target.value) handleAssignTag(e.target.value); e.target.value = ''; }}
            defaultValue=""
          >
            <option value="">+ Add tag</option>
            {tags.map((tg) => <option key={tg.id} value={tg.id}>{tg.name}</option>)}
          </select>
        )}
      </div>

      <div className="flex gap-1 border-b overflow-x-auto">
        {tabs.map((tabItem) => (
          <button
            key={tabItem.key}
            className={`whitespace-nowrap px-3 py-2 text-sm font-medium border-b-2 -mb-px transition-colors ${
              tab === tabItem.key ? 'border-primary text-primary' : 'border-transparent text-muted-foreground hover:text-foreground'
            }`}
            onClick={() => setTab(tabItem.key)}
          >
            {tabItem.label}
          </button>
        ))}
      </div>

      {tab === 'overview' && (
        <div className="space-y-4">
          <ContactOverview contact={contact} />
          {isCompany && personsList.length > 0 && (
            <Card>
              <CardHeader>
                <CardTitle className="text-sm">
                  {t('contacts_browser.persons_title', 'Contact Persons')}
                  <Badge variant="secondary" className="ml-2">{personsList.length}</Badge>
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="divide-y">
                  {personsList.map((person) => (
                    <Link
                      key={person.id}
                      to={`/contacts/${person.id}`}
                      className="flex items-center gap-3 py-2 transition-colors hover:bg-muted/50 rounded px-2 -mx-2"
                    >
                      <User className="h-4 w-4 shrink-0 text-muted-foreground" />
                      <div className="min-w-0 flex-1">
                        <div className="text-sm font-medium">{person.name1}</div>
                        <div className="flex gap-3 text-xs text-muted-foreground">
                          {person.email && <span>{person.email}</span>}
                          {person.phone && <span>{person.phone}</span>}
                        </div>
                      </div>
                    </Link>
                  ))}
                </div>
              </CardContent>
            </Card>
          )}
        </div>
      )}
      {tab === 'relationships' && <ContactRelationships contact={contact} />}
      {tab === 'invoices' && <ContactInvoicesTab data={invoicesData} />}
      {tab === 'documents' && <ContactDocumentsTab data={docsData} />}
      {tab === 'time-entries' && <ContactTimeEntriesTab data={timeData} />}
    </div>
  );
}
