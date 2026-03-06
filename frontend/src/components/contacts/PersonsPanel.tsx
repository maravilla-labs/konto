import { useContactPersonsViaRelationships } from '@/hooks/useApi';
import { Card, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import { Users, Mail, Phone, User } from 'lucide-react';
import { Link } from 'react-router-dom';
import { useI18n } from '@/i18n';

interface Props {
  companyId: string | null;
}

export function PersonsPanel({ companyId }: Props) {
  const { t } = useI18n();
  const { data: persons, isLoading } = useContactPersonsViaRelationships(companyId || undefined);

  if (!companyId) {
    return (
      <div className="flex h-full flex-col items-center justify-center gap-3 p-6 text-center">
        <Users className="h-10 w-10 text-muted-foreground/40" />
        <p className="text-sm text-muted-foreground">
          {t('contacts_browser.select_company', 'Select a company to view its contacts')}
        </p>
      </div>
    );
  }

  if (isLoading) {
    return (
      <div className="space-y-3 p-4">
        <Skeleton className="h-6 w-32" />
        {Array.from({ length: 3 }).map((_, i) => (
          <Skeleton key={i} className="h-16 w-full" />
        ))}
      </div>
    );
  }

  const personsList = persons ?? [];

  return (
    <div className="flex h-full flex-col">
      <div className="border-b px-4 py-3">
        <h3 className="text-sm font-medium">
          {t('contacts_browser.persons_title', 'Contact Persons')}
          {personsList.length > 0 && (
            <Badge variant="secondary" className="ml-2">{personsList.length}</Badge>
          )}
        </h3>
      </div>
      <div className="flex-1 overflow-y-auto">
        {personsList.length === 0 ? (
          <div className="flex flex-col items-center justify-center gap-2 p-6 text-center">
            <User className="h-8 w-8 text-muted-foreground/40" />
            <p className="text-sm text-muted-foreground">
              {t('contacts_browser.no_persons', 'No persons linked to this company')}
            </p>
          </div>
        ) : (
          <div className="divide-y p-2">
            {personsList.map((person) => (
              <Link
                key={person.id}
                to={`/contacts/${person.id}`}
                className="block rounded-md p-3 transition-colors hover:bg-muted/50"
              >
                <Card className="border-0 shadow-none">
                  <CardContent className="p-0">
                    <div className="font-medium text-sm">{person.name1}</div>
                    {person.name2 && (
                      <div className="text-xs text-muted-foreground">{person.name2}</div>
                    )}
                    <div className="mt-1.5 flex flex-wrap gap-3 text-xs text-muted-foreground">
                      {person.email && (
                        <span className="flex items-center gap-1">
                          <Mail className="h-3 w-3" /> {person.email}
                        </span>
                      )}
                      {person.phone && (
                        <span className="flex items-center gap-1">
                          <Phone className="h-3 w-3" /> {person.phone}
                        </span>
                      )}
                    </div>
                  </CardContent>
                </Card>
              </Link>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
