import React from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';
import './styles.css';
import { isDesktop } from './api';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

async function start() {
  const root = createRoot(document.getElementById('root')!);
  if (isDesktop()) {
    let error: string | null;
    try { error = await invoke<string | null>('desktop_startup_error'); }
    catch (e) { error = String(e); }
    if (error) {
      root.render(<main role="alert" style={{ padding: 32 }}><h1>Nextleaf could not start</h1><pre style={{ whiteSpace: 'pre-wrap' }}>{error}</pre></main>);
      return;
    }
    void listen('nextleaf-data-changed', () => window.dispatchEvent(new Event('nextleaf-data-changed')));
    void invoke<{ remote_url: string | null; suspended: boolean }>('desktop_status').then(status => {
      const remote = status.remote_url;
      if (remote && !status.suspended) return invoke('desktop_sync', { remote });
    }).catch(() => { /* Offline startup remains usable; sync status explains the failure. */ });
  }
  root.render(<React.StrictMode><App /></React.StrictMode>);
}
void start();
