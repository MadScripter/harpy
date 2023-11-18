import { useContentful } from '@hooks';
import { AnimatePresence } from 'framer-motion';
import { MoveRightIcon } from 'lucide-react';
import { useState } from 'react';

import { NewsCard } from './news-card';
import { NewsPagination } from './news-pagination';

export function NewsCarousel() {
    const { data } = useContentful();
    const [currentIndex, setCurrent] = useState<number>(0);

    const totalArticles = data?.items.length ?? 0;

    const handleNext = () => {
        setCurrent((prev) => (prev === totalArticles - 1 ? 0 : prev + 1));
    };

    const onPaginationClick = (article: number) => {
        setCurrent(article);
    };

    const news = data?.items
        .slice(currentIndex, currentIndex + 2)
        .concat(
            data?.items.slice(0, Math.max(0, currentIndex - totalArticles + 2)),
        )
        .map((item) => <NewsCard key={item.sys.id} item={item} />);

    return (
        <div className="relative flex h-full flex-col justify-center">
            <div className="z-10 flex w-80 flex-row items-center justify-between">
                <h2 className="text-lg font-medium text-white">
                    Latest Articles
                </h2>
                <NewsPagination
                    total={totalArticles}
                    current={currentIndex}
                    onClick={onPaginationClick}
                />
            </div>

            <div className="relative mt-2 flex w-fit flex-row items-center justify-center">
                <div className="absolute -left-64 -top-24 z-0">
                    <img className="h-auto w-1/2" src="/character.png" />
                </div>

                <div className="flex h-auto w-full flex-row gap-2">
                    <AnimatePresence>{news}</AnimatePresence>
                </div>
            </div>

            <div className="relative flex w-fit items-center justify-center">
                <button
                    type="button"
                    className="absolute top-1/2 flex h-12 w-12 -translate-y-1/2 items-center justify-center rounded-full bg-secondary shadow-base shadow-primary"
                    onClick={handleNext}
                >
                    <MoveRightIcon size={24} className="fill-primary" />
                </button>
            </div>
        </div>
    );
}
