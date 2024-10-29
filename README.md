# Usage

* Install `xbuild` from a WIP branch with "dex packaging support":

```sh
cargo install --git https://github.com/rust-mobile/xbuild --branch classes-dex
```

* Build java:
```
mvn install -f java/androidnative/pom.xml
```

* List the emulators: `$ANDROID_HOME/emulator/emulator -list-avds`

* Start the emulator: `$ANDROID_HOME/emulator/emulator -avd AVD_NAME`

* List running devices: `x devices`

* Create the dex intermediate file:

```
$ANDROID_HOME/build-tools/35.0.0/d8 ./java/androidnative/target/androidnative-1.0-SNAPSHOT.jar --intermediate --file-per-class --output ./dexed-j4rs.jar --lib $ANDROID_HOME/platforms/android-33/android.jar
```

* Create the classes.dex
```
$ANDROID_HOME/build-tools/35.0.0/d8 ./dexed-j4rs.jar
```

* Run: `x run --device adb:<device identifier>`

