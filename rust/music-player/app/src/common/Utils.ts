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

export const storeSet = <T>(key: string, value: T) => {
    localStorage.setItem(key, JSON.stringify(value));
};

export const storeGet = <T>(key: string): T | null => {
    const value = localStorage.getItem(key);
    if (value) {
        return JSON.parse(value);
    }
    return null;
};
