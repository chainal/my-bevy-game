# my-bevy-game
## environments and dependencies
- rustup 1.28.2
- cargo 1.88.0
- android sdk
- android ndk
- java 17.0.2
- cygwin64 for windows
## build root program
```
cargo build
```
## run root program
```
cargo run -- -h
```
## bevy on android program
### build bevy mobile .so file
```
cargo ndk -t arm64-v8a -o android_example/app/src/main/jniLibs build -p bevy-mobile
```
### build android apk with bevy-so
```
cd android_example
./gradlew build
```
### install to android devices
now in **android_example** directory
```
adb install ./app/build/outputs/apk/debug/app-debug.apk
```
