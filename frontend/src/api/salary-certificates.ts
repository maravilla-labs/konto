import client from './client';
import type { SalaryCertificateData } from '@/types/salary-certificate';

export const salaryCertificatesApi = {
  list: (year: number) => client.get<SalaryCertificateData[]>(`/salary-certificates/${year}`),
  downloadPdf: (year: number, employeeId: string) =>
    client.get(`/salary-certificates/${year}/${employeeId}/pdf`, { responseType: 'blob' }),
  downloadZip: (year: number) =>
    client.get(`/salary-certificates/${year}/zip`, { responseType: 'blob' }),
};
