# Android support code for Rust written in Java

Demonstrating how to:

1. Compile some Java code against `android.jar`;
2. Create a `.dex` file from the resulting `.class` [^1];
3. Build an APK containing the `.dex` file and some Rust code in a `.so`;
4. Call a static class function in the aforementioned class from Rust code via `jni`.

[^1]: Expecting that this is later extended with user code, `jar` files etc, all these raw inputs should likely be passed to `xbuild` in `manifest.yaml` so that it can call `d8` to combine everything at once.

## Usage

* Install `xbuild` from a WIP branch with "dex packaging support":

```sh
cargo install --git https://github.com/rust-mobile/xbuild --branch classes-dex
```

* List the emulators: `$ANDROID_HOME/emulator/emulator -list-avds`

* Start the emulator: `$ANDROID_HOME/emulator/emulator -avd AVD_NAME`

* List running devices: `x devices`

* Create the dex intermediate file from `j4rs`:

```
$ANDROID_HOME/build-tools/34.0.0/d8 ./j4rs-0.19.0-SNAPSHOT-jar-with-dependencies.jar --intermediate --file-per-class --output ./dexed-j4rs.jar --lib $ANDROID_HOME/platforms/android-33/android.jar
```

* Create the classes.dex
```
$ANDROID_HOME/build-tools/34.0.0/d8 ./dexed-j4rs.jar
```

* Run: `x run -p example  --device adb:<device identifier>`

