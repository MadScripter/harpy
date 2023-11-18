import { GameVersion } from './game';
import { LauncherVersion } from './launcher';

export function Versions() {
    return (
        <div className="flex flex-row items-center space-x-2">
            <LauncherVersion />
            <div className="h-4 w-px bg-dimmed/40" />
            <GameVersion />
        </div>
    );
}
