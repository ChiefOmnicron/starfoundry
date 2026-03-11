import { InternalLink } from "@internal/links/InternalLink";
import { formatDateUTC } from "@internal/utils";

export function Nakamura({
    endDate
}: NakamuraProps) {
    const milliseconds = new Date(endDate).valueOf();
    const nakamuraLink = `https://time.nakamura-labs.com/?#${milliseconds}`;

    return <>
        <InternalLink
            to={nakamuraLink}
            target="_blank"
            content={formatDateUTC(milliseconds)}
        />
    </>
}

export type NakamuraProps = {
    endDate: string,
}
