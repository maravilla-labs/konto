import type { Block } from '../../lib/editor/types';
import { InspectorTextPanel } from './InspectorTextPanel';
import { InspectorTablePanel } from './InspectorTablePanel';
import { InspectorContactPanel } from './InspectorContactPanel';
import { InspectorDocMetaPanel } from './InspectorDocMetaPanel';
import { InspectorImagePanel } from './InspectorImagePanel';
import { InspectorSignaturePanel } from './InspectorSignaturePanel';
import { InspectorSimplePanel } from './InspectorSimplePanel';

export interface EditorContext {
  contacts: {
    id: string;
    name: string;
    company?: string;
    address?: string;
    city?: string;
    postal_code?: string;
    country: string;
  }[];
  projects: { id: string; name: string }[];
  templates?: { id: string; name: string; is_default: boolean }[];
  docType?: string;
}

interface BlockInspectorProps {
  block: Block | null;
  onChange: (block: Block) => void;
  templateMode?: boolean;
  editorContext?: EditorContext;
}

export function BlockInspector({
  block,
  onChange,
  templateMode,
  editorContext,
}: BlockInspectorProps) {
  return (
    <div className="w-72 border-l bg-white flex flex-col overflow-hidden">
      <div className="flex-1 overflow-y-auto p-4 text-sm">
        {block ? (
          <InspectorPanel
            block={block}
            onChange={onChange}
            templateMode={templateMode}
            editorContext={editorContext}
          />
        ) : (
          <NoSelectionPanel editorContext={editorContext} />
        )}
      </div>
    </div>
  );
}

function InspectorPanel({
  block,
  onChange,
  templateMode,
  editorContext,
}: {
  block: Block;
  onChange: (block: Block) => void;
  templateMode?: boolean;
  editorContext?: EditorContext;
}) {
  switch (block.type) {
    case 'h1':
    case 'h2':
    case 'h3':
    case 'p':
    case 'blockquote':
      return (
        <InspectorTextPanel
          block={block}
          onChange={onChange}
          templateMode={templateMode}
        />
      );
    case 'table':
      return (
        <InspectorTablePanel
          block={block}
          onChange={onChange}
          templateMode={templateMode}
        />
      );
    case 'contact_info':
      return (
        <InspectorContactPanel
          block={block}
          onChange={onChange}
          templateMode={templateMode}
          contacts={editorContext?.contacts ?? []}
        />
      );
    case 'doc_meta':
      return (
        <InspectorDocMetaPanel
          block={block}
          onChange={onChange}
          templateMode={templateMode}
          projects={editorContext?.projects ?? []}
        />
      );
    case 'image':
      return (
        <InspectorImagePanel
          block={block}
          onChange={onChange}
          templateMode={templateMode}
        />
      );
    case 'signature':
      return (
        <InspectorSignaturePanel
          block={block}
          onChange={onChange}
          templateMode={templateMode}
        />
      );
    case 'spacer':
    case 'divider':
    case 'placeholder':
      return (
        <InspectorSimplePanel
          block={block}
          onChange={onChange}
          templateMode={templateMode}
        />
      );
    default:
      return (
        <div className="text-sm text-gray-400 text-center">
          Unknown block type
        </div>
      );
  }
}

function NoSelectionPanel({
  editorContext,
}: {
  editorContext?: EditorContext;
}) {
  return (
    <div className="space-y-4">
      <p className="text-sm text-gray-400 text-center">
        Select a block to inspect
      </p>

      {editorContext?.templates && editorContext.templates.length > 0 && (
        <div>
          <span className="text-xs text-gray-500 block mb-1">Templates</span>
          <ul className="text-xs text-gray-600 space-y-1">
            {editorContext.templates.map((tpl) => (
              <li key={tpl.id}>
                {tpl.name}
                {tpl.is_default ? ' (Default)' : ''}
              </li>
            ))}
          </ul>
        </div>
      )}

      {editorContext?.docType && (
        <div>
          <span className="text-xs text-gray-500 block mb-1">Document Type</span>
          <span className="text-xs font-mono capitalize">{editorContext.docType}</span>
        </div>
      )}
    </div>
  );
}
