# zip-android ![Hits](https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2FDiamondMiner88%2Fzip-android&count_bg=%2379C83D&title_bg=%23555555&icon=github.svg&icon_color=%23E7E7E7&title=views&edge_flat=true) ![Maven version](https://img.shields.io/maven-metadata/v?metadataUrl=https%3A%2F%2Fredditvanced.ddns.net%2Fmaven%2Freleases%2Fcom%2Fgithub%2Fdiamondminer88%2Fzip-android%2Fmaven-metadata.xml&style=flat-square)

Java Android bindings for [zip-rs](https://github.com/zip-rs/zip), a native rust zip library.

### Installation

```kotlin
repositories {
    maven("https://redditvanced.ddns.net/maven")
}

dependencies {
    implementation("com.github.diamondminer88:zip-android:1.0.0")
}
```

### Usage (Kotlin)

```kotlin
ZipReader(zipFile).use { zip ->
    "Entry count: ${zip.entryCount}"
    "Entries: ${zip.entryNames.joinToString()}"

    // Loop over entries
    zip.forEach {
        "Entry: ${it.name} size: ${it.size} modified: ${it.lastModified}"
        if (!it.isDir) {
            "Content: ${it.read().decodeToString()}"
        }
    }
}

ZipWriter(zipFile).use { zip ->
    zip.setComment("a comment".toByteArray())
    zip.writeEntry("test.txt", "hot garbage")
    zip.writeDir("com/github/diamondminer88/zip")
    zip.deleteEntries("abc.txt")

    // Use this for .so, with zipalign-ing
    zip.writeEntryUncompressed("uncompressed.txt", "ihy".toByteArray())
}
```
