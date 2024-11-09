export interface FileInfo {
    path: string;
    totalDuration: string;
    size: string;
    mb: string;
    title: string;
    artist: string;
    album: string;
}

export interface OwnFileInfo extends FileInfo {
    originIndex: number;
}

export interface PlayerCtl {
    currentIndex: number;
    isPlaying: boolean;
    isPause: boolean;
}
