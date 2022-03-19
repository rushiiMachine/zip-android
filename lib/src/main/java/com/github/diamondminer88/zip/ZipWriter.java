package com.github.diamondminer88.zip;

import java.io.Closeable;
import java.io.File;
import java.nio.charset.StandardCharsets;

@SuppressWarnings("unused")
public class ZipWriter implements Closeable {
    @SuppressWarnings("FieldMayBeFinal")
    private long innerPtr = 0;

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

    /**
     * Append to an existing archive.
     * @param data Existing archive's bytes
     */
    public ZipWriter(byte[] data) {
        open(data);
    }

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
     * Set the comment for the zip archive.
     * @param comment UTF-8 encoded comment (usually)
     */
    public native void setComment(byte[] comment);

    /**
     * Create an entry and write bytes to it.
     * @param path Path to entry inside the archive
     * @param data Raw bytes
     */
    public native void writeEntry(String path, byte[] data);

    /**
     * Create an entry and write to it.
     * @param path    Path to entry inside the archive
     * @param content Content that will be encoded as UTF-8
     */
    public void writeEntry(String path, String content) {
        writeEntry(path, content.getBytes(StandardCharsets.UTF_8));
    }

    /**
     * Finalizes the archive and saves to disk.
     * You cannot use this ZipWriter instance after closing it.
     */
    @Override
    public native void close();
}
