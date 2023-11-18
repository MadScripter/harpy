import { appWindow } from '@tauri-apps/api/window';
import { XIcon } from 'lucide-react';

import { ControlBoxButton } from './controlbox-button';

export function CloseButton() {
    return (
        <ControlBoxButton onClick={() => appWindow.close()}>
            <XIcon className="fill-primary" size={20} />
        </ControlBoxButton>
    );
}
