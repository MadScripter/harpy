import { Versions } from '@components/versions';

import { Links } from './links';

export function Footer() {
    return (
        <footer className="flex h-9 w-full flex-row items-center justify-between">
            <Versions />
            <Links />
        </footer>
    );
}
