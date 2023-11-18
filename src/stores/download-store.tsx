import { DownloadProgress } from '@models';
import { createTrackedSelector } from 'react-tracked';
import { create } from 'zustand';

export enum DownloadStatus {
    Idle,
    Init,
    Downloading,
    Cancelled,
}

type State = {
    status: DownloadStatus;
    speed: string;
    speed_limit: bigint;
    current: string;
    total: string;
    progress: number;
    eta: string;
};

type Actions = {
    init: () => void;
    update: (value: DownloadProgress) => void;
    reset: () => void;

    IsIdle: () => boolean;
    isDownloading: () => boolean;
    isInitializing: () => boolean;
    isCancelled: () => boolean;

    fromStatus: () => string;
};

const initialState: State = {
    status: DownloadStatus.Idle,
    speed: '0 B',
    speed_limit: BigInt(0),
    current: '0 B',
    total: '0 B',
    progress: 0,
    eta: '0',
};

const store = create<State & Actions>()((set, get) => ({
    ...initialState,

    init: () => set(() => ({ status: DownloadStatus.Init })),
    update: (value: DownloadProgress) =>
        set(() => ({
            status: DownloadStatus.Downloading,
            current: value.current,
            total: value.total,
            progress: value.percentage,
            speed: value.speed,
            eta: value.eta,
        })),
    reset: () => set(() => initialState),

    IsIdle: () => get().status === DownloadStatus.Idle,
    isDownloading: () => get().status === DownloadStatus.Downloading,
    isInitializing: () => get().status === DownloadStatus.Init,
    isCancelled: () => get().status === DownloadStatus.Cancelled,

    fromStatus: () => {
        const { status } = get();

        switch (status) {
            case DownloadStatus.Init:
                return 'initializing';
            case DownloadStatus.Downloading:
                return 'downloading';
            default:
                return 'idle';
        }
    },
}));

export const useDownloadStore = createTrackedSelector(store);
