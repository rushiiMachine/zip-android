group = "com.github.diamondminer88"
version = "1.0.0"

plugins {
    id("com.android.library")
    id("org.mozilla.rust-android-gradle.rust-android")
    id("maven-publish")
}

android {
    compileSdk = 31

    defaultConfig {
        minSdk = 21
        targetSdk = 29
    }

    buildTypes {
        release {
            isMinifyEnabled = false
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
    }
}

cargo {
    module = "./rust"
    profile = "release"
    libname = "ziprs"
    targets = listOf("arm", "arm64")
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
    }
}
