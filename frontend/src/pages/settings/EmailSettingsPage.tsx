import { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Switch } from '@/components/ui/switch';
import { Skeleton } from '@/components/ui/skeleton';
import { useEmailSettings, useUpdateEmailSettings, useSendTestEmail } from '@/hooks/useEmailApi';
import { toast } from 'sonner';
import { Send } from 'lucide-react';
import type { UpdateEmailSettings } from '@/types/email';

const encryptionOptions = [
  { value: 'starttls', label: 'STARTTLS (port 587)' },
  { value: 'ssl', label: 'SSL/TLS (port 465)' },
  { value: 'none', label: 'None (port 25)' },
];

const defaultForm: UpdateEmailSettings = {
  smtp_host: '',
  smtp_port: 587,
  smtp_username: '',
  smtp_password: '',
  smtp_encryption: 'starttls',
  from_email: '',
  from_name: '',
  reply_to_email: '',
  bcc_email: '',
  is_active: false,
};

export function EmailSettingsPage() {
  const { data, isLoading } = useEmailSettings();
  const updateSettings = useUpdateEmailSettings();
  const sendTest = useSendTestEmail();

  const [form, setForm] = useState<UpdateEmailSettings>(defaultForm);

  useEffect(() => {
    if (data) {
      setForm({
        smtp_host: data.smtp_host,
        smtp_port: data.smtp_port,
        smtp_username: data.smtp_username,
        smtp_password: data.smtp_password,
        smtp_encryption: data.smtp_encryption,
        from_email: data.from_email,
        from_name: data.from_name,
        reply_to_email: data.reply_to_email ?? '',
        bcc_email: data.bcc_email ?? '',
        is_active: data.is_active,
      });
    }
  }, [data]);

  function set(field: keyof UpdateEmailSettings, value: string | number | boolean) {
    setForm({ ...form, [field]: value });
  }

  function handleSave() {
    updateSettings.mutate(form, {
      onSuccess: () => toast.success('Email settings saved'),
      onError: () => toast.error('Failed to save email settings'),
    });
  }

  function handleTest() {
    sendTest.mutate(undefined, {
      onSuccess: () => toast.success('Test email sent'),
      onError: () => toast.error('Failed to send test email'),
    });
  }

  if (isLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-64 w-full" />
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div>
        <h2 className="text-lg font-semibold">Email Settings</h2>
        <p className="text-sm text-muted-foreground">
          Configure SMTP for sending invoices and documents
        </p>
      </div>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">SMTP Server</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid gap-4 sm:grid-cols-2">
            <div>
              <Label>SMTP Host</Label>
              <Input
                value={form.smtp_host}
                onChange={(e) => set('smtp_host', e.target.value)}
                placeholder="smtp.example.com"
              />
            </div>
            <div>
              <Label>SMTP Port</Label>
              <Input
                type="number"
                value={form.smtp_port}
                onChange={(e) => set('smtp_port', parseInt(e.target.value) || 587)}
              />
            </div>
          </div>
          <div className="grid gap-4 sm:grid-cols-2">
            <div>
              <Label>Username</Label>
              <Input
                value={form.smtp_username}
                onChange={(e) => set('smtp_username', e.target.value)}
              />
            </div>
            <div>
              <Label>Password</Label>
              <Input
                type="password"
                value={form.smtp_password ?? ''}
                onChange={(e) => set('smtp_password', e.target.value)}
                placeholder={data ? '********' : ''}
              />
            </div>
          </div>
          <div>
            <Label>Encryption</Label>
            <Select
              value={form.smtp_encryption}
              onValueChange={(v) => set('smtp_encryption', v)}
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {encryptionOptions.map((o) => (
                  <SelectItem key={o.value} value={o.value}>
                    {o.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Sender Details</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid gap-4 sm:grid-cols-2">
            <div>
              <Label>From Email</Label>
              <Input
                type="email"
                value={form.from_email}
                onChange={(e) => set('from_email', e.target.value)}
                placeholder="invoice@example.com"
              />
            </div>
            <div>
              <Label>From Name</Label>
              <Input
                value={form.from_name}
                onChange={(e) => set('from_name', e.target.value)}
                placeholder="Your Company"
              />
            </div>
          </div>
          <div className="grid gap-4 sm:grid-cols-2">
            <div>
              <Label>Reply-To Email</Label>
              <Input
                type="email"
                value={form.reply_to_email ?? ''}
                onChange={(e) => set('reply_to_email', e.target.value)}
                placeholder="Optional"
              />
            </div>
            <div>
              <Label>BCC Email</Label>
              <Input
                type="email"
                value={form.bcc_email ?? ''}
                onChange={(e) => set('bcc_email', e.target.value)}
                placeholder="Optional"
              />
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Status</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex items-center gap-3">
            <Switch
              checked={form.is_active}
              onCheckedChange={(v) => set('is_active', v)}
            />
            <Label>{form.is_active ? 'Active' : 'Inactive'}</Label>
          </div>
          <p className="mt-2 text-xs text-muted-foreground">
            Enable to allow sending emails from the application
          </p>
        </CardContent>
      </Card>

      <div className="flex justify-end gap-2">
        <Button
          variant="outline"
          onClick={handleTest}
          disabled={sendTest.isPending || !data?.is_active}
        >
          <Send className="mr-1 h-3.5 w-3.5" />
          {sendTest.isPending ? 'Sending...' : 'Send Test Email'}
        </Button>
        <Button onClick={handleSave} disabled={updateSettings.isPending}>
          {updateSettings.isPending ? 'Saving...' : 'Save Settings'}
        </Button>
      </div>
    </div>
  );
}
