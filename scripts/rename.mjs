import fs from 'fs';
import path from 'path';

const oldPath = path.join(process.cwd(), 'projects', 'ys-cli');
const newPath = path.join(process.cwd(), 'projects', 'ys-tools');

console.log(`Renaming ${oldPath} to ${newPath}...`);

try {
  fs.renameSync(oldPath, newPath);
  console.log('Rename successful!');
  
  const cargoTomlPath = path.join(newPath, 'Cargo.toml');
  let cargoToml = fs.readFileSync(cargoTomlPath, 'utf8');
  cargoToml = cargoToml.replace('name = "ys-cli"', 'name = "ys-tools"');
  fs.writeFileSync(cargoTomlPath, cargoToml, 'utf8');
  console.log('Updated Cargo.toml package name!');
  
} catch (err) {
  console.error('Error:', err);
  process.exit(1);
}
