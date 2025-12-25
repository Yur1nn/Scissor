Scissor
===========

A high-performance Minecraft server fork of [Paper](https://github.com/PaperMC/Paper) that aims to improve and optimize the experience specifically for survival servers.

## About Scissor

Scissor is a fork of Paper that focuses on enhancing gameplay mechanics, performance, and features tailored for survival server environments. While maintaining compatibility with Paper plugins and APIs, Scissor introduces optimizations and improvements that make survival gameplay more enjoyable and server administration easier.

### Key Focus Areas (Placeholder - Coming Soon)

- **Survival Gameplay Enhancements**: Improved mechanics and features specifically designed for survival servers
- **Performance Optimizations**: Better performance for survival-focused server workloads
- **Quality of Life Improvements**: Enhancements that improve the overall survival experience
- **Server Administration Tools**: Better tools and configurations for managing survival servers

**Note**: This project is in active development. Specific features and improvements will be documented as they are implemented.

## Support and Project Discussion

- [GitHub Issues](https://github.com/Yur1nn/Scissor/issues)
- [GitHub Discussions](https://github.com/Yur1nn/Scissor/discussions)

## How To (Server Admins)
Scissor is a jar file that you can download and run just like a normal jar file.

Download Scissor from the [releases page](https://github.com/Yur1nn/Scissor/releases).

Run the Scissor jar directly from your server. Just like old times!

* Documentation on using Scissor: *Coming soon*
* For a sneak peek at upcoming features, [see here](https://github.com/Yur1nn/Scissor/projects)

## How To (Plugin Developers)
Scissor maintains full compatibility with Paper's API, so all Paper plugins should work without modification.

* See the Paper API [here](paper-api)
* Paper API javadocs: [papermc.io/javadocs](https://papermc.io/javadocs/)

#### Repository (for paper-api)
##### Maven

```xml
<repository>
    <id>papermc</id>
    <url>https://repo.papermc.io/repository/maven-public/</url>
</repository>
```

```xml
<dependency>
    <groupId>io.papermc.paper</groupId>
    <artifactId>paper-api</artifactId>
    <version>1.21.11-R0.1-SNAPSHOT</version>
    <scope>provided</scope>
</dependency>
```

##### Gradle
```kotlin
repositories {
    maven {
        url = uri("https://repo.papermc.io/repository/maven-public/")
    }
}

dependencies {
    compileOnly("io.papermc.paper:paper-api:1.21.11-R0.1-SNAPSHOT")
}

java {
    toolchain.languageVersion.set(JavaLanguageVersion.of(21))
}
```

## How To (Compiling Jar From Source)
To compile Scissor, you need JDK 21 and an internet connection.

Clone this repo, run `./gradlew applyPatches`, then `./gradlew createMojmapBundlerJar` from your terminal. You can find the compiled jar in the `paper-server/build/libs` directory.

To get a full list of tasks, run `./gradlew tasks`.

## How To (Pull Request)
See [Contributing](CONTRIBUTING.md) (if available)

## Credits
Scissor is a fork of [Paper](https://github.com/PaperMC/Paper), which is a fork of [Spigot](https://www.spigotmc.org/), which is a fork of [CraftBukkit](https://github.com/cbukkit/craftbukkit), which is a fork of [Minecraft Server](https://www.minecraft.net/).

Special Thanks To:
-------------

The [PaperMC team](https://github.com/PaperMC) for their excellent work on Paper, which this project is based on.
