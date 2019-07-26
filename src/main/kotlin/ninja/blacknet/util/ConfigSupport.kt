package ninja.blacknet.util

import java.io.File

object ConfigSupport {

    private const val defaultConfigDirName = "config"

    fun getConfigDir(): String = when (PlatformSupport.isWindows) {
        true -> File(defaultConfigDirName).readText(Charsets.UTF_8)
        false -> defaultConfigDirName
    }
}