package com.github.diamondminer88.zip;

import org.jetbrains.annotations.Nullable;

import java.io.Closeable;
import java.io.File;
import java.util.Iterator;

@SuppressWarnings("unused")
public class ZipReader implements Closeable {
    static {
        System.loadLibrary("ziprs");
    }

    @SuppressWarnings("FieldMayBeFinal")
    private long innerPtr = 0;

    /**
     * Open a zip file with readonly operations
     * @param path Path to the archive
     */
    public ZipReader(String path) {
        open(path);
    }

    /**
     * Open a zip with readonly operations
     * @param file File of the archive
     */
    public ZipReader(File file) {
        open(file.getAbsolutePath());
    }

    /**
     * Opens an archive and sets {@link ZipReader#innerPtr} to the struct
     */
    private native void open(String path);

    /**
     * Destructs the ZipArchive at {@link ZipReader#innerPtr}
     * This MUST be called otherwise you can have a memory leak.
     */
    @Override
    public native void close();

    /**
     * Get a contained file by index. Returns null if entry not found.
     * @param index Index of the file.
     */
    @Nullable
    public native ZipEntry openEntry(int index);

    /**
     * Search for a file entry by name. Returns null if entry not found.
     * @param path Path to the file inside the archive.
     */
    @Nullable
    public native ZipEntry openEntry(String path);

    /**
     * Get a contained file by index without decompressing it.
     * @param index Index of the file.
     */
    @Nullable
    public native ZipEntry openEntryRaw(int index);

    /**
     * Number of files contained in this zip.
     */
    public native long getEntryCount();

    class ZipEntryNameIterator implements Iterator<String> {
        @SuppressWarnings("FieldMayBeFinal")
        private long innerPtr = 0;

        @Override
        public native boolean hasNext();

        @Override
        public native String next();
    }
}
