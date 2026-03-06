import { useState } from 'react';
import { useContacts } from '@/hooks/useApi';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import { Search, Building2, User } from 'lucide-react';
import { Link } from 'react-router-dom';
import { useI18n } from '@/i18n';

type CategoryFilter = 'all' | 'company' | 'person';

interface Props {
  onSelectCompany: (id: string) => void;
  selectedCompanyId: string | null;
}

export function ContactBrowserPanel({ onSelectCompany, selectedCompanyId }: Props) {
  const { t } = useI18n();
  const [search, setSearch] = useState('');
  const [page, setPage] = useState(1);
  const [categoryFilter, setCategoryFilter] = useState<CategoryFilter>('all');
  const { data, isLoading } = useContacts({ search: search || undefined, page });

  const allContacts = data?.data ?? [];
  // Derive effective category: use category field, fallback to contact_type for legacy imports
  const getCategory = (c: { category?: string; contact_type: string }) =>
    c.category || (c.contact_type === 'company' ? 'company' : 'person');
  const contacts = categoryFilter === 'all'
    ? allContacts
    : allContacts.filter((c) => getCategory(c) === categoryFilter);

  const filterTabs: { key: CategoryFilter; label: string }[] = [
    { key: 'all', label: t('contacts.filter.all', 'All') },
    { key: 'company', label: t('contacts.filter.companies', 'Companies') },
    { key: 'person', label: t('contacts.filter.persons', 'Persons') },
  ];

  return (
    <div className="flex h-full flex-col">
      <div className="space-y-3 border-b p-3">
        <div className="relative">
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
              className="text-xs"
            >
              {ft.label}
            </Button>
          ))}
        </div>
      </div>

      <div className="flex-1 overflow-y-auto">
        {isLoading ? (
          <div className="space-y-2 p-3">
            {Array.from({ length: 8 }).map((_, i) => (
              <Skeleton key={i} className="h-12 w-full" />
            ))}
          </div>
        ) : contacts.length === 0 ? (
          <p className="py-8 text-center text-sm text-muted-foreground">
            {t('contacts.no_results', 'No contacts found.')}
          </p>
        ) : (
          <div className="divide-y">
            {contacts.map((contact) => {
              const isCompany = getCategory(contact) === 'company';
              const isSelected = isCompany && selectedCompanyId === contact.id;

              if (isCompany) {
                return (
                  <button
                    key={contact.id}
                    className={`flex w-full items-center gap-3 px-3 py-2.5 text-left transition-colors hover:bg-muted/50 ${
                      isSelected ? 'bg-muted border-l-2 border-l-primary' : ''
                    }`}
                    onClick={() => onSelectCompany(contact.id)}
                  >
                    <Building2 className="h-4 w-4 shrink-0 text-muted-foreground" />
                    <div className="min-w-0 flex-1">
                      <div className="truncate text-sm font-medium">{contact.name1}</div>
                      <div className="flex items-center gap-2 text-xs text-muted-foreground">
                        {contact.email && <span className="truncate">{contact.email}</span>}
                        {contact.country && <span>{contact.country}</span>}
                      </div>
                    </div>
                    <Badge variant="secondary" className="shrink-0 text-xs">
                      {t(`contacts.type.${contact.contact_type}`, contact.contact_type)}
                    </Badge>
                  </button>
                );
              }

              return (
                <Link
                  key={contact.id}
                  to={`/contacts/${contact.id}`}
                  className="flex items-center gap-3 px-3 py-2.5 transition-colors hover:bg-muted/50"
                >
                  <User className="h-4 w-4 shrink-0 text-muted-foreground" />
                  <div className="min-w-0 flex-1">
                    <div className="truncate text-sm font-medium">{contact.name1}</div>
                    <div className="flex items-center gap-2 text-xs text-muted-foreground">
                      {contact.email && <span className="truncate">{contact.email}</span>}
                      {contact.country && <span>{contact.country}</span>}
                    </div>
                  </div>
                  <Badge variant="outline" className="shrink-0 text-xs">
                    {t(`contacts.type.${contact.contact_type}`, contact.contact_type)}
                  </Badge>
                </Link>
              );
            })}
          </div>
        )}
      </div>

      {data?.total_pages && data.total_pages > 1 && (
        <div className="flex items-center justify-center gap-2 border-t p-2">
          <Button
            variant="outline"
            size="sm"
            onClick={() => setPage((p) => Math.max(1, p - 1))}
            disabled={page <= 1}
          >
            {t('common.previous', 'Previous')}
          </Button>
          <span className="text-xs text-muted-foreground">
            {data.page} / {data.total_pages}
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
