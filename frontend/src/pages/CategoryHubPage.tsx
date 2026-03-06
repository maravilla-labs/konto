import { useMemo } from 'react';
import { NavLink } from 'react-router-dom';
import { ChevronRight, type LucideIcon } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { useNavigation } from '@/hooks/useNavigation';
import type { NavCategory, NavItem } from '@/lib/navigation';

interface CategoryHubPageProps {
  category: NavCategory;
  title: string;
  description: string;
}

function CategoryHubPage({ category, title, description }: CategoryHubPageProps) {
  const { filteredItems } = useNavigation();

  const categoryItems = useMemo(
    () => filteredItems.filter((item) => item.category === category),
    [category, filteredItems],
  );

  const parentItems = useMemo(
    () => categoryItems.filter((item) => !item.parent),
    [categoryItems],
  );

  const childrenByParent = useMemo(() => {
    const grouped = new Map<string, NavItem[]>();
    for (const item of categoryItems) {
      if (!item.parent) continue;
      const prev = grouped.get(item.parent) ?? [];
      prev.push(item);
      grouped.set(item.parent, prev);
    }
    return grouped;
  }, [categoryItems]);

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-semibold">{title}</h2>
        <p className="text-sm text-muted-foreground">{description}</p>
      </div>

      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {parentItems.map((parent) => (
          <HubCard
            key={parent.id}
            icon={parent.icon}
            title={parent.label}
            links={[parent, ...(childrenByParent.get(parent.id) ?? [])]}
          />
        ))}
      </div>
    </div>
  );
}

function HubCard({ title, icon: Icon, links }: { title: string; icon: LucideIcon; links: NavItem[] }) {
  return (
    <Card>
      <CardHeader className="flex flex-row items-center gap-3 pb-3">
        <div className="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-muted">
          <Icon className="h-5 w-5 text-muted-foreground" />
        </div>
        <CardTitle className="text-base">{title}</CardTitle>
      </CardHeader>
      <CardContent className="pt-0">
        <ul className="space-y-1">
          {links.map((link) => (
            <li key={link.id}>
              <NavLink
                to={link.path}
                className="flex items-center justify-between rounded-md px-2 py-1.5 text-sm text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors"
              >
                <span>{link.label}</span>
                <ChevronRight className="h-4 w-4 opacity-50" />
              </NavLink>
            </li>
          ))}
        </ul>
      </CardContent>
    </Card>
  );
}

export function SalesHubPage() {
  return (
    <CategoryHubPage
      category="Sales"
      title="Sales"
      description="Overview of invoices, credit notes, and sales documents."
    />
  );
}

export function FinanceHubPage() {
  return (
    <CategoryHubPage
      category="Finance"
      title="Finance"
      description="Overview of accounts, journal entries, expenses, and banking."
    />
  );
}

export function CrmHubPage() {
  return (
    <CategoryHubPage
      category="CRM"
      title="CRM"
      description="Overview of contacts, projects, and time tracking."
    />
  );
}
