import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import type { SwissBalanceSheet, GroupedSection } from '@/types/annual-report';

function formatChf(value: number): string {
  const abs = Math.abs(value);
  const parts = abs.toFixed(2).split('.');
  const int = parts[0].replace(/\B(?=(\d{3})+(?!\d))/g, "'");
  const result = `${int}.${parts[1]}`;
  return value < 0 ? `-${result}` : result;
}

function SectionTable({ section }: { section: GroupedSection }) {
  return (
    <div className="mb-4">
      <div className="rounded bg-blue-50 px-3 py-1.5">
        <span className="text-sm font-semibold">{section.label}</span>
      </div>
      {section.groups.map((group) => (
        <div key={group.label} className="ml-2">
          {group.accounts.map((acct) => (
            <div
              key={acct.account_id}
              className="flex items-center justify-between border-b border-gray-100 px-3 py-1 text-sm"
            >
              <span className="text-muted-foreground">
                {acct.account_number} {acct.account_name}
              </span>
              <span className="tabular-nums">{formatChf(acct.balance)}</span>
            </div>
          ))}
          {group.accounts.length > 0 && (
            <div className="flex items-center justify-between border-b px-3 py-1 text-sm font-medium italic">
              <span>{group.total_label}</span>
              <span className="tabular-nums">{formatChf(group.subtotal)}</span>
            </div>
          )}
        </div>
      ))}
      <div className="flex items-center justify-between border-t-2 border-gray-300 px-3 py-1.5 text-sm font-bold">
        <span>Total {section.label}</span>
        <span className="tabular-nums">{formatChf(section.total)}</span>
      </div>
    </div>
  );
}

export function SwissBalanceSheetPreview({
  data,
}: {
  data: SwissBalanceSheet;
}) {
  return (
    <div className="grid gap-4 lg:grid-cols-2">
      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-base">AKTIVEN</CardTitle>
        </CardHeader>
        <CardContent className="space-y-2">
          {data.assets.map((section) => (
            <SectionTable key={section.key} section={section} />
          ))}
          <div className="flex items-center justify-between rounded bg-gray-100 px-3 py-2 text-base font-bold">
            <span>TOTAL AKTIVEN</span>
            <span className="tabular-nums">{formatChf(data.total_assets)}</span>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="pb-2">
          <CardTitle className="text-base">PASSIVEN</CardTitle>
        </CardHeader>
        <CardContent className="space-y-2">
          {data.liabilities.map((section) => (
            <SectionTable key={section.key} section={section} />
          ))}
          <div className="flex items-center justify-between rounded bg-gray-100 px-3 py-2 text-base font-bold">
            <span>TOTAL PASSIVEN</span>
            <span className="tabular-nums">
              {formatChf(data.total_liabilities)}
            </span>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
