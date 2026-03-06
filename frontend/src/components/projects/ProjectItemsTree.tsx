import { useState } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@/components/ui/collapsible';
import { useProjectItems, useDeleteProjectItem } from '@/hooks/useApi';
import { ProjectItemDialog } from './ProjectItemDialog';
import { ChevronRight, ChevronDown, Plus, Pencil, Trash2, Layers, Package, CheckSquare } from 'lucide-react';
import { toast } from 'sonner';
import { useI18n } from '@/i18n';
import type { ProjectItem, ProjectItemType } from '@/types/project-item';

interface ProjectItemsTreeProps {
  projectId: string;
}

const statusVariant: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
  pending: 'outline',
  in_progress: 'default',
  completed: 'secondary',
  cancelled: 'destructive',
};

const typeIcon: Record<ProjectItemType, typeof Layers> = {
  phase: Layers,
  work_package: Package,
  task: CheckSquare,
};

interface TreeNodeProps {
  item: ProjectItem;
  depth: number;
  allItems: ProjectItem[];
  onEdit: (item: ProjectItem) => void;
  onDelete: (itemId: string) => void;
  onAdd: (parentId: string, type: ProjectItemType) => void;
}

function TreeNode({ item, depth, allItems, onEdit, onDelete, onAdd }: TreeNodeProps) {
  const { t } = useI18n();
  const [expanded, setExpanded] = useState(true);
  const Icon = typeIcon[item.item_type];
  const hasChildren = item.children && item.children.length > 0;

  const childType: ProjectItemType | null =
    item.item_type === 'phase' ? 'work_package' :
    item.item_type === 'work_package' ? 'task' : null;

  const childLabel =
    item.item_type === 'phase' ? t('projects.add_work_package', 'Add Work Package') :
    item.item_type === 'work_package' ? t('projects.add_task', 'Add Task') : null;

  return (
    <div>
      <Collapsible open={expanded} onOpenChange={setExpanded}>
        <div
          className="flex items-center gap-2 py-1.5 px-2 rounded-md hover:bg-accent/50 group"
          style={{ paddingLeft: `${depth * 24 + 8}px` }}
        >
          {hasChildren ? (
            <CollapsibleTrigger asChild>
              <Button variant="ghost" size="icon" className="h-6 w-6 shrink-0">
                {expanded ? <ChevronDown className="h-3.5 w-3.5" /> : <ChevronRight className="h-3.5 w-3.5" />}
              </Button>
            </CollapsibleTrigger>
          ) : (
            <span className="w-6 shrink-0" />
          )}

          <Icon className="h-4 w-4 text-muted-foreground shrink-0" />

          <span className="text-sm font-medium flex-1 truncate">{item.name}</span>

          <Badge variant={statusVariant[item.status] ?? 'outline'} className="text-xs shrink-0">
            {item.status.replace('_', ' ')}
          </Badge>

          {item.estimated_hours != null && (
            <span className="text-xs text-muted-foreground font-mono shrink-0">
              {item.estimated_hours}h
            </span>
          )}

          {item.due_date && (
            <span className="text-xs text-muted-foreground font-mono shrink-0 hidden md:inline">
              {item.due_date}
            </span>
          )}

          <div className="flex gap-0.5 opacity-0 group-hover:opacity-100 transition-opacity shrink-0">
            {childType && (
              <Button variant="ghost" size="icon" className="h-6 w-6" onClick={() => onAdd(item.id, childType)} title={childLabel ?? ''}>
                <Plus className="h-3 w-3" />
              </Button>
            )}
            <Button variant="ghost" size="icon" className="h-6 w-6" onClick={() => onEdit(item)} title={t('common.edit', 'Edit')}>
              <Pencil className="h-3 w-3" />
            </Button>
            <Button variant="ghost" size="icon" className="h-6 w-6" onClick={() => onDelete(item.id)} title={t('common.delete', 'Delete')}>
              <Trash2 className="h-3 w-3" />
            </Button>
          </div>
        </div>

        {hasChildren && (
          <CollapsibleContent>
            {item.children!.map((child) => (
              <TreeNode
                key={child.id}
                item={child}
                depth={depth + 1}
                allItems={allItems}
                onEdit={onEdit}
                onDelete={onDelete}
                onAdd={onAdd}
              />
            ))}
          </CollapsibleContent>
        )}
      </Collapsible>
    </div>
  );
}

export function ProjectItemsTree({ projectId }: ProjectItemsTreeProps) {
  const { t } = useI18n();
  const { data: items, isLoading } = useProjectItems(projectId);
  const deleteItem = useDeleteProjectItem();

  const [dialogOpen, setDialogOpen] = useState(false);
  const [editItem, setEditItem] = useState<ProjectItem | null>(null);
  const [addParentId, setAddParentId] = useState<string | undefined>();
  const [addDefaultType, setAddDefaultType] = useState<ProjectItemType>('phase');

  const list = items ?? [];

  function handleEdit(item: ProjectItem) {
    setEditItem(item);
    setAddParentId(undefined);
    setDialogOpen(true);
  }

  function handleAdd(parentId: string, type: ProjectItemType) {
    setEditItem(null);
    setAddParentId(parentId);
    setAddDefaultType(type);
    setDialogOpen(true);
  }

  function handleAddPhase() {
    setEditItem(null);
    setAddParentId(undefined);
    setAddDefaultType('phase');
    setDialogOpen(true);
  }

  function handleDelete(itemId: string) {
    if (!confirm(t('projects.confirm_delete_item', 'Delete this item and all its children?'))) return;
    deleteItem.mutate(itemId, {
      onSuccess: () => toast.success(t('projects.item_deleted', 'Item deleted')),
      onError: () => toast.error(t('projects.item_delete_failed', 'Failed to delete item')),
    });
  }

  if (isLoading) return <p className="text-sm text-muted-foreground py-4">{t('common.loading', 'Loading...')}</p>;

  return (
    <div className="space-y-4">
      <div className="flex justify-end">
        <Button size="sm" onClick={handleAddPhase}>
          <Plus className="mr-1 h-4 w-4" /> {t('projects.add_phase', 'Add Phase')}
        </Button>
      </div>

      <Card>
        <CardContent className="p-2">
          {list.length > 0 ? (
            <div className="space-y-0.5">
              {list.map((item) => (
                <TreeNode
                  key={item.id}
                  item={item}
                  depth={0}
                  allItems={list}
                  onEdit={handleEdit}
                  onDelete={handleDelete}
                  onAdd={handleAdd}
                />
              ))}
            </div>
          ) : (
            <p className="py-6 text-center text-sm text-muted-foreground">
              {t('projects.no_items', 'No work breakdown structure. Add your first phase.')}
            </p>
          )}
        </CardContent>
      </Card>

      <ProjectItemDialog
        projectId={projectId}
        item={editItem}
        parentId={addParentId}
        defaultType={addDefaultType}
        phases={list}
        open={dialogOpen}
        onOpenChange={setDialogOpen}
      />
    </div>
  );
}
