import { useState, useEffect } from 'react';
import { useUpdateContactRelationship } from '@/hooks/useApi';
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
import type { ContactRelationship } from '@/types/contact-relationship';

interface Props {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  relationship: ContactRelationship;
}

export function EditRelationshipDialog({ open, onOpenChange, relationship }: Props) {
  const { t } = useI18n();
  const updateRelationship = useUpdateContactRelationship();
  const [form, setForm] = useState({
    role: relationship.role ?? '',
    position: relationship.position ?? '',
    department: relationship.department ?? '',
    is_primary: relationship.is_primary,
    notes: relationship.notes ?? '',
  });

  useEffect(() => {
    setForm({
      role: relationship.role ?? '',
      position: relationship.position ?? '',
      department: relationship.department ?? '',
      is_primary: relationship.is_primary,
      notes: relationship.notes ?? '',
    });
  }, [relationship]);

  function handleSave() {
    updateRelationship.mutate(
      {
        id: relationship.id,
        data: {
          role: form.role || undefined,
          position: form.position || undefined,
          department: form.department || undefined,
          is_primary: form.is_primary,
          notes: form.notes || undefined,
        },
      },
      {
        onSuccess: () => {
          toast.success(t('contact_relationships.updated'));
          onOpenChange(false);
        },
        onError: () => toast.error(t('contact_relationships.update_failed')),
      },
    );
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>{t('contact_relationships.edit')}</DialogTitle>
        </DialogHeader>
        <div className="space-y-4">
          <div className="grid grid-cols-2 gap-3">
            <div>
              <Label>{t('contact_relationships.role')}</Label>
              <Input
                value={form.role}
                onChange={(e) => setForm({ ...form, role: e.target.value })}
              />
            </div>
            <div>
              <Label>{t('contact_relationships.position')}</Label>
              <Input
                value={form.position}
                onChange={(e) => setForm({ ...form, position: e.target.value })}
              />
            </div>
          </div>

          <div>
            <Label>{t('contact_relationships.department')}</Label>
            <Input
              value={form.department}
              onChange={(e) => setForm({ ...form, department: e.target.value })}
            />
          </div>

          <div className="flex items-center gap-2">
            <Checkbox
              id="edit_is_primary"
              checked={form.is_primary}
              onCheckedChange={(checked) =>
                setForm({ ...form, is_primary: checked === true })
              }
            />
            <Label htmlFor="edit_is_primary" className="cursor-pointer">
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
            onClick={handleSave}
            className="w-full"
            disabled={updateRelationship.isPending}
          >
            {t('common.save_changes')}
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
