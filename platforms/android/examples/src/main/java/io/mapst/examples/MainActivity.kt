package io.mapst.examples

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import io.mapst.core.Core

class MainActivity : AppCompatActivity() {

    private val core = Core()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        core.call()
    }
}