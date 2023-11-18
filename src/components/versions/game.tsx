import { useAppStore } from '@stores';

import VersionWrapper from './version-wrapper';

export function GameVersion() {
    const { currentVersion } = useAppStore();

    return <VersionWrapper title="Game" version={currentVersion} />;
}
