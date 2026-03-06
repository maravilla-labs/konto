import { useI18n } from '@/i18n';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { motion } from 'framer-motion';

interface AdminData {
  full_name: string;
  email: string;
  password: string;
  confirm_password: string;
}

interface AdminAccountStepProps {
  value: AdminData;
  onChange: (data: AdminData) => void;
  errors: Record<string, string>;
}

const fieldVariants = {
  hidden: { opacity: 0, y: 10 },
  visible: (i: number) => ({
    opacity: 1,
    y: 0,
    transition: { delay: i * 0.05, duration: 0.3, ease: 'easeOut' as const },
  }),
};

const inputClassName =
  'h-10 rounded-xl border-[#E8E6E1] bg-[#FAFAF8] px-4 text-sm text-[#1B2B4B] placeholder:text-[#1B2B4B]/20 focus:border-[#1B2B4B] focus:ring-[#1B2B4B]/10';

export function AdminAccountStep({ value, onChange, errors }: AdminAccountStepProps) {
  const { t } = useI18n();

  const update = (field: keyof AdminData, val: string) => {
    onChange({ ...value, [field]: val });
  };

  return (
    <div className="space-y-5">
      <div className="text-center">
        <h2
          className="text-xl sm:text-2xl text-[#1B2B4B]"
          style={{ fontFamily: "'DM Serif Display', serif" }}
        >
          {t('setup.admin_account', 'Admin Account')}
        </h2>
        <p className="mt-1.5 text-sm text-[#1B2B4B]/40 font-light">
          {t('setup.admin_account_desc', 'Create the administrator account for your system.')}
        </p>
      </div>

      <div className="space-y-4">
        <motion.div custom={0} variants={fieldVariants} initial="hidden" animate="visible" className="space-y-1.5">
          <Label htmlFor="full_name" className="text-xs font-semibold uppercase tracking-wider text-[#1B2B4B]/50">
            {t('setup.full_name', 'Full Name')}
          </Label>
          <Input
            id="full_name"
            value={value.full_name}
            onChange={(e) => update('full_name', e.target.value)}
            placeholder="Max Muster"
            className={inputClassName}
          />
          {errors.full_name && <p className="text-xs text-[#C1272D]">{errors.full_name}</p>}
        </motion.div>

        <motion.div custom={1} variants={fieldVariants} initial="hidden" animate="visible" className="space-y-1.5">
          <Label htmlFor="email" className="text-xs font-semibold uppercase tracking-wider text-[#1B2B4B]/50">
            {t('common.email', 'Email')}
          </Label>
          <Input
            id="email"
            type="email"
            value={value.email}
            onChange={(e) => update('email', e.target.value)}
            placeholder="admin@company.ch"
            className={inputClassName}
          />
          {errors.email && <p className="text-xs text-[#C1272D]">{errors.email}</p>}
        </motion.div>

        <motion.div custom={2} variants={fieldVariants} initial="hidden" animate="visible" className="space-y-1.5">
          <Label htmlFor="password" className="text-xs font-semibold uppercase tracking-wider text-[#1B2B4B]/50">
            {t('common.password', 'Password')}
          </Label>
          <Input
            id="password"
            type="password"
            value={value.password}
            onChange={(e) => update('password', e.target.value)}
            placeholder="Minimum 8 characters"
            className={inputClassName}
          />
          {errors.password && <p className="text-xs text-[#C1272D]">{errors.password}</p>}
        </motion.div>

        <motion.div custom={3} variants={fieldVariants} initial="hidden" animate="visible" className="space-y-1.5">
          <Label htmlFor="confirm_password" className="text-xs font-semibold uppercase tracking-wider text-[#1B2B4B]/50">
            {t('setup.confirm_password', 'Confirm Password')}
          </Label>
          <Input
            id="confirm_password"
            type="password"
            value={value.confirm_password}
            onChange={(e) => update('confirm_password', e.target.value)}
            placeholder="Repeat password"
            className={inputClassName}
          />
          {errors.confirm_password && (
            <p className="text-xs text-[#C1272D]">{errors.confirm_password}</p>
          )}
        </motion.div>
      </div>
    </div>
  );
}
