import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const packageJsonPath = path.join(__dirname, '..', 'package.json');
const tauriConfPath = path.join(__dirname, '..', 'src-tauri', 'tauri.conf.json');

const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, 'utf8'));

const version = packageJson.version;

tauriConf.version = version;

fs.writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2) + '\n');

console.log(`✓ Synced version to tauri.conf.json: ${version}`);
