import { useEffect, useRef, useState } from 'react';
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
import { useAuth } from '@/hooks/useAuth';
import { useFeatureFlagStore } from '@/stores/featureFlagStore';
import { SUPPORTED_LANGUAGES } from '@/lib/language';
import { useI18n } from '@/i18n';
import { toast } from 'sonner';

import { resolveUploadUrl } from '@/lib/platform';

function avatarSrc(url: string | null | undefined): string | null {
  return resolveUploadUrl(url);
}

export function ProfilePage() {
  const fileInputRef = useRef<HTMLInputElement>(null);
  const { user, updateProfile, uploadAvatar } = useAuth();
  const experimental = useFeatureFlagStore((s) => s.experimental);
  const setExperimental = useFeatureFlagStore((s) => s.setExperimental);
  const { t } = useI18n();
  const [fullName, setFullName] = useState('');
  const [language, setLanguage] = useState('en');

  useEffect(() => {
    setFullName(user?.full_name ?? '');
    setLanguage(user?.language ?? 'en');
  }, [user]);

  if (!user) {
    return <p className="py-8 text-center text-muted-foreground">Profile not available.</p>;
  }

  async function handleSave() {
    try {
      await updateProfile({ full_name: fullName, language });
      toast.success('Profile updated');
    } catch {
      toast.error('Failed to update profile');
    }
  }

  async function handleAvatarChange(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    if (!file) return;
    try {
      await uploadAvatar(file);
      toast.success('Profile image uploaded');
    } catch {
      toast.error('Failed to upload profile image');
    } finally {
      e.target.value = '';
    }
  }

  return (
    <div className="space-y-4">
      <div>
        <h2 className="text-lg font-semibold">Profile</h2>
        <p className="text-sm text-muted-foreground">Manage your account settings</p>
      </div>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Profile Image</CardTitle>
        </CardHeader>
        <CardContent className="flex items-center gap-4">
          {avatarSrc(user.avatar_url) ? (
            <img
              src={avatarSrc(user.avatar_url) ?? ''}
              alt={user.full_name}
              className="h-16 w-16 rounded-full object-cover border"
            />
          ) : (
            <div className="flex h-16 w-16 items-center justify-center rounded-full bg-muted text-lg font-semibold">
              {user.full_name.charAt(0).toUpperCase()}
            </div>
          )}
          <div>
            <input
              ref={fileInputRef}
              type="file"
              accept="image/*"
              className="hidden"
              onChange={handleAvatarChange}
            />
            <Button variant="outline" size="sm" onClick={() => fileInputRef.current?.click()}>
              Upload Image
            </Button>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Account</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div>
            <Label>Full Name</Label>
            <Input value={fullName} onChange={(e) => setFullName(e.target.value)} />
          </div>
          <div>
            <Label>Email</Label>
            <Input value={user.email} disabled />
          </div>
          <div>
            <Label>Language</Label>
            <Select value={language} onValueChange={setLanguage}>
              <SelectTrigger><SelectValue /></SelectTrigger>
              <SelectContent>
                {SUPPORTED_LANGUAGES.map((lang) => (
                  <SelectItem key={lang.code} value={lang.code}>
                    {lang.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <Button onClick={handleSave}>Save Profile</Button>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t('profile.experimental_title', 'Experimental Features')}</CardTitle>
        </CardHeader>
        <CardContent className="space-y-2">
          <p className="text-sm text-muted-foreground">
            {t('profile.experimental_description', 'Enable features that are still in development. These may be incomplete or unstable.')}
          </p>
          <div className="flex items-center gap-2">
            <Switch
              id="experimental-toggle"
              checked={experimental}
              onCheckedChange={setExperimental}
            />
            <Label htmlFor="experimental-toggle">
              {t('profile.experimental_label', 'Enable experimental features')}
            </Label>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
