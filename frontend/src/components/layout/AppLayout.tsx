import { useEffect, useRef } from 'react';
import { Outlet } from 'react-router-dom';
import { SidebarProvider, SidebarInset, useSidebar } from '@/components/ui/sidebar';
import { AppSidebar } from './Sidebar';
import { TopBar } from './TopBar';
import { MobileNav } from './MobileNav';
import { CommandPalette } from './CommandPalette';
import { MacTrafficLights, useIsMac } from './WindowControls';
import { isTauri } from '@/lib/platform';

const isDesktop = isTauri();

/**
 * On macOS Tauri (transparent + undecorated window), layout shifts from
 * sidebar collapse can leave the WebView's hit-test region stale.
 * A 1px resize bounce forces recalculation.
 * Also ensures keyboard focus stays in the content area after collapse.
 */
function SidebarHitTestFix() {
  const { open } = useSidebar();
  const prevOpen = useRef(open);

  useEffect(() => {
    if (prevOpen.current === open) return;
    prevOpen.current = open;

    // After sidebar collapses, ensure focus stays in content so keyboard
    // shortcuts (Cmd+K) continue to work.
    if (!open) {
      requestAnimationFrame(() => {
        if (!document.activeElement || document.activeElement === document.body) {
          (document.querySelector('[data-slot="sidebar-trigger"]') as HTMLElement)?.focus();
        }
      });
    }

    if (!isDesktop) return;

    const timer = setTimeout(async () => {
      const { refreshHitTest } = await import('@/lib/native');
      await refreshHitTest();
    }, 250); // after sidebar CSS transition (200ms)

    return () => clearTimeout(timer);
  }, [open]);

  return null;
}

export function AppLayout() {
  const isMac = useIsMac();

  return (
    <div className="flex h-screen overflow-hidden">
      {/* Mount traffic lights ONCE in a fixed position — never conditionally
          toggled — to avoid Tauri event listener poisoning on mount/unmount. */}
      {isDesktop && isMac && (
        <div className="fixed top-[14px] left-[14px] z-50">
          <MacTrafficLights />
        </div>
      )}
      <SidebarProvider>
        <SidebarHitTestFix />
        <AppSidebar />
        <SidebarInset>
          <TopBar />
          <main className="flex-1 overflow-auto p-4 pb-20 md:p-6 md:pb-6">
            <Outlet />
          </main>
          {!isDesktop && <MobileNav />}
        </SidebarInset>
        <CommandPalette />
      </SidebarProvider>
    </div>
  );
}
