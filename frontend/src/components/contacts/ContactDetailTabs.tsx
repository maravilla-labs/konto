import { Link } from 'react-router-dom';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Badge } from '@/components/ui/badge';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Plus, X } from 'lucide-react';
import type { ContactPerson } from '@/types/contacts';
import type { PaginatedResponse } from '@/types/common';
import type { DocumentSummary } from '@/types/contacts';
import type { TimeEntry } from '@/types/projects';

interface PersonsTabProps {
  persons: ContactPerson[] | undefined;
  personOpen: boolean;
  setPersonOpen: (v: boolean) => void;
  personForm: {
    first_name: string;
    last_name: string;
    email: string;
    phone: string;
    department: string;
    position: string;
  };
  setPersonForm: (v: PersonsTabProps['personForm']) => void;
  handleCreatePerson: () => void;
  createPersonPending: boolean;
  onDeletePerson: (id: string) => void;
}

export function ContactPersonsTab({
  persons,
  personOpen,
  setPersonOpen,
  personForm,
  setPersonForm,
  handleCreatePerson,
  createPersonPending,
  onDeletePerson,
}: PersonsTabProps) {
  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between">
        <CardTitle className="text-sm">Contact Persons</CardTitle>
        <Dialog open={personOpen} onOpenChange={setPersonOpen}>
          <DialogTrigger asChild>
            <Button size="sm"><Plus className="mr-1 h-4 w-4" /> Add Person</Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader><DialogTitle>Add Contact Person</DialogTitle></DialogHeader>
            <div className="space-y-3">
              <div className="grid grid-cols-2 gap-3">
                <div><Label>First Name</Label><Input value={personForm.first_name} onChange={(e) => setPersonForm({ ...personForm, first_name: e.target.value })} /></div>
                <div><Label>Last Name</Label><Input value={personForm.last_name} onChange={(e) => setPersonForm({ ...personForm, last_name: e.target.value })} /></div>
              </div>
              <div><Label>Email</Label><Input value={personForm.email} onChange={(e) => setPersonForm({ ...personForm, email: e.target.value })} /></div>
              <div><Label>Phone</Label><Input value={personForm.phone} onChange={(e) => setPersonForm({ ...personForm, phone: e.target.value })} /></div>
              <div className="grid grid-cols-2 gap-3">
                <div><Label>Department</Label><Input value={personForm.department} onChange={(e) => setPersonForm({ ...personForm, department: e.target.value })} /></div>
                <div><Label>Position</Label><Input value={personForm.position} onChange={(e) => setPersonForm({ ...personForm, position: e.target.value })} /></div>
              </div>
              <Button onClick={handleCreatePerson} className="w-full" disabled={createPersonPending}>Add Person</Button>
            </div>
          </DialogContent>
        </Dialog>
      </CardHeader>
      <CardContent className="p-0">
        {persons && persons.length > 0 ? (
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Name</TableHead>
                <TableHead>Email</TableHead>
                <TableHead className="hidden md:table-cell">Phone</TableHead>
                <TableHead className="hidden lg:table-cell">Position</TableHead>
                <TableHead className="w-16" />
              </TableRow>
            </TableHeader>
            <TableBody>
              {persons.map((p) => (
                <TableRow key={p.id}>
                  <TableCell className="font-medium">{p.first_name} {p.last_name}</TableCell>
                  <TableCell>{p.email ?? '\u2014'}</TableCell>
                  <TableCell className="hidden md:table-cell">{p.phone ?? '\u2014'}</TableCell>
                  <TableCell className="hidden lg:table-cell">{p.position ?? '\u2014'}</TableCell>
                  <TableCell>
                    <Button variant="ghost" size="icon" onClick={() => onDeletePerson(p.id)}>
                      <X className="h-3.5 w-3.5" />
                    </Button>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        ) : (
          <p className="py-6 text-center text-sm text-muted-foreground">No contact persons yet.</p>
        )}
      </CardContent>
    </Card>
  );
}

export function ContactInvoicesTab({ data }: { data: PaginatedResponse<unknown> | undefined }) {
  return (
    <Card>
      <CardContent className="p-0">
        {data && data.data.length > 0 ? (
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Number</TableHead>
                <TableHead>Status</TableHead>
                <TableHead className="text-right">Total</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {(data.data as Array<{ id: string; invoice_number?: string; status: string; total: string }>).map((inv) => (
                <TableRow key={inv.id}>
                  <TableCell>
                    <Link to={`/invoices/${inv.id}`} className="text-primary hover:underline font-medium">
                      {inv.invoice_number ?? 'Draft'}
                    </Link>
                  </TableCell>
                  <TableCell><Badge variant="secondary">{inv.status}</Badge></TableCell>
                  <TableCell className="text-right font-mono">{Number(inv.total).toFixed(2)}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        ) : (
          <p className="py-6 text-center text-sm text-muted-foreground">No invoices for this contact.</p>
        )}
      </CardContent>
    </Card>
  );
}

export function ContactDocumentsTab({ data }: { data: PaginatedResponse<DocumentSummary> | undefined }) {
  return (
    <Card>
      <CardContent className="p-0">
        {data && data.data.length > 0 ? (
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Number</TableHead>
                <TableHead>Title</TableHead>
                <TableHead>Type</TableHead>
                <TableHead>Status</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {data.data.map((doc) => (
                <TableRow key={doc.id}>
                  <TableCell className="font-mono">{doc.doc_number ?? '\u2014'}</TableCell>
                  <TableCell>
                    <Link to={`/documents/${doc.id}`} className="text-primary hover:underline">
                      {doc.title}
                    </Link>
                  </TableCell>
                  <TableCell><Badge variant="outline">{doc.doc_type}</Badge></TableCell>
                  <TableCell><Badge variant="secondary">{doc.status}</Badge></TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        ) : (
          <p className="py-6 text-center text-sm text-muted-foreground">No documents for this contact.</p>
        )}
      </CardContent>
    </Card>
  );
}

export function ContactTimeEntriesTab({ data }: { data: PaginatedResponse<TimeEntry> | undefined }) {
  return (
    <Card>
      <CardContent className="p-0">
        {data && data.data.length > 0 ? (
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Date</TableHead>
                <TableHead>Hours</TableHead>
                <TableHead>Description</TableHead>
                <TableHead>Billed</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {data.data.map((te) => (
                <TableRow key={te.id}>
                  <TableCell className="font-mono">{te.date}</TableCell>
                  <TableCell className="font-mono">{(te.actual_minutes / 60).toFixed(1)}h</TableCell>
                  <TableCell className="max-w-xs truncate">{te.description ?? '\u2014'}</TableCell>
                  <TableCell>{te.billed ? <Badge>Billed</Badge> : <Badge variant="outline">Unbilled</Badge>}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        ) : (
          <p className="py-6 text-center text-sm text-muted-foreground">No time entries for this contact.</p>
        )}
      </CardContent>
    </Card>
  );
}
