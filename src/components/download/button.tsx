import { motion } from 'framer-motion';
import { MouseEvent } from 'react';

type Props = {
    icon: React.ReactNode;
    onClick: (e: MouseEvent<HTMLElement>) => void;
};

export function Button({ icon, onClick }: Props) {
    return (
        <motion.button
            type="button"
            className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent"
            onClick={onClick}
        >
            {icon}
        </motion.button>
    );
}
