import { motion } from 'framer-motion';
import { GaugeIcon, PauseIcon } from 'lucide-react';

import { useDownloadStore } from '@stores';

import { ProgressBar } from './download/progress-bar';
import { DownloadStat } from './download/stat';

export function Download() {
    const store = useDownloadStore();

    return (
        <div className="mb-6" key="download">
            <div className="mb-1.5 flex w-full flex-row items-center justify-between">
                <div className="flex select-none flex-row items-center space-x-1.5">
                    <span className="text-sm font-medium uppercase tracking-wide text-white">
                        {store.fromStatus()}
                    </span>
                    <span className="text-sm font-semibold text-accent">{`${store.progress}%`}</span>
                </div>
                <div className="flex select-none flex-row items-center space-x-1.5">
                    <span className="text-sm font-semibold tracking-wide text-accent">
                        {store.current}
                    </span>
                    <span className="text-sm font-semibold tracking-wide text-dimmed">
                        / {store.total}
                    </span>
                </div>
            </div>

            <motion.div className="w-full">
                <ProgressBar />
            </motion.div>

            <div className="mt-2 flex w-full flex-row items-center justify-between">
                <div className="flex flex-row gap-10">
                    <DownloadStat title="Speed" value={store.speed} />
                    <DownloadStat title="Disk usage" value="0 B/S" />
                    <DownloadStat title="Remaining" value={store.eta} />
                </div>
                <div className="flex flex-row gap-2">
                    <motion.button className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
                        <PauseIcon
                            className="fill-primary stroke-transparent"
                            size={20}
                        />
                    </motion.button>
                    <motion.button className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
                        <GaugeIcon
                            className="fill-transparent stroke-primary"
                            size={20}
                            strokeWidth={3}
                        />
                    </motion.button>
                </div>
            </div>
        </div>
    );
}
