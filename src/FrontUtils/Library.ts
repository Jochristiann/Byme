export const title = "Byme"

export const formatLikeNumber = (num: number): string => {
    if (num >= 1_000_000_000) {
        const billions = num / 1_000_000_000;
        return billions % 1 === 0 ? `${billions}B` : `${billions.toFixed(1)}B`;
    }else if (num >= 1_000_000) {
        const millions = num / 1_000_000;
        return millions % 1 === 0 ? `${millions}M` : `${millions.toFixed(1)}M`;
    }

    return num.toLocaleString('en-US');
}