import React from 'react';
import ReactDOM from 'react-dom/client';
import { ModalProvider, ModalRenderer } from 'react-modal-state';

import { AppProvider } from '@providers';

import { InstallModal } from '@components/modals';

import App from './App';
import './styles.css';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <AppProvider>
            <ModalProvider modals={[['install', InstallModal]]}>
                <App />
                <ModalRenderer Component={InstallModal} />
            </ModalProvider>
        </AppProvider>
    </React.StrictMode>,
);
