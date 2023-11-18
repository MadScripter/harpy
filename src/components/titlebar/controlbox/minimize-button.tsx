import { appWindow } from '@tauri-apps/api/window';
import { MinusIcon } from 'lucide-react';

import { ControlBoxButton } from './controlbox-button';

export function MinimizeButton() {
    return (
        <ControlBoxButton onClick={() => appWindow.minimize()}>
            <MinusIcon className="fill-primary" size={20} />
        </ControlBoxButton>
    );
}
