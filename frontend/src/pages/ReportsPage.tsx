import { Link } from 'react-router-dom';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import {
  Scale,
  Building2,
  TrendingUp,
  BookMarked,
  Receipt,
  Banknote,
  ClipboardList,
  FileOutput,
  FileBarChart,
} from 'lucide-react';

const reports = [
  {
    to: '/reports/trial-balance',
    title: 'Trial Balance',
    description: 'Overview of all account balances with debit and credit totals.',
    icon: Scale,
  },
  {
    to: '/reports/balance-sheet',
    title: 'Balance Sheet',
    description: 'Assets, liabilities, and equity at a point in time.',
    icon: Building2,
  },
  {
    to: '/reports/profit-loss',
    title: 'Profit & Loss',
    description: 'Revenue and expenses over a period to see net result.',
    icon: TrendingUp,
  },
  {
    to: '/reports/cash-flow',
    title: 'Cash Flow',
    description: 'Operating, investing, and financing cash flow analysis.',
    icon: Banknote,
  },
  {
    to: '/reports/vat-report',
    title: 'VAT Report',
    description: 'Swiss VAT summary grouped by tax code for filing.',
    icon: Receipt,
  },
  {
    to: '/reports/ar-aging',
    title: 'AR Aging',
    description: 'Outstanding receivables grouped by age buckets.',
    icon: ClipboardList,
  },
  {
    to: '/reports/ap-aging',
    title: 'AP Aging',
    description: 'Outstanding payables grouped by age buckets.',
    icon: FileOutput,
  },
  {
    to: '/reports/account-ledger',
    title: 'Account Ledger',
    description: 'Detailed transaction history for individual accounts.',
    icon: BookMarked,
  },
  {
    to: '/reports/annual-report',
    title: 'Jahresrechnung',
    description: 'Swiss annual report in legal format (OR Art. 957-962).',
    icon: FileBarChart,
  },
  {
    to: '/reports/salary-certificates',
    title: 'Salary Certificates',
    description: 'Annual salary certificates (ESTV Formular 11).',
    icon: FileOutput,
  },
];

export function ReportsPage() {
  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-semibold">Reports</h2>
        <p className="text-sm text-muted-foreground">
          Generate financial reports and statements
        </p>
      </div>
      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {reports.map((report) => (
          <Link key={report.to} to={report.to} className="group">
            <Card className="transition-colors group-hover:border-primary/50">
              <CardHeader className="flex flex-row items-center gap-3 pb-2">
                <div className="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-muted group-hover:bg-primary/10">
                  <report.icon className="h-5 w-5 text-muted-foreground group-hover:text-primary" />
                </div>
                <CardTitle className="text-base">{report.title}</CardTitle>
              </CardHeader>
              <CardContent>
                <p className="text-sm text-muted-foreground">{report.description}</p>
              </CardContent>
            </Card>
          </Link>
        ))}
      </div>
    </div>
  );
}
