import { useState } from 'react';
import { useContactRelationships, useDeleteContactRelationship } from '@/hooks/useApi';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Plus, Pencil, Trash2 } from 'lucide-react';
import { toast } from 'sonner';
import { useI18n } from '@/i18n';
import { Link } from 'react-router-dom';
import type { Contact } from '@/types/contacts';
import type { ContactRelationship } from '@/types/contact-relationship';
import { AddRelationshipDialog } from './AddRelationshipDialog';
import { EditRelationshipDialog } from './EditRelationshipDialog';

interface Props {
  contact: Contact;
}

export function ContactRelationships({ contact }: Props) {
  const { t } = useI18n();
  const { data: relationships } = useContactRelationships(contact.id);
  const deleteRelationship = useDeleteContactRelationship();
  const [addOpen, setAddOpen] = useState(false);
  const [editingRel, setEditingRel] = useState<ContactRelationship | null>(null);

  const isCompany = contact.category === 'company' || contact.contact_type === 'company';

  function handleDelete(id: string) {
    if (!window.confirm(t('contact_relationships.delete_confirm'))) return;
    deleteRelationship.mutate(id, {
      onSuccess: () => toast.success(t('contact_relationships.deleted')),
      onError: () => toast.error(t('contact_relationships.delete_failed')),
    });
  }

  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between">
        <CardTitle className="text-sm">{t('contact_relationships.title')}</CardTitle>
        <Button size="sm" onClick={() => setAddOpen(true)}>
          <Plus className="mr-1 h-4 w-4" /> {t('contact_relationships.add')}
        </Button>
      </CardHeader>
      <CardContent className="p-0">
        {relationships && relationships.length > 0 ? (
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>
                  {isCompany
                    ? t('contact_relationships.person')
                    : t('contact_relationships.organization')}
                </TableHead>
                <TableHead className="hidden sm:table-cell">
                  {t('contact_relationships.role')}
                </TableHead>
                <TableHead className="hidden md:table-cell">
                  {t('contact_relationships.position')}
                </TableHead>
                <TableHead className="hidden lg:table-cell">
                  {t('contact_relationships.department')}
                </TableHead>
                <TableHead className="w-24" />
              </TableRow>
            </TableHeader>
            <TableBody>
              {relationships.map((rel) => {
                const linkedName = isCompany ? rel.person_name : rel.org_name;
                const linkedId = isCompany ? rel.person_contact_id : rel.org_contact_id;
                return (
                  <TableRow key={rel.id}>
                    <TableCell className="font-medium">
                      <Link
                        to={`/contacts/${linkedId}`}
                        className="text-primary hover:underline"
                      >
                        {linkedName ?? linkedId}
                      </Link>
                      {rel.is_primary && (
                        <Badge variant="default" className="ml-2 text-xs">
                          {t('contact_relationships.primary')}
                        </Badge>
                      )}
                    </TableCell>
                    <TableCell className="hidden sm:table-cell">
                      {rel.role ?? '\u2014'}
                    </TableCell>
                    <TableCell className="hidden md:table-cell">
                      {rel.position ?? '\u2014'}
                    </TableCell>
                    <TableCell className="hidden lg:table-cell">
                      {rel.department ?? '\u2014'}
                    </TableCell>
                    <TableCell>
                      <div className="flex gap-1">
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => setEditingRel(rel)}
                        >
                          <Pencil className="h-3.5 w-3.5" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => handleDelete(rel.id)}
                        >
                          <Trash2 className="h-3.5 w-3.5" />
                        </Button>
                      </div>
                    </TableCell>
                  </TableRow>
                );
              })}
            </TableBody>
          </Table>
        ) : (
          <p className="py-6 text-center text-sm text-muted-foreground">
            {t('contact_relationships.no_results')}
          </p>
        )}
      </CardContent>

      <AddRelationshipDialog
        open={addOpen}
        onOpenChange={setAddOpen}
        contact={contact}
      />

      {editingRel && (
        <EditRelationshipDialog
          open={!!editingRel}
          onOpenChange={(open) => { if (!open) setEditingRel(null); }}
          relationship={editingRel}
        />
      )}
    </Card>
  );
}
