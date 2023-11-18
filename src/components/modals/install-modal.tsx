import { open as openDialog } from '@tauri-apps/api/dialog';
import { invoke } from '@tauri-apps/api/tauri';
import { FolderIcon } from 'lucide-react';
import { useState } from 'react';
import { useModalInstance } from 'react-modal-state';

import { useAppStore } from '@stores';

import { Modal } from './modal';

export function InstallModal() {
    const { isOpen, close } = useModalInstance();
    const { requisitesInstalled } = useAppStore();

    const [path, setPath] = useState<string>('');
    const [availableSpace, setAvailableSpace] = useState<string>('N/A');

    const onSelect = async () => {
        const selected = await openDialog({
            directory: true,
            multiple: false,
        });

        setPath(selected as string);

        invoke<string>('query_disk_space', { path: selected })
            .then(setAvailableSpace)
            .catch(console.error);
    };

    const onInstall = () => {
        invoke('install', { path, withRequisites: !requisitesInstalled })
            .then(console.log)
            .catch(console.error);
    };

    return (
        <Modal title="Choose Install Location" isOpen={isOpen} onClose={close}>
            <div>
                <label htmlFor="path" className="text-sm text-secondary">
                    Select a folder
                </label>
                <div className="relative mt-1">
                    <input
                        id="path"
                        name="path"
                        className="h-9 w-96 rounded-lg bg-secondary px-2.5 text-xs font-semibold tracking-wide text-primary"
                        type="text"
                        value={path}
                        disabled
                        onChange={(e) => setPath(e.target.value)}
                    />
                    <button
                        type="button"
                        className="absolute right-0.5 top-1/2 flex h-8 w-8 -translate-y-1/2 items-center justify-center rounded-full bg-primary"
                        onClick={onSelect}
                    >
                        <FolderIcon
                            className="fill-secondary stroke-transparent"
                            size={16}
                        />
                    </button>
                </div>
            </div>

            <div className="mt-2 flex w-full flex-col gap-1">
                <div className="flex flex-row items-center gap-1.5">
                    <div className="h-2 w-2 rounded-full bg-secondary" />
                    <div className="text-xs uppercase text-secondary">
                        Space required
                    </div>
                    <div className="text-xs text-accent">0 GB</div>
                </div>

                <div className="flex flex-row items-center gap-1.5">
                    <div className="h-2 w-2 rounded-full bg-green-500" />
                    <div className="text-xs uppercase text-secondary">
                        Space available
                    </div>
                    <div className="text-xs text-accent">{availableSpace}</div>
                </div>
            </div>

            <div className="mt-10 flex w-full flex-row justify-end gap-2.5">
                <button
                    type="button"
                    className="h-9 rounded-lg bg-secondary px-8 text-sm font-bold text-primary"
                    onClick={close}
                >
                    Cancel
                </button>
                <button
                    type="button"
                    className="h-9 rounded-lg bg-accent px-8 text-sm font-bold text-primary"
                    onClick={onInstall}
                >
                    Install
                </button>
            </div>
        </Modal>
    );
}
