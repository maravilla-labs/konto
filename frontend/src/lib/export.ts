export function downloadCsv(
  endpoint: string,
  params?: Record<string, string>,
) {
  const searchParams = new URLSearchParams({ ...params, format: 'csv' });
  const stored = localStorage.getItem('konto_tokens');
  const token = stored ? JSON.parse(stored).access_token : '';

  fetch(`/api/v1${endpoint}?${searchParams}`, {
    headers: { Authorization: `Bearer ${token}` },
  })
    .then((res) => res.blob())
    .then((blob) => {
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `${endpoint.replace(/\//g, '-').replace(/^-/, '')}.csv`;
      a.click();
      URL.revokeObjectURL(url);
    });
}
