type Props = {
    title: string;
    value: string;
};

export function DownloadStat({ title, value }: Props) {
    return (
        <span className="flex h-auto w-fit select-none flex-col">
            <span className="text-sm font-semibold uppercase text-accent">
                {value}
            </span>
            <span className="text-xs font-medium uppercase text-dimmed">
                {title}
            </span>
        </span>
    );
}
