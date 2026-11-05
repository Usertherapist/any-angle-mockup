# Any Angle Mockup

Put any image in 3D and shoot it from any angle through a simulated camera lens. Export a PNG with or without background.

## Features

- Any aspect ratio, or fixed frames (iPhone, 1:1, 3:4, 4:3, 9:16, 16:9)
- iOS-style continuous corners: radius and smoothing (can be turned off)
- Angle presets, free rotation, mouse-follow preview; orbit, pan and zoom with mouse or touch
- Lens: focal length 12–300 mm, optional dolly to keep framing
- Real thin-lens depth of field: f-stop in 0.1 steps, tap-to-focus, subject size (can be turned off)
- Light: on/off, intensity, direction, height
- Reflection: on/off, strength, surface roughness
- Background: solid colors, gradients, custom color, or transparent PNG
- Export 1080 px to 4K, 48–256 accumulated samples, anti-aliasing toggle
- Responsive: side panel on desktop, bottom sheet on mobile

## Run locally

Static site, no build step. Open `index.html`, or:

```
npx serve .
```

## Deploy

Vercel: import the repo as a static project (Framework: Other, no build command).
