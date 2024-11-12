console.log(Array.from({ length: 10 }));

function debounce(func: (...args: any[]) => void, wait: number) {
    let timeout: number;
    return function (...args: any[]) {
        const later = () => {
            timeout && clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}

