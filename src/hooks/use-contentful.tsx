import { EntryCollection, EntrySkeletonType, createClient } from 'contentful';
import { useEffect, useState } from 'react';

const ACCESS_TOKEN = '';
const SPACE_ID = '';

const client = createClient({
    accessToken: ACCESS_TOKEN,
    space: SPACE_ID,
});

export function useContentful() {
    const [data, setData] = useState<EntryCollection<
        EntrySkeletonType,
        undefined,
        string
    > | null>(null);
    const [loading, setLoading] = useState<boolean>(true);

    useEffect(() => {
        client
            .getEntries({
                skip: 0,
                limit: 5,
                content_type: 'article',
                order: '-fields.publishDatetime',
                select: 'sys.id,fields.metaImage,fields.title,fields.publishDatetime,fields.tags,fields.slug',
                locale: 'en-US',
            })
            .then((entries) => setData(entries))
            .finally(() => setLoading(false));
    }, []);

    return { loading, data };
}
