# zip-android ![Maven central version](https://img.shields.io/maven-central/v/io.github.diamondminer88/zip-android?style=flat-square) 

Android JNI bindings for [zip-rs](https://github.com/zip-rs/zip), a native rust zip library.

This has significant performance improvements compared to similar libraries
written in JVM languages.

### Installation

```kotlin
repositories {
    mavenCentral()
}

dependencies {
    implementation("io.github.diamondminer88:zip-android:2.2.0@aar")
}
```

### Example Usage (Kotlin)

```kotlin
// Open a zip reader from a java.io.File to be read from disk
// You can autoclose reader/writer by using .use {} (kotlin) or .close()/try block for java
ZipReader(file).use { zip ->
    "Entry count: ${zip.entryCount}"
    "Entries: ${zip.entryNames.joinToString()}"

    // Loop over entries
    zip.entries().forEach { /* guh */ }
    zip.forEach {
        "Entry: ${it.name} size: ${it.size} modified: ${it.lastModified}"
        if (it.isFile) {
            "Content: ${it.read().decodeToString()}"
        }
    }
}

// Open zip reader from an in memory byte array
ZipReader(byteArrayOf(/* ... */)).use { zip ->
    // Parsed in-memory zip!
}

// Open a zip writer to the specified java.io.File path and overwrite the original file
// If you wish to append to an existing zip at that location, set the append parameter to `true`.
ZipWriter(file, /* append = */ false).use { zip ->
    zip.setComment("a comment".toByteArray())
    zip.writeEntry("compressed.txt", "hot garbage")
    zip.writeEntry("data/compressed_bytes.txt", bytes)
    zip.writeDir("com/github/diamondminer88/zip")
    zip.deleteEntries("abc.txt", "guh.txt")
    zip.deleteEntry("husk.txt")

    // Write page-aligned (4096 byte) uncompressed entry (useful for writing zip aligned .so's)
    zip.writeEntry("lib.so", bytes, ZipCompression.NONE, 4096)
    // Delete entry from central dir, preserving alignment for all existing zip entries
    // If fillVoid is false, then it un-aligns all entries whose data comes after this one
    zip.deleteEntry("lib.so", /* fillVoid = */ true)
}

// Start a new in-memory zip file
val newZipBytes = ZipWriter().use { zip ->
    // ...
    zip.toByteArray() // Closes the zip and retrieves the bytes
}

// Open and append to an existing zip from an in memory byte array
val modifiedZipBytes = ZipWriter(byteArrayOf(/* ... */)).use { zip ->
    // ...
    zip.toByteArray() // Closes the zip and retrieves the modified zip file bytes
}
```

The available compression methods are: `Stored` (none), `Deflate` (default), `Bzip2`, and `Zstd`.

### Building Prerequisites
1. `rustup install nightly && rustup default nightly`
2. `rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android`
3. `cargo install --force cargo-ndk`
