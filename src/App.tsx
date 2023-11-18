import 'ldrs/squircle';

import {
    DownloadStatus,
    GameStatus,
    useAppStore,
    useDownloadStore,
} from '@stores';

import { InstallButton, PlayButton, UpdateButton } from '@components/buttons';
import { Download } from '@components/download';
import { Footer } from '@components/footer';
import { NewsCarousel } from '@components/news';
import { Titlebar } from '@components/titlebar';

function App() {
    const appStore = useAppStore();
    const downloadStore = useDownloadStore();

    const isIdle = downloadStore.status === DownloadStatus.Idle;
    const isDownloading = downloadStore.status === DownloadStatus.Downloading;
    const isLoading = appStore.status === GameStatus.Unknown && !isDownloading;

    const actionButton = () => {
        switch (appStore.status) {
            case GameStatus.Missing:
                return <InstallButton />;
            case GameStatus.UpdateRequired:
                return <UpdateButton />;
            case GameStatus.UpToDate:
                return <PlayButton />;
            case GameStatus.Unknown:
                return null;
            default:
                return null;
        }
    };

    return (
        <div
            id="app-background"
            className="relative flex h-full w-full flex-col bg-primary"
        >
            <div className="grid h-full w-full grid-rows-3 gap-0">
                <div className="row-span-12 relative h-full">
                    <Titlebar />
                </div>

                <div className="row-span-12 relative h-full w-full">
                    <div className="absolute right-0 h-full w-1/2">
                        <NewsCarousel />
                    </div>
                </div>

                <div className="row-span-12 relative h-full">
                    {isLoading && (
                        <div className="absolute left-1/2 top-1/2 m-auto -translate-x-1/2 -translate-y-1/2">
                            <l-squircle
                                size="37"
                                stroke="5"
                                stroke-length="0.15"
                                bg-opacity="0.1"
                                speed="0.9"
                                color="#FF8A65"
                            />
                        </div>
                    )}
                    {isIdle && actionButton()}
                    <div className="absolute bottom-0 w-full px-4">
                        {isDownloading && <Download />}
                        <Footer />
                    </div>
                </div>
            </div>
        </div>
    );
}

export default App;
