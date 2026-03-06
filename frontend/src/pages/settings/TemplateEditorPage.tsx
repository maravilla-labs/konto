import { useState, useEffect, useCallback } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Checkbox } from '@/components/ui/checkbox';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import {
  useTemplate,
  useCreateTemplate,
  useUpdateTemplate,
} from '@/hooks/useTemplatesApi';
import { DocumentEditor } from '@/components/editor/DocumentEditor';
import { createDefaultDocument } from '@/lib/editor/block-factory';
import type { DocumentModel } from '@/lib/editor/types';
import { DEFAULT_PAGE_SETUP } from '@/lib/editor/types';
import { toast } from 'sonner';
import { Save } from 'lucide-react';

const templateTypes = [
  { value: 'invoice', label: 'Invoice' },
  { value: 'quote', label: 'Quote' },
  { value: 'offer', label: 'Offer' },
  { value: 'sow', label: 'SOW' },
  { value: 'contract', label: 'Contract' },
  { value: 'letterhead', label: 'Letterhead' },
];

function parseDocModel(json: string): DocumentModel {
  try {
    const parsed = JSON.parse(json);
    return {
      id: parsed.id || 'doc-1',
      blocks: Array.isArray(parsed.blocks) ? parsed.blocks : [],
      pageSetup: parsed.pageSetup || { ...DEFAULT_PAGE_SETUP },
      header: Array.isArray(parsed.header) ? parsed.header : [],
      footer: Array.isArray(parsed.footer) ? parsed.footer : [],
    };
  } catch {
    return createDefaultDocument();
  }
}

export function TemplateEditorPage() {
  const { id } = useParams<{ id: string }>();
  const isNew = !id;
  const navigate = useNavigate();
  const { data, isLoading } = useTemplate(id);
  const createTemplate = useCreateTemplate();
  const updateTemplate = useUpdateTemplate();

  const [name, setName] = useState('');
  const [templateType, setTemplateType] = useState('invoice');
  const [isDefault, setIsDefault] = useState(false);
  const [doc, setDoc] = useState<DocumentModel>(createDefaultDocument());

  useEffect(() => {
    if (data) {
      setName(data.name);
      setTemplateType(data.template_type);
      setIsDefault(data.is_default);
      setDoc(parseDocModel(data.content_json));
    }
  }, [data]);

  const handleDocChange = useCallback((updated: DocumentModel) => {
    setDoc(updated);
  }, []);

  function handleSave() {
    const contentJson = JSON.stringify(doc);
    const headerJson = doc.header.length > 0 ? JSON.stringify(doc.header) : undefined;
    const footerJson = doc.footer.length > 0 ? JSON.stringify(doc.footer) : undefined;
    const pageSetupJson = JSON.stringify(doc.pageSetup);

    const payload = {
      name,
      template_type: templateType,
      content_json: contentJson,
      header_json: headerJson,
      footer_json: footerJson,
      page_setup_json: pageSetupJson,
      is_default: isDefault,
    };

    if (isNew) {
      createTemplate.mutate(payload, {
        onSuccess: (res) => {
          toast.success('Template created');
          navigate(`/settings/templates/${res.data.id}`);
        },
        onError: () => toast.error('Failed to create template'),
      });
    } else {
      updateTemplate.mutate(
        { id: id!, data: payload },
        {
          onSuccess: () => toast.success('Template saved'),
          onError: () => toast.error('Failed to save template'),
        },
      );
    }
  }

  if (!isNew && isLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-[600px] w-full" />
      </div>
    );
  }

  const isPending = createTemplate.isPending || updateTemplate.isPending;

  return (
    <div className="flex flex-col h-[calc(100vh-64px)]">
      {/* Top bar: metadata + save */}
      <div className="flex items-center gap-4 px-4 py-3 border-b bg-white shrink-0">
        <div className="flex items-center gap-3 flex-1">
          <div className="min-w-[200px]">
            <Label className="text-xs text-muted-foreground">Template Name</Label>
            <Input
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="e.g. Standard Invoice"
              className="h-8"
            />
          </div>
          <div className="min-w-[140px]">
            <Label className="text-xs text-muted-foreground">Type</Label>
            <Select value={templateType} onValueChange={setTemplateType}>
              <SelectTrigger className="h-8"><SelectValue /></SelectTrigger>
              <SelectContent>
                {templateTypes.map((t) => (
                  <SelectItem key={t.value} value={t.value}>{t.label}</SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <div className="flex items-center gap-2 pt-4">
            <Checkbox
              checked={isDefault}
              onCheckedChange={(v) => setIsDefault(!!v)}
            />
            <Label className="text-xs">Default</Label>
          </div>
        </div>
        <Button size="sm" onClick={handleSave} disabled={isPending || !name}>
          <Save className="mr-1 h-3.5 w-3.5" />
          {isPending ? 'Saving...' : 'Save'}
        </Button>
      </div>

      {/* Full-height editor with templateMode */}
      <div className="flex-1 overflow-hidden">
        <DocumentEditor
          value={doc}
          onChange={handleDocChange}
          templateMode
        />
      </div>
    </div>
  );
}
