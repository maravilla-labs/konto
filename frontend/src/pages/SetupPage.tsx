import { useState, useCallback } from 'react';
import { useNavigate } from 'react-router-dom';
import { Button } from '@/components/ui/button';
import { Check, ChevronLeft, ChevronRight } from 'lucide-react';
import { AnimatePresence, motion } from 'framer-motion';
import { useI18n } from '@/i18n';
import type { SupportedLanguage } from '@/lib/language';
import { LanguageStep } from '@/components/setup/LanguageStep';
import { AdminAccountStep } from '@/components/setup/AdminAccountStep';
import { CompanyInfoStep } from '@/components/setup/CompanyInfoStep';
import { AccountingConfigStep } from '@/components/setup/AccountingConfigStep';
import { ReviewStep } from '@/components/setup/ReviewStep';
import { useCompleteSetup } from '@/hooks/useSetup';
import { extractErrorMessage } from '@/api/client';
import { isTauri } from '@/lib/platform';
import { MacTrafficLights, WinControls, useIsMac } from '@/components/layout/WindowControls';

const isDesktop = isTauri();

const STEPS = ['language', 'account', 'company', 'accounting', 'review'] as const;

const slideVariants = {
  enter: (dir: number) => ({
    x: dir > 0 ? 60 : -60,
    opacity: 0,
  }),
  center: {
    x: 0,
    opacity: 1,
  },
  exit: (dir: number) => ({
    x: dir > 0 ? -60 : 60,
    opacity: 0,
  }),
};

export function SetupPage() {
  const navigate = useNavigate();
  const { t, language: uiLang, setLanguage: setUiLang } = useI18n();
  const isMac = useIsMac();
  const completeSetup = useCompleteSetup();

  const [step, setStep] = useState(0);
  const [direction, setDirection] = useState(0);
  const [error, setError] = useState<string | null>(null);
  const [errors, setErrors] = useState<Record<string, string>>({});

  const [language, setLanguage] = useState<SupportedLanguage>(uiLang);
  const [admin, setAdmin] = useState({
    full_name: '',
    email: '',
    password: '',
    confirm_password: '',
  });
  const [company, setCompany] = useState({
    legal_name: '',
    trade_name: '',
    street: '',
    postal_code: '',
    city: '',
    country: 'CH',
    legal_entity_type: 'GmbH',
  });
  const [accounting, setAccounting] = useState({
    default_currency: 'cur-chf',
    vat_method: 'effective',
    flat_rate_percentage: '6.2',
    date_format: 'dd.MM.yyyy',
    fiscal_year_start_month: '1',
  });

  const handleLanguageChange = useCallback(
    (lang: SupportedLanguage) => {
      setLanguage(lang);
      setUiLang(lang);
    },
    [setUiLang],
  );

  const validateStep = useCallback(
    (s: number): boolean => {
      const errs: Record<string, string> = {};

      if (s === 1) {
        if (!admin.full_name.trim()) errs.full_name = t('setup.name_required', 'Name is required');
        if (!admin.email.trim() || !admin.email.includes('@'))
          errs.email = t('auth.valid_email_required', 'Please enter a valid email');
        if (admin.password.length < 8)
          errs.password = t('setup.password_min_length', 'Password must be at least 8 characters');
        if (admin.password !== admin.confirm_password)
          errs.confirm_password = t('setup.password_mismatch', 'Passwords do not match');
      }

      if (s === 2) {
        if (!company.legal_name.trim())
          errs.legal_name = t('setup.legal_name_required', 'Company name is required');
      }

      setErrors(errs);
      return Object.keys(errs).length === 0;
    },
    [admin, company, t],
  );

  const goNext = useCallback(() => {
    if (!validateStep(step)) return;
    setDirection(1);
    setStep((s) => Math.min(s + 1, STEPS.length - 1));
  }, [step, validateStep]);

  const goBack = useCallback(() => {
    setDirection(-1);
    setStep((s) => Math.max(s - 1, 0));
    setErrors({});
  }, []);

  const goToStep = useCallback((s: number) => {
    setDirection(s > step ? 1 : -1);
    setStep(s);
    setErrors({});
  }, [step]);

  const handleComplete = useCallback(async () => {
    setError(null);
    try {
      const result = await completeSetup.mutateAsync({
        admin_email: admin.email,
        admin_password: admin.password,
        admin_full_name: admin.full_name,
        admin_language: language,
        legal_name: company.legal_name,
        trade_name: company.trade_name || undefined,
        street: company.street || undefined,
        postal_code: company.postal_code || undefined,
        city: company.city || undefined,
        country: company.country || undefined,
        legal_entity_type: company.legal_entity_type || undefined,
        default_currency: accounting.default_currency || undefined,
        vat_method: accounting.vat_method || undefined,
        flat_rate_percentage:
          accounting.vat_method === 'flat_rate'
            ? parseFloat(accounting.flat_rate_percentage) || undefined
            : undefined,
        date_format: accounting.date_format || undefined,
        fiscal_year_start_month: parseInt(accounting.fiscal_year_start_month) || undefined,
      });

      localStorage.setItem(
        'konto_tokens',
        JSON.stringify({
          access_token: result.access_token,
          refresh_token: result.refresh_token,
        }),
      );

      window.location.href = '/dashboard';
    } catch (err) {
      setError(extractErrorMessage(err));
    }
  }, [admin, language, company, accounting, completeSetup, navigate]);

  return (
    <div
      className="login-bg flex min-h-screen flex-col overflow-y-auto"
      style={{ fontFamily: "'DM Sans', sans-serif" }}
    >
      {isDesktop && (
        <div className="relative flex h-10 shrink-0 items-center gap-2 px-4 select-none">
          <div data-tauri-drag-region className="absolute inset-x-0 top-0 z-0 h-3" />
          <div className="relative z-10">{isMac ? <MacTrafficLights /> : <div />}</div>
          <div className="flex-1" />
          <div className="relative z-10">{!isMac ? <WinControls /> : <div />}</div>
        </div>
      )}

      <div className="relative z-10 flex flex-1 items-center justify-center px-4 py-4">
        <div className="w-full max-w-xl">
          {/* Header */}
          <motion.div
            className="mb-3 flex items-center justify-center gap-3"
            initial={{ opacity: 0, y: -12 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.4, ease: 'easeOut' }}
          >
            <img
              src="/logo.png"
              alt="Maravilla Konto"
              className="h-8 w-8 rounded-lg"
            />
            <div>
              <h1
                className="text-base tracking-wide text-white drop-shadow-sm leading-tight"
                style={{ fontFamily: "'DM Serif Display', serif" }}
              >
                Maravilla Konto
              </h1>
              <p className="text-[9px] font-light tracking-widest uppercase text-white/45">
                {t('setup.subtitle', 'Setup Wizard')}
              </p>
            </div>
          </motion.div>

          {/* Step Indicator */}
          <motion.div
            className="mb-3"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 0.15, duration: 0.4 }}
          >
            <div className="flex items-center justify-center">
              {STEPS.map((s, i) => (
                <div key={s} className="flex items-center">
                  <div className="flex flex-col items-center">
                    <motion.div
                      className={`relative flex h-7 w-7 items-center justify-center rounded-full text-[11px] font-semibold transition-colors duration-500 ${
                        i <= step
                          ? 'bg-white/90 text-[#1B2B4B] shadow-md'
                          : 'bg-white/15 text-white/50 border border-white/20'
                      }`}
                      animate={i === step ? { scale: [1, 1.08, 1] } : { scale: 1 }}
                      transition={{ duration: 0.35, ease: 'easeInOut' }}
                    >
                      {i < step ? (
                        <Check className="h-3.5 w-3.5" strokeWidth={3} />
                      ) : (
                        i + 1
                      )}
                    </motion.div>
                    <span className="mt-1.5 text-[9px] font-medium tracking-wider uppercase text-white/40 hidden sm:block">
                      {t(`setup.step_${s}`, s)}
                    </span>
                  </div>
                  {i < STEPS.length - 1 && (
                    <div className="mx-2 h-px w-8 sm:w-12 relative">
                      <div className="absolute inset-0 bg-white/15" />
                      <motion.div
                        className="absolute inset-y-0 left-0 bg-white/60"
                        initial={false}
                        animate={{ width: i < step ? '100%' : '0%' }}
                        transition={{ duration: 0.4, ease: 'easeInOut' }}
                      />
                    </div>
                  )}
                </div>
              ))}
            </div>
          </motion.div>

          {/* Card */}
          <motion.div
            className="flex flex-col rounded-2xl border border-white/20 bg-white/90 backdrop-blur-xl shadow-lg shadow-black/[0.06]"
            style={{ maxHeight: 'calc(100vh - 180px)' }}
            initial={{ opacity: 0, y: 16 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.2, duration: 0.4 }}
          >
            <div className="flex-1 overflow-y-auto p-5 sm:p-6">
              <AnimatePresence mode="wait" custom={direction}>
                <motion.div
                  key={step}
                  custom={direction}
                  variants={slideVariants}
                  initial="enter"
                  animate="center"
                  exit="exit"
                  transition={{ duration: 0.25, ease: 'easeInOut' }}
                >
                  {step === 0 && <LanguageStep value={language} onChange={handleLanguageChange} />}
                  {step === 1 && <AdminAccountStep value={admin} onChange={setAdmin} errors={errors} />}
                  {step === 2 && <CompanyInfoStep value={company} onChange={setCompany} errors={errors} />}
                  {step === 3 && <AccountingConfigStep value={accounting} onChange={setAccounting} />}
                  {step === 4 && (
                    <ReviewStep
                      data={{ language, admin, company, accounting }}
                      onGoToStep={goToStep}
                      onComplete={handleComplete}
                      isSubmitting={completeSetup.isPending}
                      error={error}
                    />
                  )}
                </motion.div>
              </AnimatePresence>
            </div>

            {/* Navigation — pinned at bottom */}
            {step < 4 && (
              <div className="shrink-0 border-t border-[#E8E6E1]/40 px-5 py-3 sm:px-6 flex items-center justify-between">
                <Button
                  type="button"
                  variant="ghost"
                  onClick={goBack}
                  disabled={step === 0}
                  className="text-[#1B2B4B]/40 hover:text-[#1B2B4B] transition-colors font-medium"
                >
                  <ChevronLeft className="mr-1 h-4 w-4" />
                  {t('setup.back', 'Back')}
                </Button>
                <Button
                  type="button"
                  onClick={goNext}
                  className="bg-[#1B2B4B] hover:bg-[#0F1D33] text-white px-5 h-9 rounded-lg text-sm font-medium transition-colors"
                >
                  {t('setup.next', 'Next')}
                  <ChevronRight className="ml-1 h-4 w-4" />
                </Button>
              </div>
            )}
          </motion.div>

          {/* Footer */}
          <motion.p
            className="mt-3 text-center text-[10px] tracking-wider text-white/25"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 0.6 }}
          >
            POWERED BY MARAVILLA LABS
          </motion.p>
        </div>
      </div>
    </div>
  );
}
