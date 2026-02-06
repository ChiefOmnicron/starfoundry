export type Uuid = string;
export type TypeId = number;

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
