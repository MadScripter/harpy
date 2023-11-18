import { MouseEventHandler, ReactNode } from 'react';

interface Props {
    onClick: MouseEventHandler<HTMLButtonElement>;
    children: ReactNode;
}

export function ControlBoxButton({ onClick, children }: Props) {
    return (
        <button
            type="button"
            className="inline-flex items-center justify-center rounded-full bg-secondary p-1 backdrop-blur-sm transition-all duration-300 hover:bg-accent"
            onClick={onClick}
        >
            {children}
        </button>
    );
}
