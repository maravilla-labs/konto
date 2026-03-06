import { useState } from 'react';
import { useEmailTemplates } from '@/hooks/useEmailTemplatesApi';
import type { EmailTemplate, EmailTemplateType } from '@/types/email-template';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Skeleton } from '@/components/ui/skeleton';
import { Badge } from '@/components/ui/badge';
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from '@/components/ui/collapsible';
import { ChevronDown } from 'lucide-react';
import { EmailTemplateEditor } from './EmailTemplateEditor';

const TEMPLATE_TYPE_LABELS: Record<EmailTemplateType, string> = {
  invoice_send: 'Invoice Sending',
  invoice_reminder_1: 'Payment Reminder - Level 1',
  invoice_reminder_2: 'Payment Reminder - Level 2',
  invoice_reminder_3: 'Payment Reminder - Level 3',
  credit_note: 'Credit Note',
  document_send: 'Document Sending',
};

const TYPE_ORDER: EmailTemplateType[] = [
  'invoice_send',
  'invoice_reminder_1',
  'invoice_reminder_2',
  'invoice_reminder_3',
  'credit_note',
  'document_send',
];

export function EmailTemplatesPage() {
  const { data: templates, isLoading } = useEmailTemplates();
  const [editing, setEditing] = useState<EmailTemplate | null>(null);

  if (isLoading) {
    return (
      <div className="space-y-4 p-6">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-40 w-full" />
        <Skeleton className="h-40 w-full" />
      </div>
    );
  }

  const grouped = (templates ?? []).reduce(
    (acc, t) => {
      if (!acc[t.template_type]) acc[t.template_type] = [];
      acc[t.template_type].push(t);
      return acc;
    },
    {} as Record<string, EmailTemplate[]>,
  );

  return (
    <div className="space-y-6 p-6">
      <h1 className="text-2xl font-bold">Email Templates</h1>

      {editing ? (
        <EmailTemplateEditor
          template={editing}
          onClose={() => setEditing(null)}
        />
      ) : (
        <div className="space-y-3">
          {TYPE_ORDER.map((type) => {
            const items = grouped[type];
            if (!items) return null;
            return (
              <TemplateGroup
                key={type}
                type={type}
                templates={items}
                onEdit={setEditing}
              />
            );
          })}
        </div>
      )}
    </div>
  );
}

function TemplateGroup({
  type,
  templates,
  onEdit,
}: {
  type: EmailTemplateType;
  templates: EmailTemplate[];
  onEdit: (t: EmailTemplate) => void;
}) {
  const [open, setOpen] = useState(false);

  return (
    <Collapsible open={open} onOpenChange={setOpen}>
      <Card>
        <CollapsibleTrigger asChild>
          <CardHeader className="cursor-pointer hover:bg-muted/50">
            <div className="flex items-center justify-between">
              <CardTitle className="text-base">
                {TEMPLATE_TYPE_LABELS[type]}
              </CardTitle>
              <ChevronDown
                className={`h-4 w-4 transition-transform ${open ? 'rotate-180' : ''}`}
              />
            </div>
          </CardHeader>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <CardContent className="pt-0">
            <div className="space-y-2">
              {templates.map((t) => (
                <div
                  key={t.id}
                  className="flex items-center justify-between rounded-md border p-3"
                >
                  <div className="min-w-0 flex-1">
                    <div className="flex items-center gap-2">
                      <Badge variant="outline">
                        {t.language.toUpperCase()}
                      </Badge>
                      <span className="truncate text-sm font-medium">
                        {t.subject}
                      </span>
                      {t.is_default && (
                        <Badge variant="secondary" className="text-xs">
                          Default
                        </Badge>
                      )}
                    </div>
                  </div>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => onEdit(t)}
                  >
                    Edit
                  </Button>
                </div>
              ))}
            </div>
          </CardContent>
        </CollapsibleContent>
      </Card>
    </Collapsible>
  );
}
