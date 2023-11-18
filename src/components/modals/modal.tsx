import { AnimatePresence, motion } from 'framer-motion';
import { XIcon } from 'lucide-react';
import React from 'react';

import { ModalBackdrop } from './modal-backdrop';

type Props = {
    title: string;
    isOpen: boolean;
    onClose: (e: React.MouseEvent) => void;
    children: React.ReactNode;
};

const variants = {
    initial: {
        opacity: 0,
        scale: 1,
        y: 20,
    },
    animate: {
        opacity: 1,
        scale: 1,
        y: 0,
    },
    exit: {
        opacity: 0,
        transition: {
            duration: 0.2,
        },
    },
};

const transition = {
    duration: 0.2,
    delay: 0.1,
    ease: 'easeInOut',
};

export function Modal({ title, isOpen, onClose, children }: Props) {
    return (
        <AnimatePresence initial={false}>
            {isOpen && (
                <>
                    <motion.div
                        variants={variants}
                        transition={transition}
                        initial="initial"
                        animate="animate"
                        exit="exit"
                        key="modal"
                        className="fixed inset-4 z-50 m-auto flex h-fit w-fit max-w-md flex-col rounded-lg bg-primary p-2 shadow-md"
                    >
                        <div className="mb-6 flex w-full flex-row items-center justify-between space-x-8">
                            <h2 className="ml-2 text-center text-white">
                                {title}
                            </h2>
                            <button
                                type="button"
                                className="flex items-center justify-center rounded-full bg-secondary p-1.5 transition-all duration-300 hover:bg-accent"
                                onClick={onClose}
                            >
                                <XIcon className="fill-primary" size={18} />
                            </button>
                        </div>
                        <div className="flex flex-1 flex-col items-center overflow-y-auto p-4">
                            {children}
                        </div>
                    </motion.div>
                    <ModalBackdrop onClick={onClose} />
                </>
            )}
        </AnimatePresence>
    );
}
