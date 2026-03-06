import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';

const placeholderGroups = [
  {
    label: 'Company',
    items: [
      { key: '{{company_name}}', label: 'Company Name' },
      { key: '{{company_address}}', label: 'Address' },
      { key: '{{company_city}}', label: 'City' },
      { key: '{{company_postal_code}}', label: 'Postal Code' },
      { key: '{{company_country}}', label: 'Country' },
      { key: '{{company_vat}}', label: 'VAT Number' },
      { key: '{{company_email}}', label: 'Email' },
      { key: '{{company_phone}}', label: 'Phone' },
      { key: '{{company_website}}', label: 'Website' },
    ],
  },
  {
    label: 'Client',
    items: [
      { key: '{{client_name}}', label: 'Client Name' },
      { key: '{{client_contact}}', label: 'Contact Person' },
      { key: '{{client_address}}', label: 'Address' },
      { key: '{{client_city}}', label: 'City' },
      { key: '{{client_postal_code}}', label: 'Postal Code' },
      { key: '{{client_country}}', label: 'Country' },
      { key: '{{client_email}}', label: 'Email' },
    ],
  },
  {
    label: 'Document',
    items: [
      { key: '{{doc_number}}', label: 'Document Number' },
      { key: '{{doc_date}}', label: 'Document Date' },
      { key: '{{doc_title}}', label: 'Title' },
      { key: '{{valid_until}}', label: 'Valid Until' },
      { key: '{{subtotal}}', label: 'Subtotal' },
      { key: '{{vat_amount}}', label: 'VAT Amount' },
      { key: '{{total}}', label: 'Total' },
    ],
  },
];

interface PlaceholderPickerProps {
  onInsert: (placeholder: string) => void;
}

export function PlaceholderPicker({ onInsert }: PlaceholderPickerProps) {
  return (
    <div className="space-y-3">
      {placeholderGroups.map((group) => (
        <Card key={group.label}>
          <CardHeader className="py-2 px-3">
            <CardTitle className="text-xs font-medium text-muted-foreground">
              {group.label}
            </CardTitle>
          </CardHeader>
          <CardContent className="px-3 pb-2">
            <div className="flex flex-wrap gap-1">
              {group.items.map((item) => (
                <Button
                  key={item.key}
                  variant="outline"
                  size="sm"
                  className="h-6 text-xs"
                  onClick={() => onInsert(item.key)}
                >
                  {item.label}
                </Button>
              ))}
            </div>
          </CardContent>
        </Card>
      ))}
    </div>
  );
}
