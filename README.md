# zip-android

Java Android bindings for [zip-rs](https://github.com/zip-rs/zip), a native rust zip library.

### Installation

```kotlin
repositories {
    maven("https://jitpack.io")
}

dependencies {
    implementation("com.github.DiamondMiner88:zip-android:master-SNAPSHOT")
}
```

### Usage

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
```
