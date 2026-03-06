# Frontend Components

## shadcn/ui Components (installed)
- avatar, badge, button, card, dialog, dropdown-menu, form, input, label
- select, separator, sheet, sidebar, skeleton, sonner, table, tabs, tooltip

## Layout Components
- `AppLayout` - Main layout with sidebar + outlet
- `Sidebar` - Navigation sidebar with logo, menu items, user info
- `TopBar` - Top bar with breadcrumbs and user menu
- `MobileNav` - Bottom navigation for mobile

## Auth Components
- `LoginForm` - Email + password form with zod validation
- `AuthGuard` - Route guard (redirects to /login if unauthenticated)

## Pages
- `LoginPage` - Centered login card with Maravilla logo
- `DashboardPage` - Welcome card + summary stats
- `AccountsPage` - Chart of accounts tree view with add/edit dialog
- `ContactsPage` - Searchable contact table with CRUD
- `JournalPage` - Journal entries with date filter + create form
- `ProjectsPage` - Project list with status badges
- `TimeEntriesPage` - Time entries list view
- `ImportPage` - Multi-step wizard (upload → preview → execute → result)

## API Client Modules
- `client.ts` - Axios instance with JWT interceptor
- `auth.ts`, `accounts.ts`, `contacts.ts`, `journal.ts`, `projects.ts`, `imports.ts`

## Hooks
- `useAuth` - Auth store wrapper
- `useApi` - TanStack React Query hooks for all domains
- `use-mobile` - Mobile breakpoint detection

## Stores
- `authStore` - Zustand store (user, tokens, login/logout/refresh)
