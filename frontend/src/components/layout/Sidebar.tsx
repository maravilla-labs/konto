import { useState } from 'react';
import { NavLink, useLocation } from 'react-router-dom';
import {
  ChevronRight,
  LogOut,
  Settings,
  Upload,
} from 'lucide-react';
import { resolveUploadUrl } from '@/lib/platform';
import {
  Sidebar as ShadSidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarMenuSub,
  SidebarMenuSubButton,
  SidebarMenuSubItem,
} from '@/components/ui/sidebar';
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from '@/components/ui/collapsible';
import { Separator } from '@/components/ui/separator';
import { useAuth } from '@/hooks/useAuth';
import { useNavigation } from '@/hooks/useNavigation';
import { useSettings } from '@/hooks/useSettingsApi';
import { isTauri } from '@/lib/platform';
import { useIsMac } from './WindowControls';
import { useI18n } from '@/i18n';

const isDesktop = isTauri();

export function AppSidebar() {
  const { user, logout } = useAuth();
  const { sidebarGroups, getChildItems } = useNavigation();
  const { data: settings } = useSettings();
  const isMac = useIsMac();
  const { t } = useI18n();

  return (
    <ShadSidebar>
      <SidebarHeader className="p-3">
        <div className={`flex items-center gap-2 ${isDesktop && isMac ? 'pl-[60px]' : ''}`}>
          <img
            src={settings?.logo_url ? (resolveUploadUrl(settings.logo_url) ?? '/logo.png') : '/logo.png'}
            alt={settings?.legal_name ?? 'Maravilla Konto'}
            className="h-7 w-7 rounded pointer-events-none"
          />
          <div className="min-w-0 flex-1 select-none">
            <h2 className="text-sm font-semibold leading-tight text-sidebar-foreground truncate">
              {settings?.legal_name ?? ''}
            </h2>
            <p className="text-[11px] leading-tight text-sidebar-foreground/60 truncate">
              Maravilla Konto
            </p>
          </div>
        </div>
      </SidebarHeader>
      <Separator className="bg-sidebar-border" />
      <SidebarContent>
        {sidebarGroups.map((group) => (
          <SidebarGroup key={group.category}>
            <SidebarGroupLabel>
              {t(`category.${group.category.toLowerCase()}`, group.category)}
            </SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                {group.items.map((item) => {
                  const children = getChildItems(item.id);
                  if (children.length > 0) {
                    return (
                      <CollapsibleNavItem
                        key={item.id}
                        item={item}
                        children={children}
                      />
                    );
                  }
                  return (
                    <SidebarMenuItem key={item.id}>
                      <SidebarMenuButton asChild>
                        <NavLink
                          to={item.path}
                          end={item.path === '/reports'}
                          className={({ isActive }) =>
                            isActive ? 'bg-sidebar-accent text-sidebar-accent-foreground' : ''
                          }
                        >
                          <item.icon className="h-4 w-4" />
                          <span>{item.label}</span>
                        </NavLink>
                      </SidebarMenuButton>
                    </SidebarMenuItem>
                  );
                })}
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        ))}
      </SidebarContent>
      <Separator className="bg-sidebar-border" />
      <SidebarFooter className="p-3">
        <div className="flex items-center gap-2 mb-2">
          <NavLink
            to="/settings"
            className="flex items-center gap-2 rounded-md px-2 py-1.5 text-sm text-sidebar-foreground/70 hover:bg-sidebar-accent hover:text-sidebar-foreground transition-colors"
          >
            <Settings className="h-4 w-4" />
            <span>{t('ui.settings', 'Settings')}</span>
          </NavLink>
          <NavLink
            to="/import"
            className="flex items-center gap-2 rounded-md px-2 py-1.5 text-sm text-sidebar-foreground/70 hover:bg-sidebar-accent hover:text-sidebar-foreground transition-colors"
          >
            <Upload className="h-4 w-4" />
            <span>{t('ui.import', 'Import')}</span>
          </NavLink>
        </div>
        <Separator className="bg-sidebar-border mb-2" />
        <div className="flex items-center justify-between">
          <NavLink to="/profile" className="flex min-w-0 items-center gap-2 rounded-md px-1 py-0.5 hover:bg-sidebar-accent">
            {user?.avatar_url ? (
              <img
                src={resolveUploadUrl(user.avatar_url) ?? ''}
                alt={user.full_name}
                className="h-7 w-7 shrink-0 rounded-full object-cover"
              />
            ) : (
              <div className="flex h-7 w-7 shrink-0 items-center justify-center rounded-full bg-sidebar-accent text-xs font-medium text-sidebar-accent-foreground">
                {user?.full_name?.charAt(0)?.toUpperCase() ?? 'U'}
              </div>
            )}
            <div className="min-w-0">
              <p className="truncate text-sm font-medium text-sidebar-foreground">
                {user?.full_name ?? 'User'}
              </p>
              <p className="truncate text-xs text-sidebar-foreground/60">
                {user?.email ?? ''}
              </p>
            </div>
          </NavLink>
          <button
            onClick={logout}
            className="rounded-md p-2 text-sidebar-foreground/60 hover:bg-sidebar-accent hover:text-sidebar-foreground"
            title={t('ui.sign_out', 'Sign out')}
          >
            <LogOut className="h-4 w-4" />
          </button>
        </div>
      </SidebarFooter>
    </ShadSidebar>
  );
}

interface CollapsibleNavItemProps {
  item: { id: string; label: string; path: string; icon: React.ComponentType<{ className?: string }> };
  children: { id: string; label: string; path: string; icon: React.ComponentType<{ className?: string }> }[];
}

function CollapsibleNavItem({ item, children }: CollapsibleNavItemProps) {
  const location = useLocation();
  const isChildActive = children.some((child) => location.pathname === child.path);
  const isParentActive = location.pathname === item.path;
  const [open, setOpen] = useState(isChildActive || isParentActive);

  return (
    <Collapsible open={open} onOpenChange={setOpen} asChild>
      <SidebarMenuItem>
        <CollapsibleTrigger asChild>
          <SidebarMenuButton
            className={
              isParentActive ? 'bg-sidebar-accent text-sidebar-accent-foreground' : ''
            }
          >
            <NavLink
              to={item.path}
              end
              className="flex items-center gap-2 flex-1"
              onClick={(e) => e.stopPropagation()}
            >
              <item.icon className="h-4 w-4" />
              <span>{item.label}</span>
            </NavLink>
            <ChevronRight
              className={`ml-auto h-4 w-4 shrink-0 transition-transform duration-200 ${
                open ? 'rotate-90' : ''
              }`}
            />
          </SidebarMenuButton>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <SidebarMenuSub>
            {children.map((child) => (
              <SidebarMenuSubItem key={child.id}>
                <SidebarMenuSubButton asChild>
                  <NavLink
                    to={child.path}
                    className={({ isActive }) =>
                      isActive ? 'bg-sidebar-accent text-sidebar-accent-foreground' : ''
                    }
                  >
                    <child.icon className="h-4 w-4" />
                    <span>{child.label}</span>
                  </NavLink>
                </SidebarMenuSubButton>
              </SidebarMenuSubItem>
            ))}
          </SidebarMenuSub>
        </CollapsibleContent>
      </SidebarMenuItem>
    </Collapsible>
  );
}
