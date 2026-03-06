import { useMemo } from 'react';
import { navItems, type NavItem, type NavCategory } from '@/lib/navigation';
import { useAuthStore } from '@/stores/authStore';
import { useFeatureFlagStore } from '@/stores/featureFlagStore';
import type { Role } from '@/types/auth';
import { useI18n } from '@/i18n';

export interface NavGroup {
  category: NavCategory;
  items: NavItem[];
}

export function useNavigation() {
  const user = useAuthStore((s) => s.user);
  const userRole = (user?.role ?? 'employee') as Role;
  const experimental = useFeatureFlagStore((s) => s.experimental);
  const { t } = useI18n();

  const filteredItems = useMemo(() => {
    return navItems
      .filter((item) => {
        if (item.experimental && !experimental) return false;
        if (item.roles.length === 0) return true;
        return item.roles.includes(userRole);
      })
      .map((item) => ({
        ...item,
        label: t(`nav.${item.id}`, item.label),
      }));
  }, [t, userRole, experimental]);

  const sidebarItems = useMemo(() => {
    return filteredItems.filter((item) => item.showInSidebar);
  }, [filteredItems]);

  const sidebarGroups = useMemo(() => {
    const groups: NavGroup[] = [];
    const categoryOrder: NavCategory[] = ['Overview', 'Sales', 'Finance', 'CRM'];

    for (const category of categoryOrder) {
      const items = sidebarItems.filter((item) => item.category === category && !item.parent);
      if (items.length > 0) {
        groups.push({ category, items });
      }
    }
    return groups;
  }, [sidebarItems]);

  const getChildItems = (parentId: string) => {
    return sidebarItems.filter((item) => item.parent === parentId);
  };

  return { filteredItems, sidebarItems, sidebarGroups, getChildItems, userRole };
}
