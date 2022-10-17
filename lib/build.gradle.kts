import org.gradle.kotlin.dsl.support.listFilesOrdered

group = "com.github.diamondminer88"
version = "2.1.0"

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
    namespace = "com.github.diamondminer88.zip.zpp"
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
        val libPaths = files(android.libraryVariants.map { it.javaCompileProvider.get().classpath.files })
        tasks.getByName<Javadoc>("javadoc")
            .classpath += libPaths
    }
}

task<Jar>("javadocJar") {
    from(tasks["javadoc"].outputs)
    archiveClassifier.set("javadoc")
}

signing {
    if (findProperty("signing.secretKeyRingFile") != null)
        sign(publishing.publications)
}

afterEvaluate {
    publishing {
        publications {
            val configureBasePublication: MavenPublication.() -> Unit = {
                artifactId = "zip-android"

                artifact(tasks["sourcesJar"])
                artifact(tasks["javadocJar"])
                artifact(tasks["bundleReleaseAar"])
                artifact(tasks["bundleLibCompileToJarRelease"].outputs.files.singleFile) {
                    builtBy(tasks["bundleLibCompileToJarRelease"])
                }

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

            register("libziprs-amulet", MavenPublication::class) {
                configureBasePublication(this)
                groupId = "com.github.diamondminer88"
            }

            register("libziprs-sonatype", MavenPublication::class) {
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
                        name = "amulet"
                        credentials {
                            username = amuletUsername
                            password = amuletPassword
                        }
                        setUrl("https://redditvanced.ddns.net/maven/releases")
                    }
                }
                if (sonatypeUsername != null && sonatypePassword != null) {
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
    }

    tasks.withType<PublishToMavenRepository> {
        if (!publication.name.endsWith(repository.name)) {
            enabled = false
            setGroup(null)
        }
    }
}
