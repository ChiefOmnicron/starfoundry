export const formatNumber = (
    numberToFormat: number,
    withComma: boolean = false,
): string => {
    if (Number.isNaN(numberToFormat)) {
        return '';
    }

    const splitted = numberToFormat.toFixed(2).toString().split('.');
    const formatValue = splitted[0].split('').reverse().join('');
    const result: string[] = [];
    let count = 0;

    for (let i = 0; i < formatValue.length; i++) {
        let val = formatValue.charAt(i);

        if (val === '-' || val === '+') {
            result.push(val);
            continue;
        }

        if (count === 3) {
            result.push('.');
            count = 0;
        }
        result.push(val);
        count += 1;
    }

    if (withComma && splitted[1]) {
        return result.reverse().join('') + ',' + splitted[1];
    } else {
        return result.reverse().join('');
    }
};

export const formatTime = (numberToFormat: number): string => {
    const WEEK = 60 * 60 * 24 * 7; // seconds * minutes * hours * days
    const DAY = 60 * 60 * 24; // seconds * minutes * hours
    const HOUR = 60 * 60; // seconds * minutes
    const MINUTE = 60; // seconds

    const weeks = Math.floor(numberToFormat / WEEK);
    numberToFormat -= weeks * WEEK;

    const days = Math.floor(numberToFormat / DAY);
    numberToFormat -= days * DAY;

    const hours = Math.floor(numberToFormat / HOUR);
    numberToFormat -= hours * HOUR;

    const minutes = Math.floor(numberToFormat / MINUTE);
    numberToFormat -= minutes * MINUTE;

    const seconds = numberToFormat;

    const preZero = (value: number): string => (value >= 10 ? `${value}` : `0${value}`);

    if (weeks > 0) {
        return `${weeks}w ${preZero(days)}d ${preZero(hours)}h ${preZero(minutes)}m ${preZero(seconds)}s`;
    } else if (days > 0) {
        return `${days}d ${preZero(hours)}h ${preZero(minutes)}m ${preZero(seconds)}s`;
    } else if (hours > 0) {
        return `${hours}h ${preZero(minutes)}m ${preZero(seconds)}s`;
    } else if (minutes > 0) {
        return `${minutes}m ${preZero(seconds)}s`;
    } else {
        return `${seconds}s`;
    }
};

export const formatDateUTC = (dateMs: number): string => {
    const preZero = (val: number): string => (val >= 10 ? `${val}` : `0${val}`);

    const date = new Date(dateMs);

    const day = preZero(date.getUTCDate());
    const month = preZero(date.getUTCMonth() + 1);
    const year = date.getUTCFullYear();
    const hours = preZero(date.getUTCHours());
    const minutes = preZero(date.getUTCMinutes());

    return `${year}.${month}.${day} ${hours}:${minutes}`;
};

export const formatDate = (dateMs: number): string => {
    const preZero = (val: number): string => (val >= 10 ? `${val}` : `0${val}`);

    const offset = new Date().getTimezoneOffset();
    const date = new Date(new Date(dateMs).getTime() + offset);

    const day = preZero(date.getDate());
    const month = preZero(date.getMonth() + 1);
    const year = date.getFullYear();
    const hours = preZero(date.getHours());
    const minutes = preZero(date.getMinutes());

    return `${year}.${month}.${day} ${hours}:${minutes}`;
};
