const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');
const { Resvg } = require('B:/workgit/parquet-lens/node_modules/@resvg/resvg-js');

async function main() {
  const svgPath = path.join(__dirname, '..', 'assets', 'termloom_demo.svg');
  if (!fs.existsSync(svgPath)) {
    console.error('termloom_demo.svg not found');
    process.exit(1);
  }

  const svgContent = fs.readFileSync(svgPath, 'utf8');

  // Find max frame index
  const matches = [...svgContent.matchAll(/class="frame frame_(\d+)"/g)];
  if (matches.length === 0) {
    console.error('No frames found in SVG');
    process.exit(1);
  }

  const frameIndices = matches.map(m => parseInt(m[1], 10));
  const maxFrame = Math.max(...frameIndices);
  console.log(`Found ${matches.length} frames (max index: ${maxFrame}) in termloom_demo.svg`);

  // Sample 36 evenly spaced frames for a smooth 10fps loop
  const totalSamples = 36;
  const sampled = [];
  for (let i = 0; i < totalSamples; i++) {
    const idx = Math.min(maxFrame, Math.floor((i / (totalSamples - 1)) * maxFrame));
    sampled.push(idx);
  }

  const framesDir = path.join(__dirname, '..', 'temp_frames');
  fs.mkdirSync(framesDir, { recursive: true });

  console.log(`Rendering ${totalSamples} frames via Resvg at 850px width...`);
  for (let i = 0; i < sampled.length; i++) {
    const fIdx = sampled[i];
    // Override CSS so only frame fIdx is visible
    const injectedCss = `
    .frame { visibility: hidden !important; animation: none !important; }
    .frame_${fIdx} { visibility: visible !important; animation: none !important; }
  </style>`;

    const frameSvg = svgContent.replace('</style>', injectedCss);
    const resvg = new Resvg(frameSvg, {
      fitTo: { mode: 'width', value: 850 },
      font: { loadSystemFonts: false }
    });

    const pngBuffer = resvg.render().asPng();
    const framePath = path.join(framesDir, `frame_${String(i).padStart(3, '0')}.png`);
    fs.writeFileSync(framePath, pngBuffer);
  }

  console.log('✓ All frames rendered. Calling Python script to assemble GIF with Pillow...');
  execSync('py -3.11 scripts/assemble_gif.py', { cwd: path.join(__dirname, '..'), stdio: 'inherit' });

  // Clean up framesDir
  fs.rmSync(framesDir, { recursive: true, force: true });
  console.log('✓ Temporary frames cleaned up.');
}

main().catch(err => {
  console.error(err);
  process.exit(1);
});
