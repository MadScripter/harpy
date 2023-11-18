import { Entry, EntrySkeletonType } from 'contentful';
import { motion } from 'framer-motion';
import TimeAgo from 'react-timeago';

type Props = {
    item: Entry<EntrySkeletonType, undefined, string>;
};

export function NewsCard({ item }: Props) {
    const date = new Date(item.fields.publishDatetime);

    const image = `https:${item.fields.metaImage.fields.file.url}`;
    const imageTitle = item.fields.metaImage.fields.title;
    const title = item.fields.title;
    const url = `https://www.palia.com/news${item.fields.slug}`;
    const tags = item.fields.tags
        ?.map((tag) => (tag.fields ? tag.fields.label : null))
        .filter(Boolean);

    return (
        <motion.a
            className="relative h-48 w-80 overflow-hidden rounded-2xl bg-transparent"
            href={url}
            target="_blank"
            rel="noreferrer"
            initial={{ x: 50, opacity: 0 }}
            animate={{ x: 0, opacity: 1 }}
            transition={{ duration: 0.2 }}
        >
            <article>
                <img
                    src={image}
                    className="object-fit absolute inset-0 aspect-auto h-full w-full shadow-md"
                    alt={imageTitle}
                />
                <div className="absolute flex h-full w-full items-end bg-gradient-to-t from-primary/90 to-primary/5">
                    <div className="mx-4 mb-4 flex w-full flex-col">
                        <ul className="flex flex-row space-x-1">
                            {tags &&
                                tags.map((tag) => (
                                    <li
                                        key={tag}
                                        className="rounded-full bg-primary/80 px-2 py-0.5 text-xs font-normal tracking-wide text-white backdrop-blur-sm"
                                    >
                                        {tag}
                                    </li>
                                ))}
                        </ul>
                        <div className="mt-1 flex w-full flex-col">
                            <p className="line-clamp-2 text-lg font-medium text-white shadow-black text-shadow">
                                {title}
                            </p>
                            <div className="text-xs tracking-wider text-white/60">
                                <TimeAgo date={date} />
                            </div>
                        </div>
                    </div>
                </div>
            </article>
        </motion.a>
    );
}
