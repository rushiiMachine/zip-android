package com.github.diamondminer88.zip;

import java.io.File;
import java.util.Arrays;

@SuppressWarnings("unused")
public class ZipUtil {
    /**
     * Delete entries from a copy of this archive.
     * This creates a new archive under the hood <i>for now</i>.
     * @param entries Target paths of entries
     */
    public static void deleteEntries(String path, String... entries) {
        deleteEntries(new File(path), entries);
    }

    /**
     * Delete entries from a copy of this archive.
     * This creates a new archive under the hood <i>for now</i>.
     * @param entries Target paths of entries
     */
    public static void deleteEntries(File file, String... entries) {
        var list = Arrays.asList(entries);
        var tmpFile = new File(file.getAbsolutePath() + ".tmp");

        var reader = new ZipReader(file.getAbsolutePath());
        var writer = new ZipWriter(tmpFile);

        for (var entry : reader) {
            if (list.contains(entry.getName()))
                continue;

            if (!entry.isDir())
                writer.writeEntry(entry.getName(), entry.read());
        }

        reader.close();
        writer.close();
        assert tmpFile.renameTo(file);
    }
}
