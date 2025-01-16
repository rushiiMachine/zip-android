# zip-android ![Maven central version](https://img.shields.io/maven-central/v/io.github.diamondminer88/zip-android?style=flat-square) 

Android JNI bindings for [zip-rs](https://github.com/zip-rs/zip), a native rust zip library.

### Installation

```kotlin
repositories {
    mavenCentral()
}

dependencies {
    implementation("io.github.diamondminer88:zip-android:2.1.1@aar")
}
```

### Example Usage (Kotlin)

```kotlin
ZipReader(zipFile).use { zip ->
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

// Close reader/writer by using .use {} (kotlin) or .close()/try block for java
ZipWriter(zipFile).use { zip ->
    zip.setComment("a comment".toByteArray())
    zip.writeEntry("compressed.txt", "hot garbage")
    zip.writeEntry("data/compressed_bytes.txt", bytes)
    zip.writeDir("com/github/diamondminer88/zip")
    zip.deleteEntries("abc.txt", "guh.txt")
    zip.deleteEntry("husk.txt")

    // Delete entry from central dir and keep the existing alignment (useful for writing zip aligned .so's) 
    zip.deleteEntry("lib.so", /* fillVoid = */ true)
    // Write page-aligned (4096 byte) uncompressed entry (useful for writing zip aligned .so's)
    zip.writeEntry("lib.so", bytes, ZipCompression.NONE, 4096)
}

// Open zip reader from in memory byte array
ZipReader(byteArrayOf(/* ... */)).use { zip ->
    // Parsed zip from bytes!
}
```

### Building Prerequisites
1. `rustup install nightly && rustup default nightly`
2. `rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android`
3. `cargo install --force cargo-ndk`
