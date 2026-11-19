# Any Angle Mockup

Put any image in 3D, shoot it from any angle through a simulated camera lens, animate it with keyframes, and export a PNG, WebP, JPG, video or GIF.

## Features

- Any aspect ratio, or fixed frames (iPhone, 1:1, 3:4, 4:3, 9:16, 16:9)
- iOS-style continuous corners: radius and smoothing (can be turned off)
- Rotation, position and size; orbit, pan and zoom with mouse or touch
- Lens: focal length 12–300 mm, optional dolly to keep framing
- Real thin-lens depth of field: f-stop in 0.1 steps, tap-to-focus, subject size (can be turned off)
- Light (on/off, intensity, direction, height) and reflection (on/off, strength, roughness)
- Keyframes on a timeline: angle, position, size, camera, lens, focus, light and brightness; per-key easing; play once, loop or ping-pong
- Background: solid or gradient with hex codes, presets, or transparent
- Image export: PNG / WebP / JPG up to 4K, 48–256 samples; soft edges stay clean on any background
- Video export: MP4 (H.264) or WebM (VP9) up to 4K, 24–60 fps, bitrate presets, samples per frame, optional motion blur
- GIF export: size, fps, colors, dithering (none / Floyd–Steinberg / Bayer), global or per-frame palette, loop count, transparency with matte color
- Responsive: side panel on desktop, bottom sheet on mobile
- Works offline: all libraries and fonts are bundled in `vendor/` and `fonts/`

## Run locally

Static site, no build step:

```
npx serve .
```

Then open the address it prints (usually http://localhost:3000).

## Deploy

Vercel: import the repo as a static project (Framework: Other, no build command).

## Third-party

three.js (MIT), mp4-muxer and webm-muxer (MIT), gifenc (MIT), Vazirmatn and IBM Plex Mono fonts (OFL).

## macOS app

`desktop/` wraps the same web app in a native macOS shell (Tauri): native open/save dialogs, File menu (⌘O open, ⌘E export), and an update check.

Every push to `main` builds a universal DMG (Apple Silicon + Intel) on GitHub Actions and publishes it under **Releases**. The version is `1.0.<build number>`. The app checks the latest release on launch (and from *Check for Updates…* in the app menu) and shows a button when a newer version exists.

The app is not signed with an Apple Developer ID. On first launch, right-click the app and choose **Open**, or allow it in **System Settings → Privacy & Security**. If macOS says the app is damaged, run:

```
xattr -cr "/Applications/Any Angle Mockup.app"
```

Build locally on a Mac:

```
cd desktop && npm install && npx tauri build
```
