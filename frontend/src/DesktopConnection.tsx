import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { api, getStoredApiUrl } from './api';
import './DesktopConnection.css';
import Dialog from './Dialog';

type Conflict = { entity: string; id: string; field: string; local: unknown; server: unknown };
type SyncStatus = { initialized: boolean; pending: boolean; conflicts: Conflict[]; error: string | null; last_sync: string | null; remote_url: string | null };
type DesktopInfo = { data_directory: string; mcp_command: string; mcp_args: string[] };
export default function DesktopConnection({ onChanged }: { onChanged: () => void }) {
  const [status, setStatus] = useState<SyncStatus | null>(null);
  const [info, setInfo] = useState<DesktopInfo | null>(null);
  const [remote, setRemote] = useState('');
  const [busy, setBusy] = useState(false);
  const [resolution, setResolution] = useState<'local' | 'server' | null>(null);
  const [error, setError] = useState('');
  const [names, setNames] = useState<Record<string, string>>({});
  const conflictRecords = [...new Set(status?.conflicts.map(c => `${c.entity}:${c.id}`) ?? [])].sort().join(',');
  useEffect(() => {
    let active = true;
    if (!conflictRecords) { setNames({}); return; }
    void Promise.all([api.tasks('all'), api.tasks('completed'), api.tasks('trash'), api.projects(), api.areas()])
      .then(([open, completed, trash, projects, areas]) => {
        if (active) setNames(Object.fromEntries([
          ...[...open, ...completed, ...trash].map(task => [`task:${task.id}`, task.title]),
          ...projects.map(project => [`project:${project.id}`, project.name]),
          ...areas.map(area => [`area:${area.id}`, area.name]),
        ]));
      }).catch(() => {});
    return () => { active = false; };
  }, [conflictRecords]);
  useEffect(() => {
    let active = true;
    void Promise.all([invoke<SyncStatus>('desktop_status'), invoke<DesktopInfo>('desktop_info')]).then(([next, details]) => {
      if (active) { setStatus(next); setRemote(next.remote_url ?? getStoredApiUrl()); setInfo(details); }
    }).catch(e => { if (active) setError(String(e)); });
    const timer = window.setInterval(() => {
      void invoke<SyncStatus>('desktop_status').then(next => { if (active) { setStatus(next); if (!next.error) setError(''); } }).catch(() => {});
    }, 5000);
    return () => { active = false; clearInterval(timer); };
  }, []);
  async function run(command: string, args: Record<string, unknown>) {
    setBusy(true); setError('');
    try { setStatus(await invoke<SyncStatus>(command, args)); onChanged(); }
    catch (e) { setError(String(e)); setStatus(await invoke<SyncStatus>('desktop_status').catch(() => status)); }
    finally { setBusy(false); }
  }
  return <section className="desktopconnection" aria-label="Local data and sync">
    <h3>Local data and sync</h3>
    <p>Your tasks are saved on this computer. Connect to your server to sync changes.</p>
    <form onSubmit={event => { event.preventDefault(); void run('desktop_sync', { remote: remote.trim() }); }}>
      <input aria-label="Sync server URL" type="url" placeholder="https://your-server" value={remote} disabled={busy || status?.initialized} onChange={event => setRemote(event.target.value)}/>
      <button disabled={busy || !remote.trim()}>{busy ? 'Syncing…' : status?.initialized ? 'Sync now' : 'Connect and sync'}</button>
    </form>
    <p role="status">{status?.conflicts.length ? 'Changes need your attention.' : status?.error ? 'Server unavailable. You can keep working locally.' : status?.pending ? 'Local changes are waiting to sync.' : status?.initialized ? 'Changes are synced.' : 'Working locally. Server sync is not connected.'}</p>
    {status?.last_sync && <p>Last sync: {new Date(status.last_sync).toLocaleString()}</p>}
    {(error || status?.error) && <p className="connectionerror" role="alert">{error || status?.error}</p>}
    {!!status?.conflicts.length && <div className="syncconflicts">
      <p>Both this computer and the server changed the same data. Review the values before choosing which version to keep for these conflicts.</p>
      {status.conflicts.map((conflict, index) => <details key={index}><summary>{names[`${conflict.entity}:${conflict.id}`] ?? conflict.id} — {conflict.field}</summary><dl><dt>On this computer</dt><dd>{JSON.stringify(conflict.local)}</dd><dt>On the server</dt><dd>{JSON.stringify(conflict.server)}</dd></dl></details>)}
      <button disabled={busy} onClick={() => setResolution('local')}>Keep local values</button>
      <button disabled={busy} onClick={() => setResolution('server')}>Keep server values</button>
    </div>}
    {resolution && <Dialog title="Resolve sync conflicts" onClose={() => { if (!busy) setResolution(null); }}>
      <p>{resolution === 'local' ? 'Keep this computer’s values for all listed conflicts? The conflicting server values will be replaced on sync.' : 'Keep the server values for all listed conflicts? The conflicting values on this computer will be replaced.'}</p>
      <button disabled={busy} onClick={() => setResolution(null)}>Cancel</button>
      <button disabled={busy} onClick={() => { void run('desktop_resolve', { choice: resolution }).finally(() => setResolution(null)); }}>{busy ? 'Saving…' : 'Confirm'}</button>
    </Dialog>}
    {info && <details><summary>Storage and local MCP</summary>
      <p>Database folder: <code>{info.data_directory}</code></p>
      <p>For an agent that supports local MCP, use this command with the argument <code>--mcp</code>. It uses the same local database.</p>
      <pre>{JSON.stringify({ mcpServers: { nextleaf: { command: info.mcp_command, args: info.mcp_args } } }, null, 2)}</pre>
    </details>}
  </section>;
}

export function DesktopSyncIndicator({ onOpen }: { onOpen: () => void }) {
  const [status, setStatus] = useState<SyncStatus | null>(null);
  useEffect(() => {
    let active = true;
    const read = () => { void invoke<SyncStatus>('desktop_status').then(value => { if (active) setStatus(value); }).catch(() => {}); };
    read();
    const timer = window.setInterval(read, 5000);
    window.addEventListener('nextleaf-data-changed', read);
    return () => { active = false; clearInterval(timer); window.removeEventListener('nextleaf-data-changed', read); };
  }, []);
  const text = status?.conflicts.length ? 'Sync conflicts' : status?.error ? 'Offline' : status?.pending ? 'Sync pending' : status?.initialized ? 'Synced' : 'Local only';
  return <button className="syncindicator" onClick={onOpen} aria-label={`Sync status: ${text}. Open settings.`}>{text}</button>;
}
