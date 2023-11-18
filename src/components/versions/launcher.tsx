import { getVersion } from '@tauri-apps/api/app';
import { useEffect, useState } from 'react';

import VersionWrapper from './version-wrapper';

export function LauncherVersion() {
    const [version, setVersion] = useState<string>('N/A');

    useEffect(() => {
        getVersion().then(setVersion);
    }, []);

    return <VersionWrapper title="Launcher" version={version} />;
}
