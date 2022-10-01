import org.gradle.kotlin.dsl.support.listFilesOrdered

group = "com.github.diamondminer88"
version = "1.1.0"

plugins {
    id("com.android.library")
    id("org.mozilla.rust-android-gradle.rust-android")
    id("maven-publish")
}

android {
    compileSdk = 33

    defaultConfig {
        minSdk = 21
        targetSdk = 33
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
    }

    ndkVersion = sdkDirectory.resolve("ndk").listFilesOrdered().last().name
}

cargo {
    module = "../jni"
    profile = "release"
    libname = "ziprs"
    targets = listOf("arm", "arm64", "x86", "x86_64")
}

tasks.whenTaskAdded {
    if (listOf("mergeDebugJniLibFolders", "mergeReleaseJniLibFolders").contains(this.name))
        dependsOn("cargoBuild")
}

repositories {
    mavenCentral()
    google()
}

dependencies {
    compileOnly("org.jetbrains:annotations:23.0.0")
}

task<Jar>("sourcesJar") {
    from(android.sourceSets.named("main").get().java.srcDirs)
    archiveClassifier.set("sources")
}

afterEvaluate {
    publishing {
        publications {
            val configureBasePublication: MavenPublication.() -> Unit = {
                artifactId = "zip-android"

                artifact(tasks["bundleLibCompileToJarRelease"].outputs.files.singleFile)
                artifact(tasks["bundleReleaseAar"])
                artifact(tasks["sourcesJar"])

                pom {
                    name.set("zip-android")
                    description.set("Native zip library + java interface for android")
                    url.set("https://github.com/DiamondMiner88/zip-android")
                    licenses {
                        license {
                            name.set("Apache 2.0 license")
                            url.set("https://github.com/DiamondMiner88/zip-android/blob/master/LICENSE")
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
                            url.set("https://github.com/DiamondMiner88/")
                            email.set("vdiamond_@outlook.com")
                        }
                    }

                    scm {
                        url.set("https://github.com/DiamondMiner88/zip-android")
                        connection.set("scm:git:github.com/DiamondMiner88/zip-android.git")
                        developerConnection.set("scm:git:ssh://github.com/DiamondMiner88/zip-android.git")
                    }
                }
            }

            register("zip-android-amulet", MavenPublication::class) {
                configureBasePublication(this)
                groupId = "io.github.diamondminer88"
            }

            register("zip-android-maven-central", MavenPublication::class) {
                configureBasePublication(this)
                groupId = "io.github.diamondminer88"
            }
        }

        repositories {
            val amuletUsername = System.getenv("AMULET_USERNAME")
            val amuletPassword = System.getenv("AMULET_PASSWORD")

            val sonatypeUsername = System.getenv("SONATYPE_USERNAME")
            val sonatypePassword = System.getenv("SONATYPE_PASSWORD")

            if ((amuletUsername == null || amuletPassword == null) && (sonatypeUsername == null || sonatypePassword == null))
                mavenLocal()
            else {
                if (amuletUsername != null && amuletPassword != null) {
                    maven {
                        mavenContent {
                            includeGroup("com.github.diamondminer88")
                        }
                        credentials {
                            this.username = amuletUsername
                            this.password = amuletPassword
                        }
                        setUrl("https://redditvanced.ddns.net/maven/releases")
                    }
                }
                if (sonatypeUsername != null && sonatypePassword != null) {
                    maven {
                        mavenContent {
                            includeGroup("io.github.diamondminer88")
                        }
                        credentials {
                            this.username = sonatypeUsername
                            this.password = sonatypePassword
                        }
                        setUrl("https://s01.oss.sonatype.org")
                    }
                }
            }
        }
    }
}
