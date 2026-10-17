import { FormEvent, useCallback, useEffect, useMemo, useState } from 'react';
import { api, Area, getStoredApiUrl, Project, Task, View } from './api';

const menus: { view: View; title: string; symbol: string }[] = [
  { view: 'inbox', title: 'Inbox', symbol: '↧' },
  { view: 'next', title: 'Next', symbol: '→' },
  { view: 'waiting', title: 'Waiting', symbol: '◷' },
  { view: 'scheduled', title: 'Scheduled', symbol: '▦' },
  { view: 'someday', title: 'Someday', symbol: '◇' },
  { view: 'completed', title: 'Logbook', symbol: '✓' },
  { view: 'all', title: 'Alle Aufgaben', symbol: '≡' },
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
      <div className="brand"><span className="brandmark">✳</span><span>TaskManager</span></div>
      <div className="navlabel">Fokus</div>
      <nav>{menus.map(menu => <button key={menu.view} className={`navitem ${view === menu.view && !areaId && !projectId ? 'active' : ''}`} onClick={() => changeView(menu.view)}><span className="navsymbol">{menu.symbol}</span>{menu.title}</button>)}</nav>
      <div className="navlabel spaced">Areas</div>
      <nav>{areas.map(area => <button key={area.id} className={`navitem ${areaId === area.id && !projectId ? 'active' : ''}`} onClick={() => selectArea(area.id)}><span className="navsymbol">▣</span>{area.name}</button>)}</nav>
      <form className="sideform" onSubmit={createArea}><input aria-label="Neue Area" value={newArea} onChange={e => setNewArea(e.target.value)} placeholder="+ Neue Area"/><button disabled={busy} title="Area anlegen">+</button></form>
      <div className="navlabel spaced">Projekte</div>
      <nav>{filteredProjects.map(project => <button key={project.id} className={`navitem ${projectId === project.id ? 'active' : ''}`} onClick={() => selectProject(project.id)}><span className="navsymbol">{project.project_type === 'sequential' ? '⇥' : '◫'}</span><span className="ellipsis">{project.name}</span></button>)}</nav>
      <form className="sideform projectform" onSubmit={createProject}><input aria-label="Neues Projekt" value={newProject} onChange={e => setNewProject(e.target.value)} placeholder="+ Neues Projekt"/><select aria-label="Projekttyp" value={projectType} onChange={e => setProjectType(e.target.value as Project['project_type'])}><option value="parallel">Parallel</option><option value="sequential">Sequential</option></select><button disabled={busy} title="Projekt anlegen">+</button></form>
      <button className="settings" onClick={() => setShowSettings(v => !v)}>⚙ Verbindung</button>
    </aside>
    <main className="main">
      <header className="topbar"><div><span className="eyebrow">GTD / {activeProject ? 'Projekt' : activeArea ? 'Area' : 'Ansicht'}</span><h1>{titleText}</h1></div><div className="topright"><span className="count">{tasks.length} {tasks.length === 1 ? 'Eintrag' : 'Einträge'}</span><button className="iconbutton" aria-label="Aktualisieren" onClick={() => void reload()}>↻</button></div></header>
      {activeProject && <div className="hint">Projektmodus: <strong>{activeProject.project_type === 'sequential' ? 'Sequential' : 'Parallel'}</strong>. Im Projekt sind alle offenen Schritte sichtbar; „Next“ zeigt nur freigegebene Aufgaben.</div>}
      {showSettings && <section className="settingspanel"><h3>Serververbindung</h3><p>Für die native App standardmäßig <code>http://127.0.0.1:8787</code>. Für einen anderen Rechner die Mac-IP mit Port 8787 eintragen. Keine Freigabe ins öffentliche Internet.</p><form onSubmit={e => {e.preventDefault(); localStorage.setItem('taskmanager_api_url', apiUrl.trim()); setShowSettings(false); void reload();}}><input aria-label="Server-URL" value={apiUrl} onChange={e => setApiUrl(e.target.value)} placeholder="Standard verwenden"/><button>Speichern</button></form></section>}
      {error && <div role="alert" className="error">Server nicht erreichbar oder Fehler: {error}. Bitte Backend starten und die Verbindung prüfen.</div>}
      <form className="quickadd" onSubmit={createTask}><span className="addsymbol">+</span><input aria-label="Neue Aufgabe" value={title} onChange={e => setTitle(e.target.value)} placeholder="Neue Aufgabe erfassen …"/><select aria-label="Aufgabenstatus" value={taskStatus} onChange={e => setTaskStatus(e.target.value as View)}><option value="inbox">Inbox</option><option value="next">Next</option><option value="waiting">Waiting</option><option value="scheduled">Scheduled</option><option value="someday">Someday</option></select><button disabled={busy || !title.trim()}>Hinzufügen</button></form>
      <div className="listhead"><span>AUFGABEN</span><span>STATUS</span></div>
      <section className="tasks">
        {tasks.length === 0 && !error && <div className="empty"><div className="emptyicon">✓</div><h2>Alles erledigt oder noch nichts erfasst</h2><p>Erstelle eine Aufgabe über das Eingabefeld oben.</p></div>}
        {tasks.map(task => <article className="task" key={task.id}>
          <button className={`checkbox ${task.status === 'completed' ? 'checked' : ''}`} aria-label={task.status === 'completed' ? 'Aufgabe wieder öffnen' : 'Aufgabe abschließen'} disabled={busy} onClick={() => void action(() => api.setTaskStatus(task.id, task.status === 'completed' ? 'next' : 'completed'))}>{task.status === 'completed' ? '✓' : ''}</button>
          <div className="taskbody"><div className={`tasktitle ${task.status === 'completed' ? 'done' : ''}`}>{task.title}</div><div className="taskmeta">{projects.find(p => p.id === task.project_id)?.name ?? (areas.find(a => a.id === task.area_id)?.name ?? 'Ohne Projekt')}{task.blocked_by_sequence ? ' · In Reihenfolge blockiert' : ''}</div></div>
          <select aria-label={`Status für ${task.title}`} className="taskstatus" value={task.status} disabled={busy} onChange={e => void action(() => api.setTaskStatus(task.id, e.target.value))}><option value="inbox">Inbox</option><option value="next">Next</option><option value="waiting">Waiting</option><option value="scheduled">Scheduled</option><option value="someday">Someday</option><option value="completed">Erledigt</option></select>
        </article>)}
      </section>
      <footer className="footer">Lokale Testversion · Rust / React / Tauri · Perspectives und MCP folgen</footer>
    </main>
  </div>;
}
