import { useEffect, useState, useCallback } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandSeparator,
} from '@/components/ui/command';
import { quickActions } from '@/lib/navigation';
import { useRecentPages } from '@/hooks/useRecentPages';
import { useNavigation } from '@/hooks/useNavigation';
import { useI18n } from '@/i18n';

export function CommandPalette() {
  const [open, setOpen] = useState(false);
  const navigate = useNavigate();
  const { recentItems } = useRecentPages();
  const { filteredItems } = useNavigation();
  const { t } = useI18n();

  useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === 'k' && (e.metaKey || e.ctrlKey)) {
        e.preventDefault();
        setOpen((prev) => !prev);
      }
    };
    window.addEventListener('keydown', down);
    return () => window.removeEventListener('keydown', down);
  }, []);

  const handleSelect = useCallback(
    (path: string) => {
      setOpen(false);
      navigate(path);
    },
    [navigate],
  );

  // Filter quick actions by role
  const filteredActions = quickActions.filter((action) => {
    if (action.roles.length === 0) return true;
    return filteredItems.some((item) => item.path === action.path);
  });

  // Group filtered items by category for display
  const categories = ['Overview', 'Sales', 'Finance', 'CRM', 'Reports', 'Settings', 'Data'] as const;

  return (
    <CommandDialog open={open} onOpenChange={setOpen}>
      <CommandInput placeholder={t('ui.command_search', 'Search pages, actions...')} />
      <CommandList>
        <CommandEmpty>{t('ui.no_results', 'No results found.')}</CommandEmpty>

        {recentItems.length > 0 && (
          <CommandGroup heading={t('ui.recent', 'Recent')}>
            {recentItems.slice(0, 5).map((item) => (
              <CommandItem
                key={`recent-${item.id}`}
                value={`${item.label} ${item.keywords.join(' ')}`}
                onSelect={() => handleSelect(item.path)}
              >
                <item.icon className="mr-2 h-4 w-4" />
                <span>{item.label}</span>
                <span className="ml-auto text-xs text-muted-foreground">{item.category}</span>
              </CommandItem>
            ))}
          </CommandGroup>
        )}

        {recentItems.length > 0 && <CommandSeparator />}

        {filteredActions.length > 0 && (
          <CommandGroup heading={t('ui.quick_actions', 'Quick Actions')}>
            {filteredActions.map((action) => (
              <CommandItem
                key={action.id}
                value={`${action.label} ${action.keywords.join(' ')}`}
                onSelect={() => handleSelect(action.path)}
              >
                <action.icon className="mr-2 h-4 w-4" />
                <span>{action.label}</span>
              </CommandItem>
            ))}
          </CommandGroup>
        )}

        {filteredActions.length > 0 && <CommandSeparator />}

        {categories.map((category) => {
          const items = filteredItems.filter((item) => item.category === category);
          if (items.length === 0) return null;
          return (
            <CommandGroup key={category} heading={category}>
              {items.map((item) => (
                <CommandItem
                  key={item.id}
                  value={`${item.label} ${item.keywords.join(' ')}`}
                  onSelect={() => handleSelect(item.path)}
                >
                  <item.icon className="mr-2 h-4 w-4" />
                  <span>{item.label}</span>
                </CommandItem>
              ))}
            </CommandGroup>
          );
        })}
      </CommandList>
    </CommandDialog>
  );
}
