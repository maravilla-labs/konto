import type { Block, DocMetaData } from '../../lib/editor/types';
import { InspectorCommon, Field } from './InspectorCommon';

interface InspectorDocMetaPanelProps {
  block: Block;
  onChange: (block: Block) => void;
  templateMode?: boolean;
  projects: { id: string; name: string }[];
}

export function InspectorDocMetaPanel({
  block,
  onChange,
  templateMode,
  projects,
}: InspectorDocMetaPanelProps) {
  const data = block.data as DocMetaData;

  function updateData(updates: Partial<DocMetaData>) {
    onChange({
      ...block,
      data: { ...data, ...updates } as DocMetaData,
    });
  }

  function handleProjectChange(projectId: string) {
    const project = projects.find((p) => p.id === projectId);
    updateData({
      projectId,
      projectName: project?.name ?? '',
    });
  }

  return (
    <div className="space-y-4">
      <Field label="Type">
        <span className="text-sm font-mono">doc_meta</span>
      </Field>

      <Field label="Document Date">
        <input
          type="date"
          value={data.docDate}
          onChange={(e) => updateData({ docDate: e.target.value })}
          className="w-full border rounded px-2 py-1 text-sm"
        />
      </Field>

      <Field label="Valid Until">
        <input
          type="date"
          value={data.validUntil}
          onChange={(e) => updateData({ validUntil: e.target.value })}
          className="w-full border rounded px-2 py-1 text-sm"
        />
      </Field>

      <Field label="Project">
        <select
          value={data.projectId}
          onChange={(e) => handleProjectChange(e.target.value)}
          className="w-full border rounded px-2 py-1 text-sm"
        >
          <option value="">None</option>
          {projects.map((p) => (
            <option key={p.id} value={p.id}>{p.name}</option>
          ))}
        </select>
      </Field>

      <Field label="Document Number">
        <span className="text-sm font-mono text-gray-600">{data.docNumber || 'DRAFT'}</span>
      </Field>

      <InspectorCommon block={block} onChange={onChange} templateMode={templateMode} />
    </div>
  );
}
