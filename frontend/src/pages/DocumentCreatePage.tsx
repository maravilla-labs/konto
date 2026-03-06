import { useState, useCallback } from 'react';
import { useNavigate, useSearchParams } from 'react-router-dom';
import { Button } from '@/components/ui/button';
import { useContacts, useProjects } from '@/hooks/useApi';
import { useTemplates } from '@/hooks/useTemplatesApi';
import { useCreateDocument } from '@/hooks/useDocumentsApi';
import { DocumentEditor } from '@/components/editor/DocumentEditor';
import type { EditorContext } from '@/components/editor/BlockInspector';
import { createDefaultDocument } from '@/lib/editor/block-factory';
import type { DocumentModel } from '@/lib/editor/types';
import { DEFAULT_PAGE_SETUP } from '@/lib/editor/types';
import { extractDocMeta, extractLines } from '@/lib/editor/doc-sync';
import { toast } from 'sonner';
import { ArrowLeft } from 'lucide-react';
import { SUPPORTED_LANGUAGES } from '@/lib/language';
import { useI18n } from '@/i18n';

function parseDocModel(json: string): DocumentModel {
  try {
    const parsed = JSON.parse(json);
    return {
      id: parsed.id || 'doc-new',
      blocks: Array.isArray(parsed.blocks) ? parsed.blocks : [],
      pageSetup: parsed.pageSetup || { ...DEFAULT_PAGE_SETUP },
      header: Array.isArray(parsed.header) ? parsed.header : [],
      footer: Array.isArray(parsed.footer) ? parsed.footer : [],
    };
  } catch {
    return createDefaultDocument();
  }
}

export function DocumentCreatePage() {
  const { language: uiLanguage, t } = useI18n();
  const navigate = useNavigate();
  const [params] = useSearchParams();
  const docType = params.get('type') || 'quote';
  const createDoc = useCreateDocument();

  const { data: contactsData } = useContacts({ per_page: 200 });
  const { data: projectsData } = useProjects({ per_page: 200 });
  const { data: templatesData } = useTemplates({ template_type: docType, per_page: 100 });

  const contacts = contactsData?.data ?? [];
  const projects = projectsData?.data ?? [];
  const templates = templatesData?.data ?? [];

  const [templateId, setTemplateId] = useState('');
  const [documentLanguage, setDocumentLanguage] = useState<string>(uiLanguage);
  const [doc, setDoc] = useState<DocumentModel>(createDefaultDocument());

  const handleDocChange = useCallback((updated: DocumentModel) => {
    setDoc(updated);
  }, []);

  function handleSelectTemplate(tplId: string) {
    setTemplateId(tplId);
    const tpl = templates.find((t) => t.id === tplId);
    if (tpl) {
      setDoc(parseDocModel(tpl.content_json));
    }
  }

  function handleSave() {
    const meta = extractDocMeta(doc.blocks);
    const lines = extractLines(doc.blocks);

    createDoc.mutate(
      {
        doc_type: docType,
        title: meta.title,
        contact_id: meta.contactId,
        project_id: meta.projectId || undefined,
        template_id: templateId || undefined,
        content_json: JSON.stringify(doc),
        language: documentLanguage || undefined,
        valid_until: meta.validUntil || undefined,
        lines,
      },
      {
        onSuccess: (res) => {
          toast.success(t('documents.created', 'Document created'));
          navigate(`/documents/${res.data.id}`);
        },
        onError: () => toast.error(t('documents.create_failed', 'Failed to create document')),
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
    templates,
    docType,
  };

  return (
    <div className="flex flex-col h-[calc(100vh-64px)]">
      <div className="flex items-center gap-3 px-4 py-2 border-b bg-white shrink-0">
        <Button variant="ghost" size="sm" onClick={() => navigate(-1)}>
          <ArrowLeft className="mr-1 h-3.5 w-3.5" />
          {t('common.cancel', 'Cancel')}
        </Button>
        <div className="flex-1">
          <span className="text-sm font-medium">
            {t('documents.new_type_prefix', 'New')} {t(`documents.type.${docType}`, docType)}
          </span>
        </div>
        {templates.length > 0 && (
          <select
            value={templateId}
            onChange={(e) => handleSelectTemplate(e.target.value)}
            className="text-xs border rounded px-2 py-1 bg-white"
          >
            <option value="">{t('documents.select_template', 'Select template...')}</option>
            {templates.map((tpl) => (
              <option key={tpl.id} value={tpl.id}>
                {tpl.name} {tpl.is_default ? `(${t('documents.default', 'Default')})` : ''}
              </option>
            ))}
          </select>
        )}
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
          disabled={createDoc.isPending || !canSave}
        >
          {createDoc.isPending
            ? t('documents.creating', 'Creating...')
            : t('documents.save_as_draft', 'Save as Draft')}
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
