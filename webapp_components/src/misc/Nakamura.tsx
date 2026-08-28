import { InternalLink } from "../links/InternalLink";
import { formatDate } from "../utils";

export function Nakamura({
    endDate
}: NakamuraProps) {
    const milliseconds = new Date(endDate + 'Z').valueOf();
    const nakamuraLink = `https://time.nakamura-labs.com/?#${milliseconds / 1000}`;

    return <>
        <InternalLink
            to={nakamuraLink}
            target="_blank"
            content={formatDate(milliseconds)}
        />
    </>
}

export type NakamuraProps = {
    endDate: string,
}
