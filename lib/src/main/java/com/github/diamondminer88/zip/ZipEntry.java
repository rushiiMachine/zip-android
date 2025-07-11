package com.github.diamondminer88.zip;

import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

@SuppressWarnings("unused")
public class ZipEntry {
    /**
     * Internal pointer to ZipFile struct
     */
    private final long ptr = 0;

    /**
     * Called by JNI.
     */
    private ZipEntry() {
    }

    /**
     * Get the index of this file in the archive.
     * If trying to determine file alignment, consider {@link ZipEntry#getDataOffset()}.
     */
    public native int getIndex();

    /**
     * Get the name of the file.
     * <br/>
     * It is dangerous to use this name directly when extracting an archive.
     * It may contain an absolute path (/etc/shadow), or break out of the current directory (../runtime).
     * Carelessly writing to these paths allows an attacker to craft a ZIP archive that will overwrite critical files.
     */
    @NotNull
    public native String getName();

    /**
     * Get the comment of the file
     */
    @NotNull
    public native String getComment();

    /**
     * Get the time the file was last modified.
     * Note that zip files do not store timezone information, effectively rendering it useless for accurate information.
     * This timestamp will be relative to the original author's timezone.
     * @return UNIX timestamp (possibly UTC)
     */
    public native long getLastModified();

    /**
     * Whether the entry is a directory.
     */
    public native boolean isDir();

    /**
     * Whether the entry is a file.
     */
    public boolean isFile() {
        return !isDir();
    }

    /**
     * Get the unix mode for this file.
     */
    @Nullable
    public native Long getMode();

    /**
     * Get the CRC32 hash of the original file.
     */
    public native int getCRC32();

    /**
     * Get the extra data of the zip header for this file.
     */
    public native byte[] getExtraData();

    /**
     * Get the size of the file (bytes) when uncompressed.
     */
    public native long getSize();

    /**
     * Get the size of the file (in bytes) in the archive.
     */
    public native long getCompressedSize();

    /**
     * Internal method for getting the int value representing values in {@link ZipCompression}.
     */
    private native int _getCompression();

    /**
     * Get the compression type that this entry is compressed with.
     */
    public ZipCompression getCompression() {
        return ZipCompression.fromInternal(_getCompression());
    }

    /**
     * Get the zip entry data offset inside the archive.
     * This is not particularly useful other than determining if
     * a file is aligned to a specific boundary.
     */
    public native long getDataOffset();

    /**
     * Reads this file entry's data (decompressed or not depending on how this entry was opened)
     */
    public native byte[] read();

    /**
     * Drops the ZipFile struct internally to prevent a memory leak.
     */
    private native void _finalize();

    @Override
    protected void finalize() throws Throwable {
        _finalize();
        super.finalize();
    }
}
