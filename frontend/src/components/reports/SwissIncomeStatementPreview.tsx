import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import type { SwissIncomeStatement } from '@/types/annual-report';

function formatChf(value: number): string {
  const abs = Math.abs(value);
  const parts = abs.toFixed(2).split('.');
  const int = parts[0].replace(/\B(?=(\d{3})+(?!\d))/g, "'");
  const result = `${int}.${parts[1]}`;
  return value < 0 ? `-${result}` : result;
}

export function SwissIncomeStatementPreview({
  data,
}: {
  data: SwissIncomeStatement;
}) {
  const st = data.subtotals;

  const subtotalAfter: Record<string, { label: string; value: number }> = {
    operating_revenue: { label: 'Betriebsertrag', value: st.operating_revenue },
    material_expense: { label: 'Bruttoergebnis nach Material', value: st.gross_profit_material },
    personnel_expense: { label: 'Bruttoergebnis nach Personal', value: st.gross_profit_personnel },
    other_opex: { label: 'EBITDA', value: st.ebitda },
    depreciation: { label: 'EBIT', value: st.ebit },
    financial_result: { label: 'EBT', value: st.ebt },
  };

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-base">
          ERFOLGSRECHNUNG {data.from_date} bis {data.to_date}
        </CardTitle>
      </CardHeader>
      <CardContent>
        {data.sections.map((section) => (
          <div key={section.key} className="mb-2">
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
                {group.accounts.length > 1 && (
                  <div className="flex items-center justify-between border-b px-3 py-1 text-sm font-medium italic">
                    <span>{group.total_label}</span>
                    <span className="tabular-nums">{formatChf(group.subtotal)}</span>
                  </div>
                )}
              </div>
            ))}
            <div className="flex items-center justify-between border-t px-3 py-1 text-sm font-bold">
              <span>Total {section.label}</span>
              <span className="tabular-nums">{formatChf(section.total)}</span>
            </div>

            {subtotalAfter[section.key] && (
              <div className="flex items-center justify-between rounded bg-indigo-50 px-3 py-1.5 text-sm font-bold">
                <span>{subtotalAfter[section.key].label}</span>
                <span className="tabular-nums">
                  {formatChf(subtotalAfter[section.key].value)}
                </span>
              </div>
            )}
          </div>
        ))}

        <div className="mt-2 flex items-center justify-between rounded bg-gray-100 px-3 py-2 text-base font-bold">
          <span>JAHRESERGEBNIS</span>
          <span className="tabular-nums">{formatChf(st.net_result)}</span>
        </div>
      </CardContent>
    </Card>
  );
}
