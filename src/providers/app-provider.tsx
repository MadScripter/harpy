import { DownloadProgress } from '@models';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import { createContext, useEffect } from 'react';

import { AppState, useAppStore, useDownloadStore } from '@stores';

const AppContext = createContext(null);

export function AppProvider({ children }: { children: React.ReactNode }) {
    const appStore = useAppStore();
    const downloadStore = useDownloadStore();

    useEffect(() => {
        const unlistenStart = listen('download:start', () =>
            downloadStore.init(),
        );

        const unlistenProgress = listen<DownloadProgress>(
            'download:progress',
            (event) => downloadStore.update(event.payload),
        );

        const unlistenDone = listen('download:done', () =>
            downloadStore.reset(),
        );

        invoke<AppState>('init').then(appStore.update).catch(console.error);

        return () => {
            unlistenStart.then((f) => f());
            unlistenProgress.then((f) => f());
            unlistenDone.then((f) => f());
        };
    }, []);

    return <AppContext.Provider value={null}>{children}</AppContext.Provider>;
}
