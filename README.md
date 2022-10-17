# zip-android ![Hits](https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2FDiamondMiner88%2Fzip-android&count_bg=%2379C83D&title_bg=%23555555&icon=github.svg&icon_color=%23E7E7E7&title=views&edge_flat=true) ![Amulet maven version](https://img.shields.io/maven-metadata/v?label=maven-amulet&metadataUrl=https%3A%2F%2Fredditvanced.ddns.net%2Fmaven%2Freleases%2Fcom%2Fgithub%2Fdiamondminer88%2Fzip-android%2Fmaven-metadata.xml&style=flat-square) ![Maven central version](https://img.shields.io/maven-central/v/io.github.diamondminer88/zip-android?style=flat-square) 

Android JNI bindings for [zip-rs](https://github.com/zip-rs/zip), a native rust zip library.

### Installation

```kotlin
repositories {
    // Maven central (recommended)
    mavenCentral()
    
    // ---------OR---------
    
    // Amulet maven (mine)
    maven("https://redditvanced.ddns.net/maven/releases")
}

dependencies {
    // Maven central (recommended)
    implementation("io.github.diamondminer88:zip-android:2.1.0@aar")

    // ---------OR---------
    
    // Amulet maven
    implementation("com.github.diamondminer88:zip-android:2.1.0@aar")
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

    // Delete entry from central dir and keep the alignment (useful for writing zip aligned .so's) 
    zip.deleteEntry("lib.so", true);
    // Write page-aligned (4096 byte) uncompressed entry (useful for writing zip aligned .so's)
    zip.writeEntry("lib.so", bytes, ZipCompression.NONE, 4096)
}
```

### Building Prerequisites
1. `rustup install nightly && rustup default nightly`
2. `rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android`
3. `cargo install --force cargo-ndk`
