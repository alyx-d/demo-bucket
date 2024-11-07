export const first = <T>(arr: T[]) => {
    if (arr.length > 0) {
        return arr[0];
    }
    throw new Error("arr is empty");
};

export const last = <T>(arr: T[]) => {
    if (arr.length > 0) {
        return arr[arr.length - 1];
    }
    throw new Error("arr is empty");
};

export const byteToMB = (byte: number): string => {
    return (byte / 1024 / 1024).toFixed(2) + "MB";
};