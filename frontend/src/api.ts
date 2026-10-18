import { invoke, isTauri } from '@tauri-apps/api/core';
import type { ProjectChanges } from './projectEdit';
import type { TaskChanges } from './taskEdit';
export type View = 'inbox' | 'next' | 'waiting' | 'scheduled' | 'someday' | 'completed' | 'later' | 'focus' | 'trash' | 'all';
export type Tag = { id: string; name: string; kind: 'tag' | 'person'; archived: boolean };
export type Area = { id: string; name: string; sort_order: number; archived: boolean };
type TagChanges = Partial<Pick<Tag, 'name' | 'kind' | 'archived'>>;
type AreaChanges = Partial<Pick<Area, 'name' | 'archived'>>;
const normalizeTag = (tag: Tag): Tag => ({ ...tag, kind: tag.kind ?? 'tag', archived: tag.archived ?? false });
const normalizeArea = (area: Area): Area => ({ ...area, archived: area.archived ?? false });
export type Project = { id: string; area_id: string | null; name: string; notes: string; project_type: 'sequential' | 'parallel'; completed_at: string | null; status: string; sort_order: number; due_date: string | null; starred: boolean; tags: string[] };
export type Task = { id: string; project_id: string | null; area_id: string | null; title: string; notes: string; status: string; available: boolean; blocked_by_sequence: boolean; sort_order: number; due_date: string | null; defer_until: string | null; completed_at: string | null; starred: boolean; tags: string[]; waiting_for: string | null; estimated_minutes: number | null; energy: number | null };

export const isDesktop = () => isTauri();
export const getStoredApiUrl = () => localStorage.getItem('taskmanager_api_url') ?? '';
export function getApiBase() {
  if (isDesktop()) return 'Local database';
  const override = getStoredApiUrl().trim().replace(/\/$/, '');
  if (override) return override;
  const isTauri = window.location.protocol === 'tauri:' || window.location.hostname === 'tauri.localhost';
  return isTauri ? 'http://127.0.0.1:8787' : '';
}
export async function request<T>(path: string, options?: RequestInit): Promise<T> {
  if (isDesktop()) {
    const response = await invoke<{ status: number; body: T & { error?: string } }>('desktop_api', { path, method: options?.method ?? 'GET', body: options?.body ? String(options.body) : null });
    if (response.status >= 400) throw new Error(response.body.error || `HTTP ${response.status}`);
    return response.body;
  }
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
  health: () => request<{ capabilities?: string[] }>('/health'),
  tags: async () => (await request<Tag[]>('/tags')).map(normalizeTag),
  addTag: async (name: string, kind?: Tag['kind']) => {
    const health = await api.health();
    if (!health.capabilities?.includes(kind === undefined ? 'item-creation' : 'tag-management')) throw new Error('Update the Nextleaf server before creating tags or people.');
    return normalizeTag(await request<Tag>('/tags', { method: 'POST', body: JSON.stringify({ name, kind }) }));
  },
  updateTag: async (id: string, changes: TagChanges, expected?: TagChanges) => {
    if (!(await api.health()).capabilities?.includes('tag-management')) throw new Error('Update the Nextleaf server before managing tags or people. Your changes have not been saved.');
    return normalizeTag(await request<Tag>(`/tags/${id}`, { method: 'PATCH', body: JSON.stringify(expected ? { ...changes, expected } : changes) }));
  },
  areas: async () => (await request<Area[]>('/areas')).map(normalizeArea),
  updateArea: async (id: string, changes: AreaChanges, expected?: AreaChanges) => {
    if (!(await api.health()).capabilities?.includes('tag-management')) throw new Error('Update the Nextleaf server before managing areas. Your changes have not been saved.');
    return normalizeArea(await request<Area>(`/areas/${id}`, { method: 'PATCH', body: JSON.stringify(expected ? { ...changes, expected } : changes) }));
  },
  projects: async () => (await request<Project[]>('/projects')).map(project => ({ ...project, due_date: project.due_date ?? null, starred: project.starred ?? false, tags: project.tags ?? [] })),
  updateProject: async (id: string, changes: ProjectChanges, expected?: ProjectChanges) => {
    const health = await api.health();
    if (!health.capabilities?.includes('project-editing')) throw new Error('Update the Nextleaf server before editing projects. Your changes have not been saved.');
    if (expected && !health.capabilities?.includes('conditional-updates')) throw new Error('Update the Nextleaf server before saving. Your changes have not been saved.');
    return request<Project>(`/projects/${id}`, { method: 'PATCH', body: JSON.stringify(expected ? { ...changes, expected } : changes) });
  },
  tasks: (view: View, areaId?: string, projectId?: string) => {
    const p = new URLSearchParams({ view });
    if (areaId) p.set('area_id', areaId);
    if (projectId) p.set('project_id', projectId);
    return request<Task[]>(`/tasks?${p.toString()}`);
  },
  addArea: async (name: string) => normalizeArea(await request<Area>('/areas', { method: 'POST', body: JSON.stringify({ name }) })),
  addProject: (name: string, area_id: string | null, project_type: Project['project_type']) =>
    request<Project>('/projects', { method: 'POST', body: JSON.stringify({ name, area_id, project_type }) }),
  addTask: async (title: string, status: View, project_id: string | null, area_id: string | null, starred = false) => {
    if (starred && !(await api.health()).capabilities?.includes('item-creation')) throw new Error('Update the Nextleaf server before creating Focus tasks.');
    return request<Task>('/tasks', { method: 'POST', body: JSON.stringify({ title, status, project_id, area_id, starred }) });
  },
  updateTask: async (id: string, changes: TaskChanges, expected?: TaskChanges) => {
    // Older servers ignore unknown fields; never silently drop part of an edit.
    if (expected || Object.keys(changes).some(field => field !== 'title' && field !== 'status')) {
      const health = await request<{ capabilities?: string[] }>('/health');
      if (!health.capabilities?.includes('task-editing')) throw new Error('Update the Nextleaf server before editing task details. Your changes have not been saved.');
      if (expected && !health.capabilities?.includes('conditional-updates')) throw new Error('Update the Nextleaf server before saving. Your changes have not been saved.');
    }
    return request<Task>(`/tasks/${id}`, { method: 'PATCH', body: JSON.stringify(expected ? { ...changes, expected } : changes) });
  },
};
