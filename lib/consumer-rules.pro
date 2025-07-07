# Preserve native method names, and classes/fields/methods that are used JNI-side
-keep class com.github.diamondminer88.zip.* { private final long ptr; }
-keepclassmembers class com.github.diamondminer88.zip.ZipEntry { private <init>(); }
-keepclasseswithmembernames class com.github.diamondminer88.zip.* { native <methods>; }
