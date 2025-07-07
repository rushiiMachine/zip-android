package com.github.diamondminer88.zip;

@SuppressWarnings("unused")
public enum ZipCompression {
    UNSUPPORTED(-1),
    NONE(0),
    DEFLATE(1),
    BZIP2(2),
    ZSTD(3);

    public final byte internal;

    ZipCompression(int internal) {
        this.internal = (byte) internal;
    }

    public static ZipCompression fromInternal(int internal) {
        return values()[internal + 1];
    }
}
