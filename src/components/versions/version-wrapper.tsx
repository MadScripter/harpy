type Props = {
    title: string;
    version: string;
};

export default function VersionWrapper({ title, version }: Props) {
    return (
        <div className="w-content flex h-auto select-none flex-row space-x-1.5 text-xs uppercase">
            <span className="font-normal text-dimmed">{title}</span>
            <span className="font-medium text-accent">{version}</span>
        </div>
    );
}
