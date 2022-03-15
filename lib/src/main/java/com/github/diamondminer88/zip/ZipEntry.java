package com.github.diamondminer88.zip;

@SuppressWarnings("unused")
public class ZipEntry {
    private final long ptr = 0;

    /**
     * Called by JNI.
     */
    private ZipEntry() {
    }

    /**
     * Get the name of the file.
     * <br/>
     * It is dangerous to use this name directly when extracting an archive.
     * It may contain an absolute path (/etc/shadow), or break out of the current directory (../runtime).
     * Carelessly writing to these paths allows an attacker to craft a ZIP archive that will overwrite critical files.
     */
    private native String getName();

    /**
     * Get the comment of the file
     */
    private native String getComment();

//    /**
//     * Get the time the file was last modified.
//     * @return UNIX time
//     */
//    private native long getLastModified();

    /**
     * Returns whether the file is a directory.
     */
    private native long isDir();

    /**
     * Get the unix mode for this file.
     * Nullable.
     */
    private native Long getMode();

    /**
     * Get the CRC32 hash of the original file.
     */
    private native int getCRC32();

    /**
     * Get the extra data of the zip header for this file.
     */
    private native byte[] getExtraData();

    /**
     * Get the size of the file (bytes) when uncompressed.
     */
    private native long getSize();

    /**
     * Get the size of the file (in bytes) in the archive.
     */
    private native long getCompressedSize();
}
