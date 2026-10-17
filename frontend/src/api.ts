export type View = 'inbox' | 'next' | 'waiting' | 'scheduled' | 'someday' | 'completed' | 'all';
export type Area = { id: string; name: string; sort_order: number };
export type Project = { id: string; area_id: string | null; name: string; notes: string; project_type: 'sequential' | 'parallel'; status: string; sort_order: number };
export type Task = { id: string; project_id: string | null; area_id: string | null; title: string; notes: string; status: string; available: boolean; blocked_by_sequence: boolean; sort_order: number };

export const getStoredApiUrl = () => localStorage.getItem('taskmanager_api_url') ?? '';
export function getApiBase() {
  const override = getStoredApiUrl().trim().replace(/\/$/, '');
  if (override) return override;
  const isTauri = window.location.protocol === 'tauri:' || window.location.hostname === 'tauri.localhost';
  return isTauri ? 'http://127.0.0.1:8787' : '';
}
export async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const response = await fetch(`${getApiBase()}/api${path}`, {
    ...options,
    headers: { 'Content-Type': 'application/json', ...options?.headers },
  });
  if (!response.ok) {
    let message = `HTTP ${response.status}`;
    try { const body = await response.json() as { error?: string }; message = body.error || message; } catch { /* ignore */ }
    throw new Error(message);
  }
  return await response.json() as T;
}
export const api = {
  areas: () => request<Area[]>('/areas'),
  projects: () => request<Project[]>('/projects'),
  tasks: (view: View, areaId?: string, projectId?: string) => {
    const p = new URLSearchParams({ view });
    if (areaId) p.set('area_id', areaId);
    if (projectId) p.set('project_id', projectId);
    return request<Task[]>(`/tasks?${p.toString()}`);
  },
  addArea: (name: string) => request<Area>('/areas', { method: 'POST', body: JSON.stringify({ name }) }),
  addProject: (name: string, area_id: string | null, project_type: Project['project_type']) =>
    request<Project>('/projects', { method: 'POST', body: JSON.stringify({ name, area_id, project_type }) }),
  addTask: (title: string, status: View, project_id: string | null, area_id: string | null) =>
    request<Task>('/tasks', { method: 'POST', body: JSON.stringify({ title, status, project_id, area_id }) }),
  setTaskStatus: (id: string, status: string) => request<Task>(`/tasks/${id}`, { method: 'PATCH', body: JSON.stringify({ status }) }),
};
