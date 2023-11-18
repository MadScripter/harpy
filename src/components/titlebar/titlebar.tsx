import { CloseButton, MinimizeButton } from './controlbox';

export function Titlebar() {
    return (
        <header className="relative flex h-12 w-full flex-row items-center bg-transparent">
            <h2 className="ml-2.5 w-auto select-none font-medium uppercase text-white">
                Harpy
            </h2>
            <div
                className="pointer-events-auto z-50 h-full flex-grow select-none"
                data-tauri-drag-region
            />
            <div className="flex h-full flex-row items-center justify-center space-x-2 p-2.5">
                <MinimizeButton />
                <CloseButton />
            </div>
        </header>
    );
}
