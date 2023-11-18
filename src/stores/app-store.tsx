import { createTrackedSelector } from 'react-tracked';
import { create } from 'zustand';

export enum GameStatus {
    Missing = 'missing',
    UpdateRequired = 'update_required',
    UpToDate = 'up_to_date',
    Unknown = 'unknown',
}

export type AppState = {
    status: GameStatus;
    requisitesInstalled: boolean;
    currentVersion: string;
    latestVersion: string;
};

type Actions = {
    update: (value: AppState) => void;
};

const initialState: AppState = {
    status: GameStatus.Unknown,
    requisitesInstalled: false,
    currentVersion: 'N/A',
    latestVersion: 'N/A',
};

const store = create<AppState & Actions>()((set) => ({
    ...initialState,
    update: (value: AppState) => set(() => value),
}));

export const useAppStore = createTrackedSelector(store);
