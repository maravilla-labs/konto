import { useState } from 'react';
import { useContacts, useCreateContactRelationship } from '@/hooks/useApi';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Checkbox } from '@/components/ui/checkbox';
import { RichTextEditor } from '@/components/ui/rich-text-editor';
import { toast } from 'sonner';
import { useI18n } from '@/i18n';
import type { Contact } from '@/types/contacts';
import { Search } from 'lucide-react';

interface Props {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  contact: Contact;
}

const INITIAL_FORM = {
  role: '',
  position: '',
  department: '',
  is_primary: false,
  notes: '',
};

export function AddRelationshipDialog({ open, onOpenChange, contact }: Props) {
  const { t } = useI18n();
  const createRelationship = useCreateContactRelationship();
  const [search, setSearch] = useState('');
  const [selectedContactId, setSelectedContactId] = useState<string | null>(null);
  const [form, setForm] = useState(INITIAL_FORM);

  const isCompany = contact.category === 'company' || contact.contact_type === 'company';

  // When a company, we search for person contacts to link; vice versa
  const { data: contactsData } = useContacts({
    search: search || undefined,
    per_page: 10,
  });

  const filteredContacts = (contactsData?.data ?? []).filter(
    (c) => c.id !== contact.id,
  );

  function handleCreate() {
    if (!selectedContactId) return;

    const data = isCompany
      ? {
          person_contact_id: selectedContactId,
          role: form.role || undefined,
          position: form.position || undefined,
          department: form.department || undefined,
          is_primary: form.is_primary,
          notes: form.notes || undefined,
        }
      : {
          org_contact_id: selectedContactId,
          role: form.role || undefined,
          position: form.position || undefined,
          department: form.department || undefined,
          is_primary: form.is_primary,
          notes: form.notes || undefined,
        };

    createRelationship.mutate(
      { contactId: contact.id, data },
      {
        onSuccess: () => {
          toast.success(t('contact_relationships.created'));
          onOpenChange(false);
          setForm(INITIAL_FORM);
          setSelectedContactId(null);
          setSearch('');
        },
        onError: () => toast.error(t('contact_relationships.create_failed')),
      },
    );
  }

  const selectedContact = filteredContacts.find((c) => c.id === selectedContactId);

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>{t('contact_relationships.add')}</DialogTitle>
        </DialogHeader>
        <div className="space-y-4">
          {/* Contact search */}
          <div>
            <Label>
              {isCompany
                ? t('contact_relationships.person')
                : t('contact_relationships.organization')}
            </Label>
            {selectedContact ? (
              <div className="mt-1 flex items-center justify-between rounded border px-3 py-2 text-sm">
                <span className="font-medium">{selectedContact.name1}</span>
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => {
                    setSelectedContactId(null);
                    setSearch('');
                  }}
                >
                  {t('common.cancel')}
                </Button>
              </div>
            ) : (
              <div className="space-y-2">
                <div className="relative">
                  <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                  <Input
                    placeholder={t('contact_relationships.search_contacts')}
                    value={search}
                    onChange={(e) => setSearch(e.target.value)}
                    className="pl-9"
                  />
                </div>
                {search && filteredContacts.length > 0 && (
                  <div className="max-h-40 overflow-auto rounded border">
                    {filteredContacts.map((c) => (
                      <button
                        key={c.id}
                        type="button"
                        className="block w-full px-3 py-2 text-left text-sm hover:bg-muted"
                        onClick={() => {
                          setSelectedContactId(c.id);
                          setSearch('');
                        }}
                      >
                        <span className="font-medium">{c.name1}</span>
                        {c.name2 && (
                          <span className="ml-2 text-muted-foreground">{c.name2}</span>
                        )}
                      </button>
                    ))}
                  </div>
                )}
              </div>
            )}
          </div>

          {/* Relationship fields */}
          <div className="grid grid-cols-2 gap-3">
            <div>
              <Label>{t('contact_relationships.role')}</Label>
              <Input
                value={form.role}
                onChange={(e) => setForm({ ...form, role: e.target.value })}
                placeholder={t('contact_relationships.role')}
              />
            </div>
            <div>
              <Label>{t('contact_relationships.position')}</Label>
              <Input
                value={form.position}
                onChange={(e) => setForm({ ...form, position: e.target.value })}
                placeholder={t('contact_relationships.position')}
              />
            </div>
          </div>

          <div>
            <Label>{t('contact_relationships.department')}</Label>
            <Input
              value={form.department}
              onChange={(e) => setForm({ ...form, department: e.target.value })}
              placeholder={t('contact_relationships.department')}
            />
          </div>

          <div className="flex items-center gap-2">
            <Checkbox
              id="is_primary"
              checked={form.is_primary}
              onCheckedChange={(checked) =>
                setForm({ ...form, is_primary: checked === true })
              }
            />
            <Label htmlFor="is_primary" className="cursor-pointer">
              {t('contact_relationships.is_primary')}
            </Label>
          </div>

          <div>
            <Label>{t('contact_relationships.notes')}</Label>
            <RichTextEditor
              value={form.notes}
              onChange={(md) => setForm({ ...form, notes: md })}
            />
          </div>

          <Button
            onClick={handleCreate}
            className="w-full"
            disabled={!selectedContactId || createRelationship.isPending}
          >
            {t('contact_relationships.add')}
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
