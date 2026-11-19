// Copies the web app (the same files Vercel serves) into desktop/dist for Tauri.
import { cpSync, rmSync, mkdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
const desktop = join(dirname(fileURLToPath(import.meta.url)), '..');
const root = join(desktop, '..');
const dist = join(desktop, 'dist');
rmSync(dist, { recursive: true, force: true });
mkdirSync(dist, { recursive: true });
for (const item of ['index.html', 'vendor', 'fonts']) cpSync(join(root, item), join(dist, item), { recursive: true });
console.log('web app copied to', dist);
