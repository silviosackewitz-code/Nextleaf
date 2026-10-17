import { FormEvent, useCallback, useEffect, useMemo, useState } from 'react';
import { api, Area, getStoredApiUrl, Project, Task, View } from './api';

const menus: { view: View; title: string; symbol: string }[] = [
  { view: 'inbox', title: 'Inbox', symbol: '↧' },
  { view: 'next', title: 'Next', symbol: '→' },
  { view: 'waiting', title: 'Waiting', symbol: '◷' },
  { view: 'scheduled', title: 'Scheduled', symbol: '▦' },
  { view: 'someday', title: 'Someday', symbol: '◇' },
  { view: 'completed', title: 'Logbook', symbol: '✓' },
  { view: 'all', title: 'All tasks', symbol: '≡' },
];

export default function App() {
  const [view, setView] = useState<View>('next');
  const [areaId, setAreaId] = useState<string>('');
  const [projectId, setProjectId] = useState<string>('');
  const [areas, setAreas] = useState<Area[]>([]);
  const [projects, setProjects] = useState<Project[]>([]);
  const [tasks, setTasks] = useState<Task[]>([]);
  const [title, setTitle] = useState('');
  const [taskStatus, setTaskStatus] = useState<View>('inbox');
  const [newArea, setNewArea] = useState('');
  const [newProject, setNewProject] = useState('');
  const [projectType, setProjectType] = useState<Project['project_type']>('parallel');
  const [apiUrl, setApiUrl] = useState(getStoredApiUrl());
  const [showSettings, setShowSettings] = useState(false);
  const [error, setError] = useState<string>('');
  const [busy, setBusy] = useState(false);

  const reload = useCallback(async () => {
    try {
      const [a, p, t] = await Promise.all([
        api.areas(), api.projects(), api.tasks(view, areaId || undefined, projectId || undefined),
      ]);
      setAreas(a); setProjects(p); setTasks(t); setError('');
    } catch (e) { setError(e instanceof Error ? e.message : String(e)); }
  }, [view, areaId, projectId]);
  useEffect(() => { void reload(); }, [reload]);
  const filteredProjects = useMemo(() => projects.filter(p => !areaId || p.area_id === areaId), [projects, areaId]);
  const activeProject = projects.find(p => p.id === projectId);
  const activeArea = areas.find(a => a.id === areaId);
  const titleText = activeProject?.name ?? activeArea?.name ?? menus.find(m => m.view === view)?.title;

  async function action(callback: () => Promise<unknown>) {
    setBusy(true);
    try { await callback(); await reload(); setError(''); }
    catch (e) { setError(e instanceof Error ? e.message : String(e)); }
    finally { setBusy(false); }
  }
  function changeView(next: View) { setView(next); setAreaId(''); setProjectId(''); }
  function selectArea(id: string) { setAreaId(id); setProjectId(''); setView('all'); }
  function selectProject(id: string) { setProjectId(id); setView('all'); }
  function createTask(event: FormEvent) {
    event.preventDefault();
    if (!title.trim()) return;
    const value = title.trim();
    void action(async () => { await api.addTask(value, taskStatus, projectId || null, !projectId && areaId ? areaId : null); setTitle(''); });
  }
  function createArea(event: FormEvent) {
    event.preventDefault();
    if (!newArea.trim()) return;
    const value = newArea.trim();
    void action(async () => { await api.addArea(value); setNewArea(''); });
  }
  function createProject(event: FormEvent) {
    event.preventDefault();
    if (!newProject.trim()) return;
    const value = newProject.trim();
    void action(async () => { await api.addProject(value, areaId || null, projectType); setNewProject(''); });
  }
  return <div className="app">
    <aside className="sidebar">
      <div className="brand"><span className="brandmark">✳</span><span>Nextleaf</span></div>
      <div className="navlabel">Focus</div>
      <nav>{menus.map(menu => <button key={menu.view} className={`navitem ${view === menu.view && !areaId && !projectId ? 'active' : ''}`} onClick={() => changeView(menu.view)}><span className="navsymbol">{menu.symbol}</span>{menu.title}</button>)}</nav>
      <div className="navlabel spaced">Areas</div>
      <nav>{areas.map(area => <button key={area.id} className={`navitem ${areaId === area.id && !projectId ? 'active' : ''}`} onClick={() => selectArea(area.id)}><span className="navsymbol">▣</span>{area.name}</button>)}</nav>
      <form className="sideform" onSubmit={createArea}><input aria-label="New area" value={newArea} onChange={e => setNewArea(e.target.value)} placeholder="+ New area"/><button disabled={busy} title="Create area">+</button></form>
      <div className="navlabel spaced">Projects</div>
      <nav>{filteredProjects.map(project => <button key={project.id} className={`navitem ${projectId === project.id ? 'active' : ''}`} onClick={() => selectProject(project.id)}><span className="navsymbol">{project.project_type === 'sequential' ? '⇥' : '◫'}</span><span className="ellipsis">{project.name}</span></button>)}</nav>
      <form className="sideform projectform" onSubmit={createProject}><input aria-label="New project" value={newProject} onChange={e => setNewProject(e.target.value)} placeholder="+ New project"/><select aria-label="Project type" value={projectType} onChange={e => setProjectType(e.target.value as Project['project_type'])}><option value="parallel">Parallel</option><option value="sequential">Sequential</option></select><button disabled={busy} title="Create project">+</button></form>
      <button className="settings" onClick={() => setShowSettings(v => !v)}>⚙ Connection</button>
    </aside>
    <main className="main">
      <header className="topbar"><div><span className="eyebrow">GTD / {activeProject ? 'Project' : activeArea ? 'Area' : 'View'}</span><h1>{titleText}</h1></div><div className="topright"><span className="count">{tasks.length} {tasks.length === 1 ? 'task' : 'tasks'}</span><button className="iconbutton" aria-label="Refresh" onClick={() => void reload()}>↻</button></div></header>
      {activeProject && <div className="hint">Project mode: <strong>{activeProject.project_type === 'sequential' ? 'Sequential' : 'Parallel'}</strong>. All unfinished tasks are visible in the project; Next shows only available tasks.</div>}
      {showSettings && <section className="settingspanel"><h3>Server connection</h3><p>The desktop app uses <code>http://127.0.0.1:8787</code> by default. To connect to another computer, enter its IP address and port 8787. Do not expose the server to the public internet.</p><form onSubmit={e => {e.preventDefault(); localStorage.setItem('taskmanager_api_url', apiUrl.trim()); setShowSettings(false); void reload();}}><input aria-label="Server URL" value={apiUrl} onChange={e => setApiUrl(e.target.value)} placeholder="Use default"/><button>Save</button></form></section>}
      {error && <div role="alert" className="error">Server unavailable or error: {error}. Start the backend and check your connection.</div>}
      <form className="quickadd" onSubmit={createTask}><span className="addsymbol">+</span><input aria-label="New task" value={title} onChange={e => setTitle(e.target.value)} placeholder="Capture a new task …"/><select aria-label="Task status" value={taskStatus} onChange={e => setTaskStatus(e.target.value as View)}><option value="inbox">Inbox</option><option value="next">Next</option><option value="waiting">Waiting</option><option value="scheduled">Scheduled</option><option value="someday">Someday</option></select><button disabled={busy || !title.trim()}>Add</button></form>
      <div className="listhead"><span>TASKS</span><span>STATUS</span></div>
      <section className="tasks">
        {tasks.length === 0 && !error && <div className="empty"><div className="emptyicon">✓</div><h2>All done or nothing captured yet</h2><p>Add a task using the field above.</p></div>}
        {tasks.map(task => <article className="task" key={task.id}>
          <button className={`checkbox ${task.status === 'completed' ? 'checked' : ''}`} aria-label={task.status === 'completed' ? 'Reopen task' : 'Complete task'} disabled={busy} onClick={() => void action(() => api.setTaskStatus(task.id, task.status === 'completed' ? 'next' : 'completed'))}>{task.status === 'completed' ? '✓' : ''}</button>
          <div className="taskbody"><div className={`tasktitle ${task.status === 'completed' ? 'done' : ''}`}>{task.title}</div><div className="taskmeta">{projects.find(p => p.id === task.project_id)?.name ?? (areas.find(a => a.id === task.area_id)?.name ?? 'No project')}{task.blocked_by_sequence ? ' · Blocked by sequence' : ''}</div></div>
          <select aria-label={`Status for ${task.title}`} className="taskstatus" value={task.status} disabled={busy} onChange={e => void action(() => api.setTaskStatus(task.id, e.target.value))}><option value="inbox">Inbox</option><option value="next">Next</option><option value="waiting">Waiting</option><option value="scheduled">Scheduled</option><option value="someday">Someday</option><option value="completed">Completed</option></select>
        </article>)}
      </section>
      <footer className="footer">Local preview · Rust / React / Tauri · Perspectives and MCP coming later</footer>
    </main>
  </div>;
}
