#!/bin/bash

# Build Android app with custom icons
# Workaround for Dioxus issue #3685

set -e

echo "Building Hearth Android app with custom icons..."

# Clean the build directory first to avoid conflicts
echo "Cleaning previous build..."
rm -rf target/dx/hearth-mobile/debug/android

# Build the Android app
echo "Building Android app..."
dx build --package hearth-mobile --platform android

# Clean up any existing icon files first to avoid conflicts
echo "Cleaning up default icon files..."
rm -f target/dx/hearth-mobile/debug/android/app/app/src/main/res/mipmap-*/ic_launcher.webp
rm -f target/dx/hearth-mobile/debug/android/app/app/src/main/res/mipmap-anydpi-v26/ic_launcher.xml

# Replace default icons with our custom ones
echo "Installing custom logos..."
cp hearth-ui/assets/icons/logo-48.png target/dx/hearth-mobile/debug/android/app/app/src/main/res/mipmap-mdpi/ic_launcher.png
cp hearth-ui/assets/icons/logo-72.png target/dx/hearth-mobile/debug/android/app/app/src/main/res/mipmap-hdpi/ic_launcher.png  
cp hearth-ui/assets/icons/logo-96.png target/dx/hearth-mobile/debug/android/app/app/src/main/res/mipmap-xhdpi/ic_launcher.png
cp hearth-ui/assets/icons/logo-144.png target/dx/hearth-mobile/debug/android/app/app/src/main/res/mipmap-xxhdpi/ic_launcher.png
cp hearth-ui/assets/icons/logo-192.png target/dx/hearth-mobile/debug/android/app/app/src/main/res/mipmap-xxxhdpi/ic_launcher.png

echo "Custom icons installed successfully!"

# Configure AndroidManifest.xml for fullscreen and immersive mode
echo "Configuring AndroidManifest.xml for fullscreen mode..."
MANIFEST_PATH="target/dx/hearth-mobile/debug/android/app/app/src/main/AndroidManifest.xml"
STRINGS_PATH="target/dx/hearth-mobile/debug/android/app/app/src/main/res/values/strings.xml"

# Fix app name in strings.xml
echo "Fixing app name to 'Hearth'..."
sed -i 's/HearthMobile/Hearth/g' "$STRINGS_PATH"

# Use a compatible fullscreen theme instead of the problematic one
sed -i 's/android:exported="true"/android:exported="true"\n            android:screenOrientation="unspecified"\n            android:theme="@style\/AppTheme.Fullscreen"\n            android:launchMode="singleTop"/' "$MANIFEST_PATH"

# Add window flags for immersive mode
sed -i 's/<\/activity>/<meta-data android:name="android.app.layout_inDisplayCutout" android:value="shortEdges" \/>\n        <\/activity>/' "$MANIFEST_PATH"

# Remove any existing themes.xml to avoid conflicts
rm -f target/dx/hearth-mobile/debug/android/app/app/src/main/res/values/themes.xml

# Add fullscreen theme to existing styles.xml (avoiding duplicate AppTheme definitions)
STYLES_PATH="target/dx/hearth-mobile/debug/android/app/app/src/main/res/values/styles.xml"

# Only add the fullscreen theme if it doesn't already exist
if ! grep -q "AppTheme.Fullscreen" "$STYLES_PATH"; then
    # Insert the fullscreen theme before closing </resources> tag
    sed -i '/<\/resources>/i\
\
    <!-- Fullscreen theme for immersive experience -->\
    <style name="AppTheme.Fullscreen" parent="AppTheme">\
        <item name="android:windowFullscreen">true</item>\
        <item name="android:windowContentOverlay">@null</item>\
        <item name="android:windowLayoutInDisplayCutoutMode">shortEdges</item>\
        <item name="android:statusBarColor">@android:color/transparent</item>\
        <item name="android:navigationBarColor">@android:color/transparent</item>\
        <item name="android:windowTranslucentStatus">true</item>\
        <item name="android:windowTranslucentNavigation">true</item>\
        <item name="android:windowDrawsSystemBarBackgrounds">true</item>\
        <item name="android:windowIsTranslucent">false</item>\
    </style>' "$STYLES_PATH"
fi

# Create Kotlin helper for immersive mode
KOTLIN_DIR="target/dx/hearth-mobile/debug/android/app/app/src/main/kotlin/dev/dioxus/main"

# Create ImmersiveHelper.kt
cat > "$KOTLIN_DIR/ImmersiveHelper.kt" << 'EOF'
package dev.dioxus.main

import android.app.Activity
import android.os.Build
import android.view.View
import android.view.Window
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat

object ImmersiveHelper {
    
    fun hideSystemUI(activity: Activity) {
        val window = activity.window
        
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            // Modern approach for Android 11+
            val windowInsetsController = WindowCompat.getInsetsController(window, window.decorView)
            
            // Configure behavior - bars reappear temporarily when user swipes
            windowInsetsController.systemBarsBehavior = 
                WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
            
            // Hide both status bar and navigation bar
            windowInsetsController.hide(WindowInsetsCompat.Type.systemBars())
            
        } else {
            // Legacy approach for older Android versions
            val decorView = window.decorView
            val uiOptions = (View.SYSTEM_UI_FLAG_HIDE_NAVIGATION
                    or View.SYSTEM_UI_FLAG_FULLSCREEN
                    or View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY
                    or View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION
                    or View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN)
            decorView.systemUiVisibility = uiOptions
        }
    }
    
    fun showSystemUI(activity: Activity) {
        val window = activity.window
        
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            val windowInsetsController = WindowCompat.getInsetsController(window, window.decorView)
            windowInsetsController.show(WindowInsetsCompat.Type.systemBars())
        } else {
            val decorView = window.decorView
            decorView.systemUiVisibility = 0
        }
    }
}
EOF

# Modify the existing MainActivity.kt to add immersive mode
cat > "$KOTLIN_DIR/MainActivity.kt" << 'EOF'
package dev.dioxus.main

// need to re-export buildconfig down from the parent
import dev.norpie.hearth.BuildConfig
import android.os.Bundle
typealias BuildConfig = BuildConfig

class MainActivity : WryActivity() {
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        // Enable immersive mode on startup
        ImmersiveHelper.hideSystemUI(this)
    }
    
    override fun onWindowFocusChanged(hasFocus: Boolean) {
        super.onWindowFocusChanged(hasFocus)
        
        // Restore immersive mode when window regains focus
        if (hasFocus) {
            ImmersiveHelper.hideSystemUI(this)
        }
    }
    
    override fun onResume() {
        super.onResume()
        
        // Ensure immersive mode is active when app resumes
        ImmersiveHelper.hideSystemUI(this)
    }
}
EOF

echo "AndroidManifest.xml, themes, and immersive mode configured!"

# Rebuild the APK with custom icons
echo "Rebuilding APK with custom icons..."
cd target/dx/hearth-mobile/debug/android/app
./gradlew assembleDebug
cd - > /dev/null

echo "Android build complete with custom icons!"
echo "APK location: target/dx/hearth-mobile/debug/android/app/app/build/outputs/apk/debug/app-debug.apk"

# Optionally install to connected device/emulator
if command -v adb &> /dev/null && adb devices | grep -q "device"; then
    echo "Installing to connected Android device/emulator..."
    adb install -r target/dx/hearth-mobile/debug/android/app/app/build/outputs/apk/debug/app-debug.apk
    echo "Installation complete!"
    
    echo "Launching Hearth app..."
    adb shell am start -n dev.norpie.hearth/dev.dioxus.main.MainActivity
    echo "App launched!"
else
    echo "No Android device/emulator connected. Skipping installation."
fi