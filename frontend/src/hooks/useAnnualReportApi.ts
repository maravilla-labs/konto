import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import {
  shareholdersApi,
  annualReportNotesApi,
  annualReportApi,
  swissReportsApi,
} from '@/api/annual-report';
import type { CreateShareholder, UpdateShareholder, UpdateNoteRequest, CreateNoteRequest } from '@/types/annual-report';

// Shareholders
export function useShareholders() {
  return useQuery({
    queryKey: ['shareholders'],
    queryFn: () => shareholdersApi.list().then((r) => r.data),
  });
}

export function useCreateShareholder() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateShareholder) => shareholdersApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['shareholders'] }),
  });
}

export function useUpdateShareholder() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateShareholder }) =>
      shareholdersApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['shareholders'] }),
  });
}

export function useDeleteShareholder() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => shareholdersApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['shareholders'] }),
  });
}

// Annual Report Notes
export function useAnnualReportNotes(fiscalYearId: string | undefined) {
  return useQuery({
    queryKey: ['annual-report-notes', fiscalYearId],
    queryFn: () =>
      annualReportNotesApi.list(fiscalYearId!).then((r) => r.data),
    enabled: !!fiscalYearId,
  });
}

export function useUpdateNote() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({
      fiscalYearId,
      section,
      data,
    }: {
      fiscalYearId: string;
      section: string;
      data: UpdateNoteRequest;
    }) => annualReportNotesApi.update(fiscalYearId, section, data),
    onSuccess: (_, vars) =>
      qc.invalidateQueries({
        queryKey: ['annual-report-notes', vars.fiscalYearId],
      }),
  });
}

export function useCreateNote() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({
      fiscalYearId,
      data,
    }: {
      fiscalYearId: string;
      data: CreateNoteRequest;
    }) => annualReportNotesApi.create(fiscalYearId, data),
    onSuccess: (_, vars) =>
      qc.invalidateQueries({
        queryKey: ['annual-report-notes', vars.fiscalYearId],
      }),
  });
}

export function useDeleteNote() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({
      fiscalYearId,
      sectionKey,
    }: {
      fiscalYearId: string;
      sectionKey: string;
    }) => annualReportNotesApi.delete(fiscalYearId, sectionKey),
    onSuccess: (_, vars) =>
      qc.invalidateQueries({
        queryKey: ['annual-report-notes', vars.fiscalYearId],
      }),
  });
}

// Annual Report
export function useAnnualReport(fiscalYearId: string | undefined) {
  return useQuery({
    queryKey: ['annual-report', fiscalYearId],
    queryFn: () => annualReportApi.get(fiscalYearId!).then((r) => r.data),
    enabled: !!fiscalYearId,
  });
}

export function useGenerateAnnualReport() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (fiscalYearId: string) => annualReportApi.generate(fiscalYearId),
    onSuccess: (_, fiscalYearId) =>
      qc.invalidateQueries({ queryKey: ['annual-report', fiscalYearId] }),
  });
}

export function useFinalizeAnnualReport() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (fiscalYearId: string) => annualReportApi.finalize(fiscalYearId),
    onSuccess: (_, fiscalYearId) =>
      qc.invalidateQueries({ queryKey: ['annual-report', fiscalYearId] }),
  });
}

// Swiss Reports
export function useSwissBalanceSheet(asOf: string | undefined) {
  return useQuery({
    queryKey: ['swiss-balance-sheet', asOf],
    queryFn: () => swissReportsApi.balanceSheet(asOf!).then((r) => r.data),
    enabled: !!asOf,
  });
}

export function useSwissIncomeStatement(
  fromDate: string | undefined,
  toDate: string | undefined,
) {
  return useQuery({
    queryKey: ['swiss-income-statement', fromDate, toDate],
    queryFn: () =>
      swissReportsApi.incomeStatement(fromDate!, toDate!).then((r) => r.data),
    enabled: !!fromDate && !!toDate,
  });
}
