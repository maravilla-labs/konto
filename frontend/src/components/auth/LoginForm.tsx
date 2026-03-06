import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { useAuth } from '@/hooks/useAuth';
import { useState } from 'react';
import { Loader2 } from 'lucide-react';
import { useI18n } from '@/i18n';

export function LoginForm() {
  const { login } = useAuth();
  const { t } = useI18n();
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);
  const loginSchema = z.object({
    email: z.string().email(t('auth.valid_email_required', 'Please enter a valid email')),
    password: z.string().min(1, t('auth.password_required', 'Password is required')),
  });
  type LoginValues = z.infer<typeof loginSchema>;

  const form = useForm<LoginValues>({
    resolver: zodResolver(loginSchema),
    defaultValues: { email: '', password: '' },
  });

  async function onSubmit(values: LoginValues) {
    setError('');
    setLoading(true);
    try {
      await login(values.email, values.password);
    } catch {
      setError(t('auth.invalid_credentials', 'Invalid email or password'));
    } finally {
      setLoading(false);
    }
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
        {error && (
          <div className="rounded-md bg-destructive/10 p-3 text-sm text-destructive">
            {error}
          </div>
        )}
        <FormField
          control={form.control}
          name="email"
          render={({ field }) => (
            <FormItem>
              <FormLabel>{t('common.email', 'Email')}</FormLabel>
              <FormControl>
                <Input placeholder="you@example.com" type="email" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="password"
          render={({ field }) => (
            <FormItem>
              <FormLabel>{t('common.password', 'Password')}</FormLabel>
              <FormControl>
                <Input
                  placeholder={t('auth.enter_password', 'Enter your password')}
                  type="password"
                  {...field}
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button type="submit" className="w-full" disabled={loading}>
          {loading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
          {t('common.sign_in', 'Sign in')}
        </Button>
      </form>
    </Form>
  );
}
