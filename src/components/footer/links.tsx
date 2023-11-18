/* eslint-disable jsx-a11y/anchor-is-valid */
import { Link } from './link';

const links = [
    {
        title: 'Privacy',
        url: 'https://www.palia.com/privacy',
    },
    {
        title: 'Terms of service',
        url: 'https://www.palia.com/terms',
    },
    {
        title: 'Support',
        url: 'https://support.palia.com/',
    },
];

export function Links() {
    return (
        <ul className="flex h-full flex-row items-center space-x-2 text-xs font-normal uppercase tracking-wider">
            {links.map((item, index) => (
                <>
                    <Link key={item.title} title={item.title} url={item.url} />
                    {index !== links.length - 1 && (
                        <div className="h-1 w-1 rounded-full bg-dimmed/50" />
                    )}
                </>
            ))}
        </ul>
    );
}
