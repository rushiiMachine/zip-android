# Preserve classes and it's native methods/ptr field with the original name
-keep class com.github.diamondminer88.zip.* { private final long ptr; }
-keepclasseswithmembernames class com.github.diamondminer88.zip.* { native <methods>; }
