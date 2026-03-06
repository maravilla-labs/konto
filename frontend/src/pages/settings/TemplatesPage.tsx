import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import { Tabs, TabsList, TabsTrigger, TabsContent } from '@/components/ui/tabs';
import {
  useTemplates,
  useDeleteTemplate,
  useDuplicateTemplate,
} from '@/hooks/useTemplatesApi';
import { toast } from 'sonner';
import { Plus, Copy, Pencil, Trash2 } from 'lucide-react';
import type { DocumentTemplate } from '@/types/template';

const typeTabs = [
  { value: 'all', label: 'All' },
  { value: 'invoice', label: 'Invoice' },
  { value: 'quote', label: 'Quote' },
  { value: 'offer', label: 'Offer' },
  { value: 'sow', label: 'SOW' },
  { value: 'contract', label: 'Contract' },
  { value: 'letterhead', label: 'Letterhead' },
];

const typeVariant: Record<string, 'default' | 'secondary' | 'outline'> = {
  invoice: 'default',
  quote: 'secondary',
  offer: 'secondary',
  sow: 'outline',
  contract: 'outline',
  letterhead: 'default',
};

export function TemplatesPage() {
  const [typeFilter, setTypeFilter] = useState('all');
  const { data, isLoading } = useTemplates(
    typeFilter !== 'all' ? { template_type: typeFilter, per_page: 100 } : { per_page: 100 },
  );
  const deleteTemplate = useDeleteTemplate();
  const duplicateTemplate = useDuplicateTemplate();
  const navigate = useNavigate();

  const templates = data?.data ?? [];

  function handleDuplicate(id: string) {
    duplicateTemplate.mutate(id, {
      onSuccess: (res) => {
        toast.success('Template duplicated');
        navigate(`/settings/templates/${res.data.id}`);
      },
      onError: () => toast.error('Failed to duplicate'),
    });
  }

  function handleDelete(id: string) {
    deleteTemplate.mutate(id, {
      onSuccess: () => toast.success('Template deleted'),
      onError: () => toast.error('Failed to delete'),
    });
  }

  return (
    <div className="space-y-4">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-lg font-semibold">Templates</h2>
          <p className="text-sm text-muted-foreground">
            Manage document templates
          </p>
        </div>
        <Button asChild size="sm">
          <Link to="/settings/templates/new">
            <Plus className="mr-1 h-4 w-4" /> New Template
          </Link>
        </Button>
      </div>

      <Tabs value={typeFilter} onValueChange={setTypeFilter}>
        <TabsList>
          {typeTabs.map((t) => (
            <TabsTrigger key={t.value} value={t.value}>
              {t.label}
            </TabsTrigger>
          ))}
        </TabsList>

        {typeTabs.map((t) => (
          <TabsContent key={t.value} value={t.value}>
            {isLoading ? (
              <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                {Array.from({ length: 6 }).map((_, i) => (
                  <Skeleton key={i} className="h-32 w-full" />
                ))}
              </div>
            ) : templates.length > 0 ? (
              <TemplateGrid
                templates={templates}
                onDuplicate={handleDuplicate}
                onDelete={handleDelete}
              />
            ) : (
              <p className="py-8 text-center text-sm text-muted-foreground">
                No templates found.
              </p>
            )}
          </TabsContent>
        ))}
      </Tabs>
    </div>
  );
}

function TemplateGrid({
  templates,
  onDuplicate,
  onDelete,
}: {
  templates: DocumentTemplate[];
  onDuplicate: (id: string) => void;
  onDelete: (id: string) => void;
}) {
  return (
    <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
      {templates.map((tpl) => (
        <Card key={tpl.id}>
          <CardContent className="py-4">
            <div className="flex items-start justify-between">
              <div className="min-w-0">
                <p className="truncate font-medium">{tpl.name}</p>
                <div className="mt-1 flex gap-1">
                  <Badge variant={typeVariant[tpl.template_type] ?? 'outline'}>
                    {tpl.template_type}
                  </Badge>
                  {tpl.is_default && <Badge variant="outline">Default</Badge>}
                </div>
              </div>
              <div className="flex gap-1">
                <Button asChild variant="ghost" size="icon" className="h-8 w-8">
                  <Link to={`/settings/templates/${tpl.id}`}>
                    <Pencil className="h-3.5 w-3.5" />
                  </Link>
                </Button>
                <Button
                  variant="ghost"
                  size="icon"
                  className="h-8 w-8"
                  onClick={() => onDuplicate(tpl.id)}
                >
                  <Copy className="h-3.5 w-3.5" />
                </Button>
                <Button
                  variant="ghost"
                  size="icon"
                  className="h-8 w-8"
                  onClick={() => onDelete(tpl.id)}
                >
                  <Trash2 className="h-3.5 w-3.5" />
                </Button>
              </div>
            </div>
            <p className="mt-2 text-xs text-muted-foreground">
              Updated {tpl.updated_at}
            </p>
          </CardContent>
        </Card>
      ))}
    </div>
  );
}
