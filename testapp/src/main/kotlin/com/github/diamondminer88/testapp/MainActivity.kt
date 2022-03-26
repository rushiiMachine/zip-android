package com.github.diamondminer88.testapp

import android.Manifest
import android.content.pm.PackageManager
import android.os.Bundle
import android.os.Environment
import android.util.Log
import androidx.appcompat.app.AppCompatActivity
import androidx.core.app.ActivityCompat
import com.github.diamondminer88.zip.ZipReader
import com.github.diamondminer88.zip.ZipWriter
import java.io.File
import kotlin.system.measureNanoTime

const val TAG = "zip-android"

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

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
                Log.i(TAG, "Entry: ${it.name} Size: ${it.size} Modified: ${it.lastModified} " +
                        "JNI time: ${measureNanoTime { it.size }}ns")
                if (!it.isDir) {
                    Log.i(TAG, "Content: ${it.readEntry().decodeToString()}")
                }
            }
        }

        ZipWriter(zipFile).use { zip ->
            zip.setComment("a comment".toByteArray())
            zip.writeEntry("test.txt", "hot garbage")
        }

        ZipReader(zipFile).use { zip ->
            Log.i(TAG, "Modified zip comment: ${zip.comment}")
            Log.i(TAG, "Created entry content: ${zip.openEntry("test.txt")?.readEntry()?.decodeToString()}")
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
