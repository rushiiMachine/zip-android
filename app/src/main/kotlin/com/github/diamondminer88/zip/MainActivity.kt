package com.github.diamondminer88.zip

import android.app.Activity
import android.os.Bundle
import android.util.Log
import java.io.File
import kotlin.system.measureNanoTime

const val TAG = "zip-android"

class MainActivity : Activity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val zipBytes = resources.openRawResource(R.raw.testzip).readBytes()
        val zipFile = File(this.externalCacheDir, "modified.zip")
        zipFile.createNewFile()
        zipFile.writeBytes(zipBytes)

        // Test from file
        ZipReader(zipFile).use { zip ->
            Log.i(TAG, "Entry count: ${zip.entryCount}")
            Log.i(TAG, "Entries: ${zip.entryNames.joinToString()}")

            zip.forEach {
                Log.i(
                    TAG,
                    "Entry: ${it.name} Compressed Size: ${it.compressedSize} Size: ${it.size} Modified: ${it.lastModified} "
                )
                Log.i(TAG, "entry.size JNI access time: JNI time: ${measureNanoTime { it.size }}ns")
                if (!it.isDir) {
                    Log.i(TAG, "Content: ${it.read().decodeToString()}")
                }
            }
        }

        ZipWriter(zipFile, true).use { zip ->
            zip.setComment("a comment".toByteArray())
            Log.i(TAG, "Delete JNI time ${measureNanoTime { zip.deleteEntry("abc.txt") }}ns")
            zip.deleteEntry("abc/abc.txt", true)

            val bytes = "hot garbage".toByteArray()
            zip.writeEntry("compressed_unaligned.txt", bytes)
            zip.writeEntry("uncompressed_unaligned.txt", bytes, ZipCompression.NONE)
            zip.writeEntry("compressed_aligned.txt", bytes, ZipCompression.DEFLATE, 4)
            zip.writeEntry("uncompressed_aligned.txt", bytes, ZipCompression.NONE, 4)
        }

        ZipReader(zipFile).use { zip ->
            Log.i(TAG, "Modified zip comment: ${zip.comment}")
            Log.i(TAG, "Modified zip entries: ${zip.entryNames.joinToString()}")

            val entry = zip.openEntry("compressed_unaligned.txt")!!
            val content = entry.read()?.decodeToString()
            Log.i(TAG, "compressed_unaligned.txt index: ${entry.index} compression: ${entry.compression.name} content: $content")
            Log.i(TAG, "uncompressed_unaligned.txt aligned: ${(zip.openEntry("uncompressed_unaligned.txt")!!.dataOffset and 0x3) == 0L}")
            Log.i(TAG, "uncompressed_aligned.txt aligned: ${(zip.openEntry("uncompressed_aligned.txt")!!.dataOffset and 0x3) == 0L}")

            zip.forEach { println("Entry: ${it.name} compressed size: ${it.compressedSize} raw size: ${it.size}") }
        }

        // Test from bytes
        Log.i(TAG, "Reading from in memory byte array!")
        ZipReader(zipBytes).use { zip ->
            Log.i(TAG, "Entry count: ${zip.entryCount}")
            Log.i(TAG, "Entries: ${zip.entryNames.joinToString()}")
        }
    }
}
