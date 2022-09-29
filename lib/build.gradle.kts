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
    module = "./rust"
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
            register("zip-android", MavenPublication::class) {
                artifactId = "zip-android"
                artifact(tasks["bundleLibCompileToJarRelease"].outputs.files.singleFile)
                artifact(tasks["bundleReleaseAar"])
                artifact(tasks["sourcesJar"])
            }
        }

        repositories {
            val username = System.getenv("MAVEN_USERNAME")
            val password = System.getenv("MAVEN_PASSWORD")

            if (username == null || password == null)
                mavenLocal()
            else maven {
                credentials {
                    this.username = username
                    this.password = password
                }
                setUrl("https://redditvanced.ddns.net/maven/releases")
            }
        }
    }
}
