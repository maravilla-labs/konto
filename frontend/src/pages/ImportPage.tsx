import { useState, useRef } from 'react';
import { useUploadImport, useImportPreview, useExecuteImport } from '@/hooks/useApi';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Label } from '@/components/ui/label';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Upload, FileUp, CheckCircle2, AlertCircle, Loader2 } from 'lucide-react';
import { toast } from 'sonner';
import type { ImportType, ImportResult } from '@/types/imports';

type Step = 'upload' | 'preview' | 'executing' | 'result';

const IMPORT_LABELS: Record<ImportType, string> = {
  accounts: 'Accounts (CSV)',
  contacts: 'Contacts (CSV)',
  time_entries: 'Time Entries (CSV)',
  projects: 'Projects (XLSX)',
  journal: 'Journal Entries (XLSX)',
};

export function ImportPage() {
  const [step, setStep] = useState<Step>('upload');
  const [importType, setImportType] = useState<ImportType>('contacts');
  const [batchId, setBatchId] = useState<string | null>(null);
  const [result, setResult] = useState<ImportResult | null>(null);
  const fileRef = useRef<HTMLInputElement>(null);

  const uploadMutation = useUploadImport();
  const { data: preview, isLoading: previewLoading } = useImportPreview(batchId);
  const executeMutation = useExecuteImport();

  function handleFileSelect(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    if (!file) return;
    uploadMutation.mutate(
      { file, importType },
      {
        onSuccess: (res) => {
          setBatchId(res.data.id);
          setStep('preview');
          toast.success('File uploaded, previewing data...');
        },
        onError: () => toast.error('Upload failed'),
      }
    );
  }

  function handleExecute() {
    if (!batchId) return;
    setStep('executing');
    executeMutation.mutate(batchId, {
      onSuccess: (res) => {
        setResult(res.data);
        setStep('result');
        toast.success('Import completed');
      },
      onError: () => {
        setStep('preview');
        toast.error('Import failed');
      },
    });
  }

  function handleReset() {
    setStep('upload');
    setBatchId(null);
    setResult(null);
    if (fileRef.current) fileRef.current.value = '';
  }

  // Derive columns from preview rows
  const previewRows = preview?.preview ?? [];
  const columns = previewRows.length > 0 ? Object.keys(previewRows[0]) : [];

  const steps: { key: Step; label: string }[] = [
    { key: 'upload', label: 'Upload' },
    { key: 'preview', label: 'Preview' },
    { key: 'executing', label: 'Import' },
    { key: 'result', label: 'Result' },
  ];
  const currentIdx = steps.findIndex((s) => s.key === step);

  return (
    <div className="space-y-4">
      <div>
        <h2 className="text-lg font-semibold">Import Data</h2>
        <p className="text-sm text-muted-foreground">
          Import accounts, contacts, time entries, projects, and journal entries from CSV/XLSX files.
        </p>
      </div>

      {/* Progress bar */}
      <div className="flex items-center gap-1">
        {steps.map((s, i) => (
          <div key={s.key} className="flex items-center gap-1">
            <div className={`flex h-7 items-center gap-1.5 rounded-full px-3 text-xs font-medium ${
              i < currentIdx ? 'bg-primary/10 text-primary' :
              i === currentIdx ? 'bg-primary text-primary-foreground' :
              'bg-muted text-muted-foreground'
            }`}>
              <span className="font-bold">{i + 1}</span>
              {s.label}
            </div>
            {i < steps.length - 1 && (
              <div className={`h-px w-6 ${i < currentIdx ? 'bg-primary' : 'bg-border'}`} />
            )}
          </div>
        ))}
      </div>

      {step === 'upload' && (
        <Card>
          <CardHeader>
            <CardTitle className="text-base">Upload File</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div>
              <Label>Import Type</Label>
              <Select value={importType} onValueChange={(v) => setImportType(v as ImportType)}>
                <SelectTrigger className="w-60">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {(Object.entries(IMPORT_LABELS) as [ImportType, string][]).map(([key, label]) => (
                    <SelectItem key={key} value={key}>{label}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div
              className="flex cursor-pointer flex-col items-center gap-2 rounded-lg border-2 border-dashed p-8 text-center transition hover:border-primary hover:bg-primary/5"
              onClick={() => fileRef.current?.click()}
            >
              <Upload className="h-8 w-8 text-muted-foreground" />
              <p className="text-sm font-medium">Click to upload a file</p>
              <p className="text-xs text-muted-foreground">CSV or XLSX</p>
              <input
                ref={fileRef}
                type="file"
                accept=".csv,.xlsx,.xls"
                className="hidden"
                onChange={handleFileSelect}
              />
            </div>
            {uploadMutation.isPending && (
              <div className="flex items-center gap-2 text-sm text-muted-foreground">
                <Loader2 className="h-4 w-4 animate-spin" />
                Uploading...
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {step === 'preview' && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2 text-base">
              <FileUp className="h-4 w-4" /> Preview
            </CardTitle>
          </CardHeader>
          <CardContent>
            {previewLoading ? (
              <div className="flex items-center gap-2 py-8 text-sm text-muted-foreground">
                <Loader2 className="h-4 w-4 animate-spin" />
                Loading preview...
              </div>
            ) : preview && previewRows.length > 0 ? (
              <div className="space-y-4">
                <p className="text-sm text-muted-foreground">
                  Total records: <strong>{preview.total}</strong> — Showing first {previewRows.length}
                </p>
                <div className="max-h-96 overflow-auto rounded border">
                  <Table>
                    <TableHeader>
                      <TableRow>
                        <TableHead className="sticky top-0 bg-background">#</TableHead>
                        {columns.map((col) => (
                          <TableHead key={col} className="sticky top-0 bg-background whitespace-nowrap">
                            {col}
                          </TableHead>
                        ))}
                      </TableRow>
                    </TableHeader>
                    <TableBody>
                      {previewRows.map((row, i) => (
                        <TableRow key={i}>
                          <TableCell className="text-muted-foreground">{i + 1}</TableCell>
                          {columns.map((col) => (
                            <TableCell key={col} className="max-w-40 truncate text-xs">
                              {String(row[col] ?? '')}
                            </TableCell>
                          ))}
                        </TableRow>
                      ))}
                    </TableBody>
                  </Table>
                </div>
                <div className="flex gap-2">
                  <Button onClick={handleExecute}>Import {preview.total} Records</Button>
                  <Button variant="outline" onClick={handleReset}>Cancel</Button>
                </div>
              </div>
            ) : (
              <p className="text-sm text-muted-foreground">No preview data available.</p>
            )}
          </CardContent>
        </Card>
      )}

      {step === 'executing' && (
        <Card>
          <CardContent className="flex flex-col items-center gap-3 py-12">
            <Loader2 className="h-8 w-8 animate-spin text-primary" />
            <p className="text-sm font-medium">Importing data...</p>
          </CardContent>
        </Card>
      )}

      {step === 'result' && result && (
        <Card>
          <CardContent className="space-y-4 py-6">
            <div className="flex flex-col items-center gap-2 text-center">
              {(result.error_rows ?? 0) === 0 ? (
                <CheckCircle2 className="h-10 w-10 text-green-600" />
              ) : (result.imported_rows ?? 0) > 0 ? (
                <AlertCircle className="h-10 w-10 text-amber-500" />
              ) : (
                <AlertCircle className="h-10 w-10 text-destructive" />
              )}
              <h3 className="text-lg font-semibold">
                {(result.error_rows ?? 0) === 0
                  ? 'Import Complete'
                  : (result.imported_rows ?? 0) > 0
                    ? 'Import Completed with Errors'
                    : 'Import Failed'}
              </h3>
            </div>

            {/* Summary stats */}
            <div className="flex justify-center gap-6 text-sm">
              <div className="flex flex-col items-center">
                <span className="text-2xl font-bold text-green-600">{result.imported_rows ?? 0}</span>
                <span className="text-muted-foreground">Imported</span>
              </div>
              <div className="flex flex-col items-center">
                <span className="text-2xl font-bold text-destructive">{result.error_rows ?? 0}</span>
                <span className="text-muted-foreground">Errors</span>
              </div>
              <div className="flex flex-col items-center">
                <span className="text-2xl font-bold">{result.total_rows ?? 0}</span>
                <span className="text-muted-foreground">Total</span>
              </div>
            </div>

            {/* Progress bar */}
            {(result.total_rows ?? 0) > 0 && (
              <div className="mx-auto w-64">
                <div className="h-2 overflow-hidden rounded-full bg-muted">
                  <div
                    className="h-full bg-green-600 transition-all"
                    style={{ width: `${((result.imported_rows ?? 0) / (result.total_rows ?? 1)) * 100}%` }}
                  />
                </div>
              </div>
            )}

            {/* Error details */}
            {result.error_log && result.error_log.length > 0 && (
              <div className="mx-auto max-w-lg">
                <p className="mb-2 text-sm font-medium">Error Details:</p>
                <div className="max-h-48 overflow-auto rounded border bg-muted/50 p-3">
                  {result.error_log.map((err, i) => (
                    <p key={i} className="text-xs text-destructive">
                      {err}
                    </p>
                  ))}
                </div>
              </div>
            )}

            <div className="flex justify-center">
              <Button onClick={handleReset}>Import Another File</Button>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
