package com.github.diamondminer88.testapp

import android.Manifest
import android.os.Bundle
import android.os.Environment
import android.os.Handler
import android.os.Looper
import android.util.Log
import android.widget.Button
import android.widget.LinearLayout
import android.widget.LinearLayout.LayoutParams.WRAP_CONTENT
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import androidx.core.app.ActivityCompat
import com.github.diamondminer88.zip.Zip
import java.io.File

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        requestPermissions()

        val lib = Zip()
        val basePath = Environment.getExternalStorageDirectory().absolutePath + "/DexAccessModifier"

        val list = findViewById<LinearLayout>(R.id.list)
        File(basePath).listFiles()
            ?.filter { it.extension == "dex" }
            ?.filterNot { it.nameWithoutExtension.endsWith("_modified") }
            ?.forEach { f ->
                list.addView(Button(baseContext).apply {
                    layoutParams = LinearLayout.LayoutParams(WRAP_CONTENT, WRAP_CONTENT)
                    text = f.name
                    setOnClickListener {
                        Thread {
                            Log.i("DexAccessModifier", "Starting...")
                            try {
                                lib.run(
                                    f.absolutePath,
                                    "$basePath/${f.nameWithoutExtension}_modified.dex",
                                    arrayOf("Landroid", "Lcom/discord/app/App;")
                                )
                            } catch (t: Throwable) {
                                t.printStackTrace()
                                Handler(Looper.getMainLooper()).post {
                                    Toast.makeText(context, t.message, Toast.LENGTH_LONG)
                                        .show()
                                }
                            }
                            Log.i("DexAccessModifier", "Finished...")
                        }.start()
                    }
                })
            }
    }

    private fun requestPermissions() {
        val REQUEST_EXTERNAL_STORAGE = 1
        val PERMISSIONS_STORAGE = arrayOf(
            Manifest.permission.READ_EXTERNAL_STORAGE,
            Manifest.permission.WRITE_EXTERNAL_STORAGE
        )
        ActivityCompat.requestPermissions(
            this,
            PERMISSIONS_STORAGE,
            REQUEST_EXTERNAL_STORAGE
        )
    }
}
