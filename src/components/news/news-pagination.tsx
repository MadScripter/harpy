import { motion } from 'framer-motion';

type Props = {
    total: number;
    current: number;
    onClick: (article: number) => void;
};

export function NewsPagination({ total: amount, current, onClick }: Props) {
    const dots = Array.from(Array(amount), (_, index) => {
        return (
            <motion.button
                type="button"
                className="h-1.5 w-1.5 rounded-full bg-secondary transition-all duration-300"
                initial={false}
                style={{
                    width: current === index ? 32 : 6,
                    backgroundColor: current === index ? '#FF8A65' : '#A1887F',
                }}
                transition={{ duration: 0.3, ease: 'easeInOut' }}
                key={index}
                onClick={() => onClick(index)}
            />
        );
    });

    return <div className="flex flex-row items-center gap-2">{dots}</div>;
}
