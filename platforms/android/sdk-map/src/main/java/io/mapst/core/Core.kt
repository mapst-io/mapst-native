package io.mapst.core

class Core {

    companion object {
        init {
            System.loadLibrary("sdkmap")
        }
    }

    fun call() {
        val str = "hello"
        val new = greeting(str)
        new.length
    }

    external fun greeting(pattern: String): String
}