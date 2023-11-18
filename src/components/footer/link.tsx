type Props = {
    title: string;
    url: string;
};

export function Link({ title, url }: Props) {
    return (
        <li className="group text-dimmed transition-all duration-300 hover:text-accent">
            <a href={url} target="_blank" rel="noreferrer">
                {title}
                <span className="block h-0.5 max-w-0 bg-accent/70 transition-all duration-300 group-hover:max-w-full" />
            </a>
        </li>
    );
}
