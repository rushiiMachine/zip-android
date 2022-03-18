package com.github.diamondminer88.zip;

import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import java.io.Closeable;
import java.io.File;
import java.util.Iterator;
import java.util.Objects;

@SuppressWarnings("unused")
public class ZipReader implements Closeable, Iterable<ZipEntry> {
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
     * Opens an archive and sets {@link ZipReader#innerPtr} to the native data.
     */
    private native void open(String path);

    /**
     * Destructs the underlying native ZipArchive at {@link ZipReader#innerPtr}
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
     * Number of files contained in this archive.
     */
    public native int getEntryCount();

    /**
     * Gets all the entry names (including dirs) in this archive.
     * This does <b>NOT</b> preserve entry index order.
     * Use this instead of
     */
    @NotNull
    public native String[] getEntryNames();

    /**
     * Get an iterator for all the entries contained in this archive.
     */
    @NotNull
    @Override
    public Iterator<ZipEntry> iterator() {
        return new Iterator<>() {
            private final int totalEntries = getEntryCount();
            int cursor = 0;

            @Override
            public boolean hasNext() {
                return cursor != totalEntries;
            }

            @NotNull
            @Override
            public ZipEntry next() {
                return Objects.requireNonNull(openEntry(cursor++));
            }
        };
    }
}
