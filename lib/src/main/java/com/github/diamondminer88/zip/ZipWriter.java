package com.github.diamondminer88.zip;

import java.io.Closeable;
import java.io.File;
import java.nio.charset.StandardCharsets;
import java.util.Collection;

@SuppressWarnings("unused")
public class ZipWriter implements Closeable {
    static {
        System.loadLibrary("ziprs");
    }

    /**
     * Internal pointer to ZipWriter struct
     */
    private final long ptr = 0;

    /**
     * Creates an archive to write to. This overwrites any existing archive.
     * @param path Path to new archive
     */
    public ZipWriter(String path) {
        open(path, false);
    }

    /**
     * Opens/creates an archive to write to.
     * @param path   Path to archive
     * @param append Append to existing archive
     */
    public ZipWriter(String path, boolean append) {
        open(path, append);
    }

    /**
     * Creates an archive to write to. This overwrites any existing archive.
     * @param file Path to new archive
     */
    public ZipWriter(File file) {
        open(file.getAbsolutePath(), false);
    }

    /**
     * Opens/creates an archive to write to.
     * @param file   Path to archive
     * @param append Append to existing archive
     */
    public ZipWriter(File file, boolean append) {
        open(file.getAbsolutePath(), append);
    }

//    /**
//     * Append to an existing archive.
//     * @param data Existing archive's bytes
//     */
//    public ZipWriter(byte[] data) {
//        open(data);
//    }

    /**
     * Opens/creates an archive to write to.
     * @param path   Path to archive
     * @param append Append to existing archive
     */
    private native void open(String path, boolean append);

    /**
     * Append to an existing archive.
     * @param input Existing archive's bytes
     */
    private native void open(byte[] input);

    /**
     * Sets the comment for the zip archive.
     * @param comment Comment raw bytes (doesn't have to be UTF-8)
     */
    public native void setComment(byte[] comment);

    /**
     * Sets the comment for the zip archive.
     */
    public void setComment(String comment) {
        setComment(comment.getBytes(StandardCharsets.UTF_8));
    }

    /**
     * Internal method for writing an entry
     */
    private native void writeEntry(String path, byte[] data, int compression, int alignment);

    /**
     * Create a deflate-compressed unaligned entry and write bytes to it.
     * @param path Path to entry inside the archive
     * @param data Raw data
     */
    public void writeEntry(String path, byte[] data) {
        writeEntry(path, data, ZipCompression.DEFLATE.internal, 0);
    }

    /**
     * Create an <b>unaligned</b> entry with specific compression and write bytes to it.
     * @param path Path to entry inside the archive
     * @param data Raw data
     * @param compression The target compression for the entry
     */
    public void writeEntry(String path, byte[] data, ZipCompression compression) {
        writeEntry(path, data, compression.internal, 0);
    }

    /**
     * Create an aligned entry with specific compression.
     * @param path Path to entry inside the archive
     * @param data Raw data
     * @param compression The target compression for the entry
     * @param alignment The target alignment for the entry data from the start of the zip. This is commonly used for zip-aligning .so's inside apks so extractNativeLibs can be set to false.
     */
    public void writeEntry(String path, byte[] data, ZipCompression compression, int alignment) {
        writeEntry(path, data, compression.internal, alignment);
    }

    /**
     * Create a deflate-compressed unaligned entry and write to it.
     * @param path    Path to entry inside the archive
     * @param content Content that will be encoded as UTF-8
     */
    public void writeEntry(String path, String content) {
        writeEntry(path, content.getBytes(StandardCharsets.UTF_8), ZipCompression.DEFLATE.internal, 0);
    }

    /**
     * Create a directory in the archive.
     * @param path Path to directory. Will automatically append a `/` if the path does not end with one already.
     */
    public native void writeDir(String path);

    /**
     * Delete an entry from this archive.
     * @param path Path to entry in the archive.
     */
    public void deleteEntry(String path) {
        deleteEntry(path, false);
    }

    /**
     * Delete an entry from this archive.
     * @param path Path to entry in the archive.
     * @param fillVoid Keep other entries' alignment by only removing the entry from the central directory and replacing the file content with nulls.
     */
    public native void deleteEntry(String path, boolean fillVoid);

    /**
     * Delete entries from this archive.
     * @param paths Target paths of entries
     */
    public native void deleteEntries(String... paths);

    /**
     * Delete entries from this archive.
     * @param paths Target paths of entries
     */
    public void deleteEntries(Collection<String> paths) {
        var entriesArr = new String[paths.size()];
        paths.toArray(entriesArr);

        deleteEntries(entriesArr);
    }

    /**
     * Finalizes the writer and saves to disk.
     * You cannot use this ZipWriter instance after closing it.
     */
    @Override
    public native void close();
}
