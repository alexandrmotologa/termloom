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

  // Sample 45 evenly spaced frames across the animation
  const totalSamples = 45;
  const sampled = [];
  for (let i = 0; i < totalSamples; i++) {
    const idx = Math.min(maxFrame, Math.floor((i / (totalSamples - 1)) * maxFrame));
    sampled.push(idx);
  }

  const framesDir = path.join(__dirname, '..', 'temp_frames');
  fs.mkdirSync(framesDir, { recursive: true });

  const fontFiles = [
    'C:/Windows/Fonts/consola.ttf',
    'C:/Windows/Fonts/consolab.ttf',
    'C:/Windows/Fonts/consolai.ttf',
    'C:/Windows/Fonts/consolaz.ttf'
  ].filter(fs.existsSync);

  console.log(`Rendering ${totalSamples} frames via Resvg with Consolas TrueColor fonts...`);
  for (let i = 0; i < sampled.length; i++) {
    const fIdx = sampled[i];
    // Replace complex keyframe CSS with lightweight single-frame display rule
    const strippedSvg = svgContent.replace(/<style>[\s\S]*?<\/style>/, `
    <style>
      :root {
        --bg: #1e1e2e;
        --fg: #cdd6f4;
        --cursor: #f5e0dc;
      }
      text {
        font-family: 'Consolas', monospace;
        font-size: 15px;
        dominant-baseline: alphabetic;
        white-space: pre;
      }
      .terminal-bg { fill: var(--bg); }
      .frame { display: none; }
      .frame_${fIdx} { display: inline; }
    </style>`);

    const resvg = new Resvg(strippedSvg, {
      fitTo: { mode: 'width', value: 850 },
      font: {
        fontFiles,
        loadSystemFonts: false,
        defaultFontFamily: 'Consolas'
      }
    });

    const pngBuffer = resvg.render().asPng();
    const framePath = path.join(framesDir, `frame_${String(i).padStart(3, '0')}.png`);
    fs.writeFileSync(framePath, pngBuffer);
  }

  console.log('✓ All frames rendered cleanly. Assembling high-fidelity animated GIF...');
  execSync('py -3.11 scripts/assemble_gif.py', { cwd: path.join(__dirname, '..'), stdio: 'inherit' });

  // Clean up framesDir
  fs.rmSync(framesDir, { recursive: true, force: true });
  console.log('✓ Temporary frame directory cleaned up.');
}

main().catch(err => {
  console.error(err);
  process.exit(1);
});
