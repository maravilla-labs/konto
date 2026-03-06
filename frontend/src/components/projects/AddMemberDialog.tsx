import { useState } from 'react';
import {
  Dialog, DialogContent, DialogHeader, DialogTitle,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
} from '@/components/ui/select';
import { useUsers, useRateFunctions, useAddProjectMember } from '@/hooks/useApi';
import { useI18n } from '@/i18n';
import { toast } from 'sonner';

interface AddMemberDialogProps {
  projectId: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function AddMemberDialog({ projectId, open, onOpenChange }: AddMemberDialogProps) {
  const { t } = useI18n();
  const { data: usersRaw } = useUsers();
  const { data: rateFunctions } = useRateFunctions();
  const addMember = useAddProjectMember();

  const [userId, setUserId] = useState('');
  const [rateFunctionId, setRateFunctionId] = useState('');
  const [hourlyRate, setHourlyRate] = useState('');
  const [roleLabel, setRoleLabel] = useState('');

  const users = usersRaw ?? [];
  const functions = rateFunctions ?? [];

  function handleSubmit() {
    if (!userId) return;
    addMember.mutate(
      {
        projectId,
        data: {
          user_id: userId,
          rate_function_id: rateFunctionId || undefined,
          hourly_rate: hourlyRate ? Number(hourlyRate) : undefined,
          role_label: roleLabel || undefined,
        },
      },
      {
        onSuccess: () => {
          toast.success(t('projects.member_added', 'Member added'));
          resetForm();
          onOpenChange(false);
        },
        onError: () => toast.error(t('projects.member_add_failed', 'Failed to add member')),
      },
    );
  }

  function resetForm() {
    setUserId('');
    setRateFunctionId('');
    setHourlyRate('');
    setRoleLabel('');
  }

  return (
    <Dialog open={open} onOpenChange={(v) => { if (!v) resetForm(); onOpenChange(v); }}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>{t('projects.add_member', 'Add Team Member')}</DialogTitle>
        </DialogHeader>
        <div className="space-y-4">
          <div>
            <Label>{t('projects.user', 'User')}</Label>
            <Select value={userId} onValueChange={setUserId}>
              <SelectTrigger><SelectValue placeholder={t('projects.select_user', 'Select user')} /></SelectTrigger>
              <SelectContent>
                {users.map((u) => (
                  <SelectItem key={u.id} value={u.id}>{u.full_name}</SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div>
            <Label>{t('projects.rate_function', 'Rate Function')}</Label>
            <Select value={rateFunctionId || '__none__'} onValueChange={(v) => setRateFunctionId(v === '__none__' ? '' : v)}>
              <SelectTrigger><SelectValue placeholder={t('common.none', 'None')} /></SelectTrigger>
              <SelectContent>
                <SelectItem value="__none__">{t('common.none', 'None')}</SelectItem>
                {functions.map((rf) => (
                  <SelectItem key={rf.id} value={rf.id}>
                    {rf.name} (CHF {rf.hourly_rate}/h)
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div>
            <Label>{t('projects.hourly_rate_override', 'Hourly Rate Override')}</Label>
            <Input
              type="number"
              step="0.01"
              value={hourlyRate}
              onChange={(e) => setHourlyRate(e.target.value)}
              placeholder={t('common.optional', 'optional')}
            />
          </div>

          <div>
            <Label>{t('projects.role_label', 'Role Label')}</Label>
            <Input
              value={roleLabel}
              onChange={(e) => setRoleLabel(e.target.value)}
              placeholder={t('projects.role_placeholder', 'e.g. Lead Developer')}
            />
          </div>

          <Button onClick={handleSubmit} className="w-full" disabled={addMember.isPending || !userId}>
            {t('projects.add_member', 'Add Team Member')}
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
