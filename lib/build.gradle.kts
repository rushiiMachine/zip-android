@file:Suppress("UnstableApiUsage")

import org.gradle.kotlin.dsl.support.listFilesOrdered

group = "com.github.diamondminer88"
version = "2.1.1"

plugins {
    id("com.android.library")
    id("org.mozilla.rust-android-gradle.rust-android")
    id("maven-publish")
    id("signing")
}

repositories {
    mavenCentral()
    google()
}

dependencies {
    compileOnly("org.jetbrains:annotations:23.0.0")
}

android {
    namespace = "com.github.diamondminer88.zip"
    compileSdk = 35

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

    ndkVersion = sdkDirectory.resolve("ndk").listFilesOrdered().last().name
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

// Must be after evaluation to get the bundleReleaseAar artifact
afterEvaluate {
    publishing {
        publications {
            register("zip-android", MavenPublication::class) {
                artifactId = "zip-android"
                groupId = "io.github.diamondminer88"

                artifact(tasks["sourcesJar"])
                artifact(tasks["javadocJar"])
                artifact(tasks["bundleReleaseAar"])
                artifact(tasks["bundleLibCompileToJarRelease"].outputs.files.singleFile) {
                    builtBy(tasks["bundleLibCompileToJarRelease"])
                }

                pom {
                    name.set("zip-android")
                    description.set("Native zip library + java interface for android")
                    url.set("https://github.com/rushiiMachine/zip-android")
                    licenses {
                        license {
                            name.set("Apache 2.0 license")
                            url.set("https://github.com/rushiiMachine/zip-android/blob/master/LICENSE")
                            comments.set("zip-android, thiserror, jni_fn, jni license")
                        }
                        license {
                            name.set("MIT license")
                            url.set("https://github.com/zip-rs/zip/blob/master/LICENSE")
                            comments.set("zip-rs, thiserror, jni_fn, jni license")
                        }
                    }

                    developers {
                        developer {
                            id.set("rushii")
                            name.set("rushii")
                            url.set("https://github.com/rushiiMachine/")
                        }
                    }

                    scm {
                        url.set("https://github.com/rushiiMachine/zip-android")
                        connection.set("scm:git:github.com/rushiiMachine/zip-android.git")
                        developerConnection.set("scm:git:ssh://github.com/rushiiMachine/zip-android.git")
                    }
                }
            }
        }

        repositories {
            val sonatypeUsername = System.getenv("SONATYPE_USERNAME")
            val sonatypePassword = System.getenv("SONATYPE_PASSWORD")

            if (sonatypeUsername == null || sonatypePassword == null)
                mavenLocal()
            else {
                maven {
                    name = "sonatype"
                    credentials {
                        username = sonatypeUsername
                        password = sonatypePassword
                    }
                    setUrl("https://s01.oss.sonatype.org/service/local/staging/deploy/maven2/")
                }
            }
        }
    }

    if (System.getenv("SONATYPE_USERNAME") != null) {
        signing {
            useInMemoryPgpKeys(
                System.getenv("SIGNING_KEY_ID"),
                System.getenv("SIGNING_KEY"),
                System.getenv("SIGNING_PASSWORD"),
            )
            sign(publishing.publications)
        }
    }
}
