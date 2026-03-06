import { useState, useEffect, useCallback } from 'react';
import { Button } from '@/components/ui/button';
import { Skeleton } from '@/components/ui/skeleton';
import { useTemplates, useCreateTemplate, useUpdateTemplate } from '@/hooks/useTemplatesApi';
import { DocumentEditor } from '@/components/editor/DocumentEditor';
import { createDefaultDocument } from '@/lib/editor/block-factory';
import type { DocumentModel } from '@/lib/editor/types';
import { DEFAULT_PAGE_SETUP } from '@/lib/editor/types';
import { toast } from 'sonner';
import { Save } from 'lucide-react';

function parseDocModel(json: string): DocumentModel {
  try {
    const parsed = JSON.parse(json);
    return {
      id: parsed.id || 'letterhead-1',
      blocks: Array.isArray(parsed.blocks) ? parsed.blocks : [],
      pageSetup: parsed.pageSetup || { ...DEFAULT_PAGE_SETUP },
      header: Array.isArray(parsed.header) ? parsed.header : [],
      footer: Array.isArray(parsed.footer) ? parsed.footer : [],
    };
  } catch {
    return createDefaultDocument();
  }
}

export function LetterheadPage() {
  const { data: templatesData, isLoading } = useTemplates({
    template_type: 'letterhead',
    per_page: 10,
  });
  const createTemplate = useCreateTemplate();
  const updateTemplate = useUpdateTemplate();

  const letterhead = templatesData?.data?.[0] ?? null;
  const [doc, setDoc] = useState<DocumentModel>(createDefaultDocument());

  useEffect(() => {
    if (letterhead) {
      setDoc(parseDocModel(letterhead.content_json));
    }
  }, [letterhead]);

  const handleDocChange = useCallback((updated: DocumentModel) => {
    setDoc(updated);
  }, []);

  function handleSave() {
    const contentJson = JSON.stringify(doc);
    const headerJson = doc.header.length > 0 ? JSON.stringify(doc.header) : undefined;
    const footerJson = doc.footer.length > 0 ? JSON.stringify(doc.footer) : undefined;
    const pageSetupJson = JSON.stringify(doc.pageSetup);

    if (letterhead) {
      updateTemplate.mutate(
        {
          id: letterhead.id,
          data: {
            name: letterhead.name,
            template_type: 'letterhead',
            content_json: contentJson,
            header_json: headerJson,
            footer_json: footerJson,
            page_setup_json: pageSetupJson,
            is_default: true,
          },
        },
        {
          onSuccess: () => toast.success('Letterhead saved'),
          onError: () => toast.error('Failed to save letterhead'),
        },
      );
    } else {
      createTemplate.mutate(
        {
          name: 'Default Letterhead',
          template_type: 'letterhead',
          content_json: contentJson,
          header_json: headerJson,
          footer_json: footerJson,
          page_setup_json: pageSetupJson,
          is_default: true,
        },
        {
          onSuccess: () => toast.success('Letterhead created'),
          onError: () => toast.error('Failed to create letterhead'),
        },
      );
    }
  }

  const isPending = createTemplate.isPending || updateTemplate.isPending;

  if (isLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-[600px] w-full" />
      </div>
    );
  }

  return (
    <div className="flex flex-col h-[calc(100vh-64px)]">
      {/* Top bar */}
      <div className="flex items-center justify-between px-4 py-3 border-b bg-white shrink-0">
        <div>
          <h2 className="text-lg font-semibold">Letterhead</h2>
          <p className="text-sm text-muted-foreground">
            Configure the default header and footer for all documents
          </p>
        </div>
        <Button size="sm" onClick={handleSave} disabled={isPending}>
          <Save className="mr-1 h-3.5 w-3.5" />
          {isPending ? 'Saving...' : 'Save Letterhead'}
        </Button>
      </div>

      {/* Editor in template mode */}
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
