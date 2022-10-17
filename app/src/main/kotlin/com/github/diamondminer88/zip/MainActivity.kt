package com.github.diamondminer88.zip

import android.Manifest
import android.app.Activity
import android.content.pm.PackageManager
import android.os.Bundle
import android.os.Environment
import android.util.Log
import androidx.core.app.ActivityCompat
import java.io.File
import kotlin.system.measureNanoTime

const val TAG = "zip-android"

class MainActivity : Activity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        if (checkSelfPermission(Manifest.permission.WRITE_EXTERNAL_STORAGE) != PackageManager.PERMISSION_GRANTED)
            return requestPermissions()

        val baseDir = File(Environment.getExternalStorageDirectory(), "/zip-android")
        val zipFile = File(baseDir, "testzip.zip")
        baseDir.mkdir()
        zipFile.createNewFile()
        zipFile.writeBytes(resources.openRawResource(R.raw.testzip).readBytes())

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
            Log.i(TAG, "compressed.txt compression: ${entry.compression.name} content: $content")

            zip.forEach { println("Entry: ${it.name} compressed size: ${it.compressedSize} raw size: ${it.size}") }
        }
    }

    private fun requestPermissions() {
        val requestId = 1
        val storagePerms = arrayOf(
            Manifest.permission.READ_EXTERNAL_STORAGE,
            Manifest.permission.WRITE_EXTERNAL_STORAGE
        )
        ActivityCompat.requestPermissions(
            this,
            storagePerms,
            requestId
        )
    }
}
