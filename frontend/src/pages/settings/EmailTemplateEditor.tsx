import { useState, useRef, useCallback, useEffect } from 'react';
import {
  useUpdateEmailTemplate,
  usePreviewEmailTemplate,
} from '@/hooks/useEmailTemplatesApi';
import type { EmailTemplate, EmailTemplateType } from '@/types/email-template';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Save, Eye, RotateCcw } from 'lucide-react';
import { toast } from 'sonner';

const TEMPLATE_TYPE_LABELS: Record<EmailTemplateType, string> = {
  invoice_send: 'Invoice Sending',
  invoice_reminder_1: 'Payment Reminder - Level 1',
  invoice_reminder_2: 'Payment Reminder - Level 2',
  invoice_reminder_3: 'Payment Reminder - Level 3',
  credit_note: 'Credit Note',
  document_send: 'Document Sending',
};

const VARIABLES = [
  'company_name',
  'contact_name',
  'contact_email',
  'invoice_number',
  'credit_note_number',
  'document_number',
  'amount',
  'currency',
  'due_date',
  'today',
  'invoice_date',
];

const SAMPLE_DATA: Record<string, string> = {
  company_name: 'Acme GmbH',
  contact_name: 'Max Mustermann',
  contact_email: 'max@example.com',
  invoice_number: 'RE-2026-001',
  credit_note_number: 'CN-2026-001',
  document_number: 'AN-00001',
  amount: "1'250.00",
  currency: 'CHF',
  due_date: '2026-04-01',
  today: new Date().toISOString().slice(0, 10),
  invoice_date: '2026-03-01',
};

function renderLocally(text: string): string {
  let result = text;
  for (const [key, val] of Object.entries(SAMPLE_DATA)) {
    result = result.replaceAll(`{{${key}}}`, val);
  }
  return result;
}

export function EmailTemplateEditor({
  template,
  onClose,
}: {
  template: EmailTemplate;
  onClose: () => void;
}) {
  const [subject, setSubject] = useState(template.subject);
  const [bodyHtml, setBodyHtml] = useState(template.body_html);
  const bodyRef = useRef<HTMLTextAreaElement>(null);
  const updateMut = useUpdateEmailTemplate();
  const previewMut = usePreviewEmailTemplate();

  const [previewSubject, setPreviewSubject] = useState('');
  const [previewBody, setPreviewBody] = useState('');

  const fetchPreview = useCallback(() => {
    previewMut.mutate(template.id, {
      onSuccess: (data) => {
        setPreviewSubject(data.rendered_subject);
        setPreviewBody(data.rendered_body_html);
      },
    });
  }, [template.id]);

  useEffect(() => {
    fetchPreview();
  }, []);

  useEffect(() => {
    const timer = setTimeout(() => {
      setPreviewSubject(renderLocally(subject));
      setPreviewBody(renderLocally(bodyHtml));
    }, 300);
    return () => clearTimeout(timer);
  }, [subject, bodyHtml]);

  function insertVariable(variable: string) {
    const el = bodyRef.current;
    if (!el) return;
    const start = el.selectionStart;
    const end = el.selectionEnd;
    const text = `{{${variable}}}`;
    const newBody = bodyHtml.slice(0, start) + text + bodyHtml.slice(end);
    setBodyHtml(newBody);
    requestAnimationFrame(() => {
      el.focus();
      el.setSelectionRange(start + text.length, start + text.length);
    });
  }

  function handleSave() {
    updateMut.mutate(
      { id: template.id, data: { subject, body_html: bodyHtml } },
      {
        onSuccess: () => {
          toast.success('Template saved');
          onClose();
        },
        onError: () => toast.error('Failed to save template'),
      },
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h2 className="text-lg font-semibold">
          {TEMPLATE_TYPE_LABELS[template.template_type]} —{' '}
          {template.language.toUpperCase()}
        </h2>
        <Button variant="ghost" onClick={onClose}>
          Back
        </Button>
      </div>

      <div className="grid gap-6 lg:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle className="text-sm">Editor</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div>
              <Label>Subject</Label>
              <Input
                value={subject}
                onChange={(e) => setSubject(e.target.value)}
              />
            </div>

            <div>
              <Label>Variables</Label>
              <div className="mt-1 flex flex-wrap gap-1">
                {VARIABLES.map((v) => (
                  <Button
                    key={v}
                    variant="outline"
                    size="sm"
                    className="h-7 text-xs"
                    onClick={() => insertVariable(v)}
                  >
                    {`{{${v}}}`}
                  </Button>
                ))}
              </div>
            </div>

            <div>
              <Label>Body</Label>
              <textarea
                ref={bodyRef}
                value={bodyHtml}
                onChange={(e) => setBodyHtml(e.target.value)}
                className="mt-1 w-full rounded-md border bg-background p-3 font-mono text-sm"
                rows={14}
              />
            </div>

            <div className="flex gap-2">
              <Button onClick={handleSave} disabled={updateMut.isPending}>
                <Save className="mr-2 h-4 w-4" />
                Save
              </Button>
              <Button
                variant="outline"
                onClick={() => {
                  setSubject(template.subject);
                  setBodyHtml(template.body_html);
                }}
              >
                <RotateCcw className="mr-2 h-4 w-4" />
                Reset
              </Button>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <div className="flex items-center gap-2">
              <Eye className="h-4 w-4" />
              <CardTitle className="text-sm">Preview</CardTitle>
            </div>
          </CardHeader>
          <CardContent>
            <div className="rounded-md border p-4">
              <p className="mb-3 text-sm font-semibold">
                Subject: {previewSubject}
              </p>
              <hr className="mb-3" />
              <pre className="whitespace-pre-wrap text-sm">{previewBody}</pre>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
