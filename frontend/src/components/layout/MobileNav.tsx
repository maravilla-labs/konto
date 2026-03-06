import { useState } from 'react';
import { NavLink } from 'react-router-dom';
import {
  LayoutDashboard,
  BookOpen,
  Users,
  FileText,
  MoreHorizontal,
  FolderKanban,
  Clock,
  Upload,
  BarChart3,
  Settings,
  X,
} from 'lucide-react';
import { useI18n } from '@/i18n';

const primaryItems = [
  { to: '/dashboard', label: 'Dashboard', icon: LayoutDashboard },
  { to: '/accounts', label: 'Accounts', icon: BookOpen },
  { to: '/contacts', label: 'Contacts', icon: Users },
  { to: '/journal', label: 'Journal', icon: FileText },
];

const moreItems = [
  { to: '/projects', label: 'Projects', icon: FolderKanban },
  { to: '/time-entries', label: 'Time Entries', icon: Clock },
  { to: '/import', label: 'Import', icon: Upload },
  { to: '/reports', label: 'Reports', icon: BarChart3 },
  { to: '/settings/fiscal-years', label: 'Fiscal Years', icon: Settings },
  { to: '/settings/exchange-rates', label: 'Exchange Rates', icon: Settings },
];

export function MobileNav() {
  const [moreOpen, setMoreOpen] = useState(false);
  const { t } = useI18n();

  return (
    <>
      {moreOpen && (
        <div className="fixed inset-0 z-40 bg-background/80 md:hidden" onClick={() => setMoreOpen(false)}>
          <div
            className="absolute bottom-16 left-0 right-0 border-t bg-card p-4"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="mb-3 flex items-center justify-between">
              <span className="text-sm font-medium">{t('ui.more', 'More')}</span>
              <button onClick={() => setMoreOpen(false)}>
                <X className="h-4 w-4" />
              </button>
            </div>
            <div className="grid grid-cols-3 gap-2">
              {moreItems.map((item) => (
                <NavLink
                  key={item.to}
                  to={item.to}
                  onClick={() => setMoreOpen(false)}
                  className={({ isActive }) =>
                    `flex flex-col items-center gap-1 rounded-md p-2 text-xs ${
                      isActive ? 'bg-primary/10 text-primary font-medium' : 'text-muted-foreground'
                    }`
                  }
                >
                  <item.icon className="h-5 w-5" />
                  <span>{t(`nav.${item.to.replace(/^\//, '').replace(/\//g, '-')}`, item.label)}</span>
                </NavLink>
              ))}
            </div>
          </div>
        </div>
      )}
      <nav className="fixed bottom-0 left-0 right-0 z-50 border-t bg-card md:hidden">
        <div className="flex items-center justify-around py-2">
          {primaryItems.map((item) => (
            <NavLink
              key={item.to}
              to={item.to}
              className={({ isActive }) =>
                `flex flex-col items-center gap-0.5 px-2 py-1 text-xs ${
                  isActive ? 'text-primary font-medium' : 'text-muted-foreground'
                }`
              }
            >
              <item.icon className="h-5 w-5" />
              <span>{t(`nav.${item.to.replace(/^\//, '').replace(/\//g, '-')}`, item.label)}</span>
            </NavLink>
          ))}
          <button
            onClick={() => setMoreOpen(!moreOpen)}
            className={`flex flex-col items-center gap-0.5 px-2 py-1 text-xs ${
              moreOpen ? 'text-primary font-medium' : 'text-muted-foreground'
            }`}
          >
            <MoreHorizontal className="h-5 w-5" />
            <span>{t('ui.more', 'More')}</span>
          </button>
        </div>
      </nav>
    </>
  );
}
