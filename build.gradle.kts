buildscript {
    repositories {
        google()
        mavenCentral()
        maven("https://plugins.gradle.org/m2/")
    }
    dependencies {
        classpath("com.android.tools.build:gradle:8.11.0")
        classpath("org.jetbrains.kotlin:kotlin-gradle-plugin:2.2.0")
        classpath("org.mozilla.rust-android-gradle:plugin:0.9.6")
        classpath("com.vanniktech:gradle-maven-publish-plugin:0.33.0")
    }
}
