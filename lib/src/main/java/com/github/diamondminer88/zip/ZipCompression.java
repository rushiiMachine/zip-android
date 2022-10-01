package com.github.diamondminer88.zip;

import org.jetbrains.annotations.ApiStatus.Internal;

@SuppressWarnings("unused")
public enum ZipCompression {
    UNSUPPORTED(-1),
    NONE(0),
    DEFLATE(1),
    BZIP2(2),
    ZSTD(3);

    @Internal
    public final byte internal;

    ZipCompression(int internal) {
        this.internal = (byte) internal;
    }

    @Internal
    public static ZipCompression fromInternal(int internal) {
        return values()[internal+1];
    }
}
