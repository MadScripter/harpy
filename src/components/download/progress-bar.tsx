import { motion } from 'framer-motion';

import { useDownloadStore } from '@stores';

export function ProgressBar() {
    const { progress } = useDownloadStore();

    return (
        <div className="relative flex h-3.5 w-full items-center rounded-full bg-[#3E2723] px-1">
            <motion.div
                className="h-2 w-0 rounded-full bg-accent shadow-base shadow-accent/30"
                animate={{
                    width: `${progress}%`,
                }}
            />
        </div>
    );
}
