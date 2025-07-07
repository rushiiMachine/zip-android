package com.github.diamondminer88.zip;

import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import java.io.Closeable;
import java.io.File;
import java.nio.charset.StandardCharsets;
import java.util.Iterator;
import java.util.Objects;

@SuppressWarnings("unused")
public class ZipReader implements Closeable, Iterable<ZipEntry> {
    static {
        System.loadLibrary("ziprs");
    }

    /**
     * Internal pointer to ZipArchive struct
     */
    private final long ptr = 0;

    /**
     * Open a zip file with readonly operations
     * @param path Path to the archive
     */
    public ZipReader(@NotNull String path) {
        open(path);
    }

    /**
     * Open a zip with readonly operations
     * @param file File of the archive
     */
    public ZipReader(@NotNull File file) {
        open(file.getAbsolutePath());
    }

    /**
     * Open a zip with readonly operations
     * @param data Zip file as a byte array
     */
    public ZipReader(byte @NotNull [] data) {
        open(data);
    }

    /**
     * Opens an archive and sets {@link ZipReader#ptr} to the native data.
     */
    private native void open(String path);

    /**
     * Parses an archive and sets {@link ZipReader#ptr} to the native data.
     */
    private native void open(byte[] data);

    /**
     * Destructs the underlying native ZipArchive at {@link ZipReader#ptr}
     * This MUST be called otherwise you can have a memory leak.
     */
    @Override
    public native void close();

    /**
     * Get a contained file by index. Returns null if entry not found.
     * @param index Index of the file.
     */
    @Nullable
    public ZipEntry openEntry(int index) {
        return openEntry0(index, null, false);
    }

    /**
     * Search for a file entry by name. Returns null if entry not found.
     * @param path Path to the file inside the archive.
     */
    @Nullable
    public ZipEntry openEntry(@NotNull String path) {
        return openEntry0(-1, path, false);
    }

    /**
     * Get a contained file by index without decompressing it.
     * @param index Index of the file.
     */
    @Nullable
    public ZipEntry openEntryRaw(int index) {
        return openEntry0(index, null, true);
    }

    /**
     * Search for a file entry by name. Returns null if entry not found.
     * Gets the contained file without decompressing it.
     * @param path Path to the file inside the archive.
     */
    @Nullable
    public ZipEntry openEntryRaw(@NotNull String path) {
        return openEntry0(-1, path, true);
    }

    private native ZipEntry openEntry0(int index, @Nullable String name, boolean raw);

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
     * Gets the raw comment on this archive.
     */
    public native byte[] getRawComment();

    /**
     * Gets the comment on this archive.
     * @return String decoded as UTF-8
     */
    public String getComment() {
        return new String(getRawComment(), StandardCharsets.UTF_8);
    }

    /**
     * Loop over all the entries within this zip.
     * <b>If you are trying to loop over names, use {@link ZipReader#getEntryNames()} instead.</b>
     * @return {@link Iterator} on every zip entry.
     */
    public Iterator<ZipEntry> getEntries() {
        return iterator();
    }

    /**
     * Iterate over all the entries contained in this archive.
     * Opens entry with decompressing.
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
