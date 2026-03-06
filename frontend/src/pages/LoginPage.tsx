import { Navigate } from 'react-router-dom';
import { LoginForm } from '@/components/auth/LoginForm';
import { useAuth } from '@/hooks/useAuth';
import { useSetupStatus, useBranding } from '@/hooks/useSetup';
import { isTauri } from '@/lib/platform';
import { MacTrafficLights, WinControls, useIsMac } from '@/components/layout/WindowControls';
import { useI18n } from '@/i18n';
import { Loader2 } from 'lucide-react';
import { motion } from 'framer-motion';

const isDesktop = isTauri();

export function LoginPage() {
  const { isAuthenticated } = useAuth();
  const { data: setupStatus, isLoading: setupLoading } = useSetupStatus();
  const { data: branding } = useBranding();
  const isMac = useIsMac();
  const { t } = useI18n();

  if (setupLoading) {
    return (
      <div className="flex h-screen items-center justify-center">
        <Loader2 className="h-8 w-8 animate-spin text-primary" />
      </div>
    );
  }

  if (setupStatus?.setup_needed) {
    return <Navigate to="/setup" replace />;
  }

  if (isAuthenticated) {
    return <Navigate to="/dashboard" replace />;
  }

  return (
    <div className="login-bg flex h-screen flex-col overflow-hidden">
      {isDesktop && (
        <div className="relative flex h-10 shrink-0 items-center gap-2 px-4 select-none">
          <div data-tauri-drag-region className="absolute inset-x-0 top-0 z-0 h-3" />
          <div className="relative z-10">
            {isMac ? <MacTrafficLights /> : <div />}
          </div>
          <div className="flex-1" />
          <div className="relative z-10">
            {!isMac ? <WinControls /> : <div />}
          </div>
        </div>
      )}
      <div className="flex flex-1 items-center justify-center px-4">
        <motion.div
          initial={{ opacity: 0, y: 16, scale: 0.98 }}
          animate={{ opacity: 1, y: 0, scale: 1 }}
          transition={{ duration: 0.5, ease: [0.22, 1, 0.36, 1] }}
          className="login-card w-full max-w-sm rounded-2xl border border-white/15 p-8"
        >
          {/* Logo — centered with soft glow */}
          <motion.div
            className="mb-6 flex justify-center"
            initial={{ opacity: 0, scale: 0.85 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ delay: 0.1, duration: 0.45, ease: [0.22, 1, 0.36, 1] }}
          >
            <img
              src="/logo.png"
              alt="Maravilla Konto"
              className="h-14 w-14 rounded-xl login-logo-glow"
            />
          </motion.div>

          {/* Title */}
          <motion.div
            className="mb-6 text-center"
            initial={{ opacity: 0, y: 6 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.2, duration: 0.4 }}
          >
            <h1 className="text-xl font-semibold text-white">
              Maravilla Konto
            </h1>
            <p className="mt-1 text-sm text-white/50">
              {branding?.legal_name
                ? branding.legal_name
                : t('ui.accounting_business_management', 'Accounting & Business Management')}
            </p>
          </motion.div>

          {/* Form */}
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 0.3, duration: 0.4 }}
          >
            <LoginForm />
          </motion.div>
        </motion.div>
      </div>
    </div>
  );
}
