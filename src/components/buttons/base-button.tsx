import { MouseEvent } from 'react';

type Props = {
    text: string;
    onClick: (e: MouseEvent<HTMLButtonElement>) => void;
};

export function BaseButton({ text: title, onClick }: Props) {
    return (
        <button
            type="button"
            className="absolute left-1/2 top-1/2 m-auto -translate-x-1/2 -translate-y-1/2 rounded-full bg-accent px-12 py-2.5 text-xl font-bold uppercase tracking-wide text-primary shadow-base transition-all duration-300 hover:shadow-accent/50"
            onClick={onClick}
        >
            {title}
        </button>
    );
}
