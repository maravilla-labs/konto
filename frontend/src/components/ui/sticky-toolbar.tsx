import { useEffect, useRef, useState } from 'react';
import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { MoreHorizontal, Loader2 } from 'lucide-react';

export interface ToolbarAction {
  icon: React.ReactNode;
  label: string;
  onClick: () => void;
  variant?: 'default' | 'destructive' | 'outline' | 'secondary' | 'ghost';
  disabled?: boolean;
  loading?: boolean;
  /** Show as primary (filled) button; default is outline */
  primary?: boolean;
}

export interface ToolbarOverflowItem {
  icon: React.ReactNode;
  label: string;
  onClick: () => void;
  destructive?: boolean;
  separator?: boolean;
}

interface StickyToolbarProps {
  /** Left side: title area (typically h2 + badge) */
  children: React.ReactNode;
  /** Visible action buttons (icon-only when stuck, icon+label at rest) */
  actions?: ToolbarAction[];
  /** Overflow menu items behind the three-dot button */
  overflow?: ToolbarOverflowItem[];
  className?: string;
}

/**
 * A reusable sticky toolbar that sits at the top of a detail page.
 * When the user scrolls past it, it becomes stuck with a glassmorphism
 * background and compresses to icon-only buttons with tooltips.
 */
export function StickyToolbar({
  children,
  actions = [],
  overflow = [],
  className,
}: StickyToolbarProps) {
  const toolbarRef = useRef<HTMLDivElement>(null);
  const [stuck, setStuck] = useState(false);

  useEffect(() => {
    const el = toolbarRef.current;
    if (!el) return;

    // Find the scrollable ancestor (<main> with overflow-auto/scroll)
    let scrollParent: HTMLElement | null = el.parentElement;
    while (scrollParent) {
      const style = getComputedStyle(scrollParent);
      const oy = style.overflowY;
      if (oy === 'auto' || oy === 'scroll') break;
      scrollParent = scrollParent.parentElement;
    }
    if (!scrollParent) return;

    function onScroll() {
      // Toolbar absorbs main padding via negative margin, so it sticks almost immediately
      setStuck(scrollParent!.scrollTop > 8);
    }

    scrollParent.addEventListener('scroll', onScroll, { passive: true });
    onScroll();
    return () => scrollParent!.removeEventListener('scroll', onScroll);
  }, []);

  const hasOverflow = overflow.length > 0;

  return (
    <div
      ref={toolbarRef}
      className={cn(
        'sticky z-30 -mx-4 px-4 md:-mx-6 md:px-6',
        'transition-all duration-300 ease-out',
        stuck
          ? '-top-4 md:-top-6 pt-2 md:pt-3 bg-white/30 dark:bg-zinc-950/30 backdrop-blur-xl border-b border-white/20 dark:border-zinc-800/40 shadow-sm pb-1.5'
          : 'top-0 bg-transparent py-1',
        className,
      )}
    >
        <div className={cn(
          'flex items-center justify-between transition-all duration-300 ease-out',
          stuck ? 'gap-2' : 'gap-3',
        )}>
          <div className="flex items-center gap-3 min-w-0">{children}</div>

          <TooltipProvider delayDuration={300}>
            <div className={cn(
              'flex items-center flex-shrink-0 transition-all duration-300 ease-out',
              stuck ? 'gap-1' : 'gap-1.5',
            )}>
              {actions.map((action, i) => (
                <ActionButton key={i} action={action} compact={stuck} />
              ))}

              {hasOverflow && (
                <DropdownMenu>
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <DropdownMenuTrigger asChild>
                        <Button variant="outline" size="icon" className="h-8 w-8">
                          <MoreHorizontal className="h-4 w-4" />
                        </Button>
                      </DropdownMenuTrigger>
                    </TooltipTrigger>
                    <TooltipContent side="bottom">More actions</TooltipContent>
                  </Tooltip>
                  <DropdownMenuContent align="end">
                    {overflow.map((item, i) => (
                      <span key={i}>
                        {item.separator && <DropdownMenuSeparator />}
                        <DropdownMenuItem
                          onClick={item.onClick}
                          className={item.destructive ? 'text-destructive' : undefined}
                        >
                          {item.icon}
                          <span className="ml-2">{item.label}</span>
                        </DropdownMenuItem>
                      </span>
                    ))}
                  </DropdownMenuContent>
                </DropdownMenu>
              )}
            </div>
          </TooltipProvider>
        </div>
    </div>
  );
}

function ActionButton({ action, compact }: { action: ToolbarAction; compact: boolean }) {
  const variant = action.primary ? 'default' : (action.variant ?? 'outline');
  const isLoading = action.loading;

  const btn = (
    <Button
      variant={variant}
      size={compact ? 'icon' : 'sm'}
      className={cn(
        'transition-all duration-300 ease-out overflow-hidden inline-flex items-center justify-center',
        compact ? 'h-8 w-8 px-0' : undefined,
      )}
      onClick={action.onClick}
      disabled={action.disabled || isLoading}
    >
      {isLoading ? (
        <Loader2 className="h-4 w-4 animate-spin shrink-0" />
      ) : (
        <span className="inline-flex shrink-0">{action.icon}</span>
      )}
      {!compact && <span className="ml-1.5">{action.label}</span>}
    </Button>
  );

  if (compact) {
    return (
      <Tooltip>
        <TooltipTrigger asChild>{btn}</TooltipTrigger>
        <TooltipContent side="bottom">{action.label}</TooltipContent>
      </Tooltip>
    );
  }

  return btn;
}
