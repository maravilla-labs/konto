import { useState } from 'react';
import { Card } from '@/components/ui/card';
import { ContactBrowserPanel } from './ContactBrowserPanel';
import { PersonsPanel } from './PersonsPanel';

export function ContactBrowser() {
  const [selectedCompanyId, setSelectedCompanyId] = useState<string | null>(null);

  return (
    <div className="grid h-[calc(100vh-16rem)] grid-cols-5 gap-4">
      <Card className="col-span-3 overflow-hidden">
        <ContactBrowserPanel
          onSelectCompany={setSelectedCompanyId}
          selectedCompanyId={selectedCompanyId}
        />
      </Card>
      <Card className="col-span-2 overflow-hidden">
        <PersonsPanel companyId={selectedCompanyId} />
      </Card>
    </div>
  );
}
