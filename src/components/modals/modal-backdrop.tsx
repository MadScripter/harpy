import { motion } from 'framer-motion';

type Props = {
    onClick: (e: React.MouseEvent) => void;
};

const variants = {
    hidden: {
        opacity: 0,
    },
    visible: {
        opacity: 1,
    },
};

const transition = {
    ease: 'easeInOut',
    duration: 0.2,
};

export function ModalBackdrop({ onClick }: Props) {
    return (
        <motion.div
            variants={variants}
            initial="hidden"
            animate="visible"
            exit="hidden"
            transition={transition}
            className="fixed inset-0 z-40 bg-black/40 backdrop-blur-sm"
            onClick={onClick}
        />
    );
}
