import React from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';
import './styles.css';
import { isDesktop } from './api';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

if (isDesktop()) {
  void listen('nextleaf-data-changed', () => window.dispatchEvent(new Event('nextleaf-data-changed')));
  void invoke<{ remote_url: string | null }>('desktop_status').then(status => {
    // Only a connection saved in this database may sync automatically.
    const remote = status.remote_url;
    if (remote) return invoke('desktop_sync', { remote });
  }).catch(() => { /* Offline startup remains usable; sync status explains the failure. */ });
}

createRoot(document.getElementById('root')!).render(<React.StrictMode><App /></React.StrictMode>);
