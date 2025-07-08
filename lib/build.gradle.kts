@file:Suppress("UnstableApiUsage")

group = "com.github.diamondminer88"
version = "2.3.0"

plugins {
    id("com.android.library")
    id("org.mozilla.rust-android-gradle.rust-android")
    id("com.vanniktech.maven.publish")
}

repositories {
    mavenCentral()
    google()
}

dependencies {
    compileOnly("org.jetbrains:annotations:26.0.2")
}

android {
    namespace = "com.github.diamondminer88.zip"
    ndkVersion = "29.0.13599879" // r28+ compiles for 16-KiB aligned pages by default
    compileSdk = 36

    defaultConfig {
        minSdk = 21

        consumerProguardFile("consumer-rules.pro")
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
    }

    buildFeatures {
        buildConfig = false
        resValues = false
    }
}

cargo {
    module = "../jni"
    profile = "release"
    libname = "ziprs"
    targets = listOf("arm", "arm64", "x86", "x86_64")
}

afterEvaluate {
    tasks["mergeDebugJniLibFolders"].dependsOn("cargoBuild")
    tasks["mergeReleaseJniLibFolders"].dependsOn("cargoBuild")
}

tasks.getByName<Delete>("clean") {
    delete("../jni/target")
    delete("../zip/target")
}

task<Jar>("sourcesJar") {
    from(android.sourceSets.named("main").get().java.srcDirs)
    archiveClassifier.set("sources")
}

task<Javadoc>("javadoc") {
    dependsOn("generateReleaseRFile")
    source(android.sourceSets.named("main").get().java.srcDirs)
    options {
        title = "zip-android $version"
        windowTitle = "zip-android $version"
        classpath(android.bootClasspath)

        // Holy fuck Javadoc extension is so shit
        val customOptionsFile = temporaryDir.resolve("custom_javadoc.options")
            .also { it.createNewFile() }
            .also { it.writeText("-Xdoclint:none") }

        optionFiles(customOptionsFile)
    }

    afterEvaluate {
        val libPaths = android.libraryVariants
            .map { it.javaCompileProvider.get().classpath.files }
            .let { files(it) }

        tasks.getByName<Javadoc>("javadoc")
            .classpath += libPaths
    }
}

task<Jar>("javadocJar") {
    from(tasks["javadoc"].outputs)
    archiveClassifier.set("javadoc")
}

mavenPublishing {
    publishToMavenCentral()
    coordinates("io.github.diamondminer88", "zip-android")

    pom {
        name = "zip-android"
        description = "Native zip library + java interface for android"
        url = "https://github.com/rushiiMachine/zip-android"
        inceptionYear = "2022"

        licenses {
            license {
                name = "Apache 2.0 license"
                url = "https://github.com/rushiiMachine/zip-android/blob/master/LICENSE"
                comments = "zip-android, thiserror, jni_fn, jni license"
            }
            license {
                name = "MIT license"
                url = "https://github.com/zip-rs/zip/blob/master/LICENSE"
                comments = "zip-rs, thiserror, jni_fn, jni license"
            }
        }

        developers {
            developer {
                id = "rushii"
                name = "rushii"
                url = "https://github.com/rushiiMachine/"
            }
        }

        scm {
            url = "https://github.com/rushiiMachine/zip-android"
            connection = "scm:git:github.com/rushiiMachine/zip-android.git"
            developerConnection = "scm:git:ssh://github.com/rushiiMachine/zip-android.git"
        }
    }
}
