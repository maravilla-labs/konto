import { useState, useEffect, useCallback } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Button } from '@/components/ui/button';
import { Skeleton } from '@/components/ui/skeleton';
import { useContacts, useProjects } from '@/hooks/useApi';
import { useDocument, useUpdateDocument } from '@/hooks/useDocumentsApi';
import { DocumentEditor } from '@/components/editor/DocumentEditor';
import type { EditorContext } from '@/components/editor/BlockInspector';
import { createDefaultDocument } from '@/lib/editor/block-factory';
import type { DocumentModel } from '@/lib/editor/types';
import { DEFAULT_PAGE_SETUP } from '@/lib/editor/types';
import { extractDocMeta, extractLines, injectDocMeta } from '@/lib/editor/doc-sync';
import { toast } from 'sonner';
import { ArrowLeft } from 'lucide-react';
import { SUPPORTED_LANGUAGES } from '@/lib/language';
import { useI18n } from '@/i18n';

function parseDocModel(json: string): DocumentModel {
  try {
    const parsed = JSON.parse(json);
    return {
      id: parsed.id || 'doc-edit',
      blocks: Array.isArray(parsed.blocks) ? parsed.blocks : [],
      pageSetup: parsed.pageSetup || { ...DEFAULT_PAGE_SETUP },
      header: Array.isArray(parsed.header) ? parsed.header : [],
      footer: Array.isArray(parsed.footer) ? parsed.footer : [],
    };
  } catch {
    return createDefaultDocument();
  }
}

export function DocumentEditPage() {
  const { language: uiLanguage, t } = useI18n();
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { data, isLoading } = useDocument(id);
  const updateDoc = useUpdateDocument();

  const { data: contactsData } = useContacts({ per_page: 200 });
  const { data: projectsData } = useProjects({ per_page: 200 });
  const contacts = contactsData?.data ?? [];
  const projects = projectsData?.data ?? [];

  const [doc, setDoc] = useState<DocumentModel>(createDefaultDocument());
  const [documentLanguage, setDocumentLanguage] = useState<string>(uiLanguage);
  const [hydrated, setHydrated] = useState(false);

  useEffect(() => {
    if (data && contacts.length > 0 && !hydrated) {
      const parsed = parseDocModel(data.content_json);
      const injected = injectDocMeta(parsed, {
        contact_id: data.contact_id,
        project_id: data.project_id,
        valid_until: data.valid_until,
        doc_number: data.doc_number,
        lines: data.lines,
      }, contacts, projects);
      setDoc(injected);
      setDocumentLanguage(data.language ?? uiLanguage);
      setHydrated(true);
    }
  }, [data, contacts, projects, hydrated, uiLanguage]);

  const handleDocChange = useCallback((updated: DocumentModel) => {
    setDoc(updated);
  }, []);

  if (isLoading) {
    return (
      <div className="space-y-4 p-8">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-[600px] w-full" />
      </div>
    );
  }

  if (!data) {
    return <p className="text-center text-muted-foreground p-8">{t('documents.not_found', 'Document not found.')}</p>;
  }

  if (data.status !== 'draft') {
    return <p className="text-center text-muted-foreground p-8">{t('documents.only_draft_editable', 'Only draft documents can be edited.')}</p>;
  }

  function handleSave() {
    const meta = extractDocMeta(doc.blocks);
    const lines = extractLines(doc.blocks);

    updateDoc.mutate(
      {
        id: id!,
        data: {
          doc_type: data!.doc_type,
          title: meta.title,
          contact_id: meta.contactId,
          project_id: meta.projectId || undefined,
          template_id: data!.template_id,
          content_json: JSON.stringify(doc),
          language: documentLanguage || undefined,
          valid_until: meta.validUntil || undefined,
          lines,
        },
      },
      {
        onSuccess: () => {
          toast.success(t('documents.updated', 'Document updated'));
          navigate(`/documents/${id}`);
        },
        onError: () => toast.error(t('documents.update_failed', 'Failed to update document')),
      },
    );
  }

  const meta = extractDocMeta(doc.blocks);
  const canSave = !!meta.title && !!meta.contactId;

  const editorContext: EditorContext = {
    contacts: contacts.map((c) => ({
      id: c.id,
      name: c.name1,
      company: c.name2,
      address: c.address,
      city: c.city,
      postal_code: c.postal_code,
      country: c.country ?? 'CH',
    })),
    projects,
    docType: data.doc_type,
  };

  return (
    <div className="flex flex-col h-[calc(100vh-64px)]">
      <div className="flex items-center gap-3 px-4 py-2 border-b bg-white shrink-0">
        <Button variant="ghost" size="sm" onClick={() => navigate(-1)}>
          <ArrowLeft className="mr-1 h-3.5 w-3.5" />
          {t('common.previous', 'Previous')}
        </Button>
        <div className="flex-1">
          <span className="text-sm font-medium">
            {t('documents.edit_prefix', 'Edit')}: {data.title}
          </span>
          <span className="ml-2 text-xs text-muted-foreground">
            {data.doc_number ?? t('invoices.draft_label', 'DRAFT')} — {data.doc_type}
          </span>
        </div>
        <select
          value={documentLanguage || '__auto__'}
          onChange={(e) => setDocumentLanguage(e.target.value === '__auto__' ? '' : e.target.value)}
          className="text-xs border rounded px-2 py-1 bg-white"
        >
          <option value="__auto__">{t('documents.language_automatic', 'Language: Automatic')}</option>
          {SUPPORTED_LANGUAGES.map((lang) => (
            <option key={lang.code} value={lang.code}>
              {lang.label}
            </option>
          ))}
        </select>
        <Button
          size="sm"
          onClick={handleSave}
          disabled={updateDoc.isPending || !canSave}
        >
          {updateDoc.isPending ? t('documents.saving', 'Saving...') : t('common.save_changes', 'Save Changes')}
        </Button>
      </div>

      <div className="flex-1 overflow-hidden">
        <DocumentEditor
          value={doc}
          onChange={handleDocChange}
          editorContext={editorContext}
        />
      </div>
    </div>
  );
}
