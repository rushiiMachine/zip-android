# zip-android

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

    zip.forEach {
        "Entry: ${it.name} size: ${it.size} modified: ${it.lastModified}"
        if (!it.isDir) {
            "Content: ${it.readEntry().decodeToString()}"
        }
    }
}

ZipWriter(zipFile).use { zip ->
    zip.setComment("a comment".toByteArray())
    zip.writeEntry("test.txt", "hot garbage")
}
```
