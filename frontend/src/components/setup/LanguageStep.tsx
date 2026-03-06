import { useI18n } from '@/i18n';
import type { SupportedLanguage } from '@/lib/language';
import { Check } from 'lucide-react';
import { motion } from 'framer-motion';

const LANGUAGES: { code: SupportedLanguage; flag: string; name: string; native: string }[] = [
  { code: 'en', flag: '🇬🇧', name: 'English', native: 'English' },
  { code: 'de', flag: '🇨🇭', name: 'Deutsch', native: 'Swiss German' },
  { code: 'fr', flag: '🇫🇷', name: 'Français', native: 'French' },
  { code: 'it', flag: '🇮🇹', name: 'Italiano', native: 'Italian' },
];

interface LanguageStepProps {
  value: SupportedLanguage;
  onChange: (lang: SupportedLanguage) => void;
}

export function LanguageStep({ value, onChange }: LanguageStepProps) {
  const { t } = useI18n();

  return (
    <div className="space-y-5">
      <div className="text-center">
        <h2
          className="text-xl sm:text-2xl text-[#1B2B4B]"
          style={{ fontFamily: "'DM Serif Display', serif" }}
        >
          {t('setup.choose_language', 'Choose Your Language')}
        </h2>
        <p className="mt-1.5 text-sm text-[#1B2B4B]/40 font-light">
          {t('setup.choose_language_desc', 'Select the language for the application interface.')}
        </p>
      </div>

      <div className="grid grid-cols-2 gap-3">
        {LANGUAGES.map((lang, i) => {
          const selected = value === lang.code;
          return (
            <motion.button
              key={lang.code}
              type="button"
              onClick={() => onChange(lang.code)}
              initial={{ opacity: 0, y: 12 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: i * 0.06, duration: 0.3, ease: 'easeOut' as const }}
              whileHover={{ y: -1, transition: { duration: 0.15 } }}
              whileTap={{ scale: 0.98 }}
              className={`relative flex flex-col items-center gap-2 rounded-xl border-2 p-4 sm:p-5 transition-all duration-300 cursor-pointer ${
                selected
                  ? 'border-[#1B2B4B] bg-[#1B2B4B]/[0.03] shadow-lg shadow-[#1B2B4B]/5'
                  : 'border-[#E8E6E1] bg-white hover:border-[#1B2B4B]/20 hover:shadow-md'
              }`}
            >
              {selected && (
                <motion.div
                  className="absolute right-2.5 top-2.5 flex h-5 w-5 items-center justify-center rounded-full bg-[#1B2B4B]"
                  initial={{ scale: 0 }}
                  animate={{ scale: 1 }}
                  transition={{ type: 'spring', stiffness: 500, damping: 25 }}
                >
                  <Check className="h-3 w-3 text-white" strokeWidth={3} />
                </motion.div>
              )}
              <span className="text-3xl leading-none">{lang.flag}</span>
              <div className="text-center">
                <span
                  className="block text-sm font-medium text-[#1B2B4B]"
                  style={{ fontFamily: "'DM Serif Display', serif" }}
                >
                  {lang.name}
                </span>
                <span className="block text-[11px] text-[#1B2B4B]/30 font-light">
                  {lang.native}
                </span>
              </div>
            </motion.button>
          );
        })}
      </div>
    </div>
  );
}
