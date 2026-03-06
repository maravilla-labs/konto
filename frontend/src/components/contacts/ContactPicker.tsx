import { useState } from 'react';
import { useContacts, useContactPersonsViaRelationships } from '@/hooks/useApi';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import { Button } from '@/components/ui/button';
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from '@/components/ui/command';
import { Building2, User, Check, ChevronsUpDown } from 'lucide-react';
import { useI18n } from '@/i18n';
import { cn } from '@/lib/utils';

interface Props {
  value?: string;
  personValue?: string;
  onChange: (contactId: string, personContactId?: string) => void;
}

export function ContactPicker({ value, personValue, onChange }: Props) {
  const { t } = useI18n();
  const [open, setOpen] = useState(false);
  const [personOpen, setPersonOpen] = useState(false);
  const [search, setSearch] = useState('');

  const { data: contactsData } = useContacts({ per_page: 200, search: search || undefined });
  const { data: persons } = useContactPersonsViaRelationships(value || undefined);

  const contacts = contactsData?.data ?? [];
  const personsList = persons ?? [];
  const selectedContact = contacts.find((c) => c.id === value);
  const selectedPerson = personsList.find((p) => p.id === personValue);
  const isCompany = selectedContact?.category === 'company';

  return (
    <div className="space-y-2">
      <Popover open={open} onOpenChange={setOpen}>
        <PopoverTrigger asChild>
          <Button
            variant="outline"
            role="combobox"
            aria-expanded={open}
            className="w-full justify-between font-normal"
          >
            {selectedContact ? (
              <span className="flex items-center gap-2 truncate">
                {selectedContact.category === 'company' ? (
                  <Building2 className="h-4 w-4 shrink-0 text-muted-foreground" />
                ) : (
                  <User className="h-4 w-4 shrink-0 text-muted-foreground" />
                )}
                {selectedContact.name1}
                {selectedContact.name2 ? ` (${selectedContact.name2})` : ''}
              </span>
            ) : (
              <span className="text-muted-foreground">
                {t('contact_picker.select_contact', 'Select contact')}
              </span>
            )}
            <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
          </Button>
        </PopoverTrigger>
        <PopoverContent className="w-[var(--radix-popover-trigger-width)] p-0" align="start">
          <Command shouldFilter={false}>
            <CommandInput
              placeholder={t('contact_picker.search', 'Search contacts...')}
              value={search}
              onValueChange={setSearch}
            />
            <CommandList>
              <CommandEmpty>{t('contacts.no_results', 'No contacts found.')}</CommandEmpty>
              {contacts.filter((c) => c.category === 'company').length > 0 && (
                <CommandGroup heading={t('contact_picker.companies', 'Companies')}>
                  {contacts
                    .filter((c) => c.category === 'company')
                    .map((c) => (
                      <CommandItem
                        key={c.id}
                        value={c.id}
                        onSelect={() => {
                          onChange(c.id, undefined);
                          setOpen(false);
                          setSearch('');
                        }}
                      >
                        <Building2 className="mr-2 h-4 w-4 text-muted-foreground" />
                        <span className="truncate">{c.name1}{c.name2 ? ` (${c.name2})` : ''}</span>
                        <Check className={cn('ml-auto h-4 w-4', value === c.id ? 'opacity-100' : 'opacity-0')} />
                      </CommandItem>
                    ))}
                </CommandGroup>
              )}
              {contacts.filter((c) => c.category === 'person').length > 0 && (
                <CommandGroup heading={t('contact_picker.persons', 'Persons')}>
                  {contacts
                    .filter((c) => c.category === 'person')
                    .map((c) => (
                      <CommandItem
                        key={c.id}
                        value={c.id}
                        onSelect={() => {
                          onChange(c.id, undefined);
                          setOpen(false);
                          setSearch('');
                        }}
                      >
                        <User className="mr-2 h-4 w-4 text-muted-foreground" />
                        <span className="truncate">{c.name1}{c.name2 ? ` (${c.name2})` : ''}</span>
                        <Check className={cn('ml-auto h-4 w-4', value === c.id ? 'opacity-100' : 'opacity-0')} />
                      </CommandItem>
                    ))}
                </CommandGroup>
              )}
            </CommandList>
          </Command>
        </PopoverContent>
      </Popover>

      {isCompany && personsList.length > 0 && (
        <Popover open={personOpen} onOpenChange={setPersonOpen}>
          <PopoverTrigger asChild>
            <Button
              variant="outline"
              role="combobox"
              aria-expanded={personOpen}
              className="w-full justify-between font-normal"
            >
              {selectedPerson ? (
                <span className="flex items-center gap-2 truncate">
                  <User className="h-4 w-4 shrink-0 text-muted-foreground" />
                  {selectedPerson.name1}
                </span>
              ) : (
                <span className="text-muted-foreground">
                  {t('contact_picker.select_person', 'Select contact person')}
                </span>
              )}
              <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
            </Button>
          </PopoverTrigger>
          <PopoverContent className="w-[var(--radix-popover-trigger-width)] p-0" align="start">
            <Command>
              <CommandInput placeholder={t('contact_picker.search', 'Search contacts...')} />
              <CommandList>
                <CommandEmpty>{t('contacts_browser.no_persons', 'No persons linked')}</CommandEmpty>
                <CommandGroup>
                  <CommandItem
                    value="__none__"
                    onSelect={() => {
                      onChange(value!, undefined);
                      setPersonOpen(false);
                    }}
                  >
                    <span className="text-muted-foreground">{t('common.none', 'None')}</span>
                    <Check className={cn('ml-auto h-4 w-4', !personValue ? 'opacity-100' : 'opacity-0')} />
                  </CommandItem>
                  {personsList.map((p) => (
                    <CommandItem
                      key={p.id}
                      value={p.id}
                      onSelect={() => {
                        onChange(value!, p.id);
                        setPersonOpen(false);
                      }}
                    >
                      <User className="mr-2 h-4 w-4 text-muted-foreground" />
                      <span className="truncate">{p.name1}</span>
                      {p.email && <span className="ml-2 text-xs text-muted-foreground">{p.email}</span>}
                      <Check className={cn('ml-auto h-4 w-4', personValue === p.id ? 'opacity-100' : 'opacity-0')} />
                    </CommandItem>
                  ))}
                </CommandGroup>
              </CommandList>
            </Command>
          </PopoverContent>
        </Popover>
      )}
    </div>
  );
}
