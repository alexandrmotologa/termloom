const fs = require('fs');
const path = require('path');
const { Resvg } = require('B:/workgit/parquet-lens/node_modules/@resvg/resvg-js');

function buildLogoSvg() {
  return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" width="1024" height="1024">
  <defs>
    <clipPath id="squircle-clip">
      <rect x="24" y="24" width="976" height="976" rx="220" />
    </clipPath>

    <!-- Linear Gradients for Volumetric Origami Lighting -->
    <linearGradient id="cyan-glow" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#00f5ff"/>
      <stop offset="100%" stop-color="#0284c7"/>
    </linearGradient>

    <linearGradient id="amber-lens" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#ea580c"/>
    </linearGradient>

    <linearGradient id="slate-top" x1="0%" y1="0%" x2="0%" y2="100%">
      <stop offset="0%" stop-color="#475569"/>
      <stop offset="100%" stop-color="#334155"/>
    </linearGradient>

    <linearGradient id="slate-mid" x1="0%" y1="0%" x2="0%" y2="100%">
      <stop offset="0%" stop-color="#334155"/>
      <stop offset="100%" stop-color="#1e293b"/>
    </linearGradient>

    <linearGradient id="slate-dark" x1="0%" y1="0%" x2="0%" y2="100%">
      <stop offset="0%" stop-color="#1e293b"/>
      <stop offset="100%" stop-color="#0f172a"/>
    </linearGradient>

    <linearGradient id="obsidian-deep" x1="0%" y1="0%" x2="0%" y2="100%">
      <stop offset="0%" stop-color="#0f172a"/>
      <stop offset="100%" stop-color="#020617"/>
    </linearGradient>

    <linearGradient id="titanium-brow" x1="0%" y1="0%" x2="0%" y2="100%">
      <stop offset="0%" stop-color="#cbd5e1"/>
      <stop offset="100%" stop-color="#64748b"/>
    </linearGradient>

    <linearGradient id="beak-amber" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#fbbf24"/>
      <stop offset="100%" stop-color="#c2410c"/>
    </linearGradient>

    <filter id="subtle-shadow" x="-10%" y="-10%" width="120%" height="120%">
      <feDropShadow dx="0" dy="16" stdDeviation="22" flood-color="#000000" flood-opacity="0.18" />
    </filter>
  </defs>

  <!-- Luxury White Squircle Container -->
  <rect x="24" y="24" width="976" height="976" rx="220" fill="#ffffff" stroke="#e2e8f0" stroke-width="6" />

  <g clip-path="url(#squircle-clip)">
    <g transform="translate(512, 512)" filter="url(#subtle-shadow)">

      <!-- Hexagonal Architectural Gateway Frame -->
      <polygon points="
        0,-410
        355,-205
        355,205
        0,410
        -355,205
        -355,-205
      " fill="none" stroke="#0f172a" stroke-width="40" stroke-linejoin="round" />

      <!-- Inner Cyan Telemetry Ring (Dashed) -->
      <polygon points="
        0,-375
        324,-187
        324,187
        0,375
        -324,187
        -324,-187
      " fill="none" stroke="#00f5ff" stroke-width="4" opacity="0.45" stroke-dasharray="16, 12" />

      <!-- ============================================================ -->
      <!-- THE WEAVER OWL: WATERTIGHT SOLID SILHOUETTE BASE             -->
      <!-- ============================================================ -->
      <path d="
        M 0,-340
        L 95,-310
        L 215,-365
        L 230,-245
        L 295,-155
        L 320,-25
        L 290,120
        L 240,245
        L 155,335
        L 70,360
        L 0,370
        L -70,360
        L -155,335
        L -240,245
        L -290,120
        L -320,-25
        L -295,-155
        L -230,-245
        L -215,-365
        L -95,-310
        Z
      " fill="url(#obsidian-deep)" stroke="#0f172a" stroke-width="6" stroke-linejoin="round" />

      <!-- ============================================================ -->
      <!-- FOREHEAD CROWN & ERECT FEATHERED EAR HORNS                   -->
      <!-- ============================================================ -->
      <!-- Center Forehead Facets -->
      <polygon points="0,-340 95,-310 0,-225" fill="url(#slate-top)" />
      <polygon points="0,-340 -95,-310 0,-225" fill="url(#slate-mid)" />

      <!-- Right Ear Tuft -->
      <polygon points="95,-310 215,-365 145,-250" fill="url(#slate-top)" />
      <polygon points="215,-365 230,-245 145,-250" fill="url(#slate-mid)" />
      <polyline points="95,-310 215,-365 145,-250" fill="none" stroke="#38bdf8" stroke-width="2" opacity="0.6" />

      <!-- Left Ear Tuft -->
      <polygon points="-95,-310 -215,-365 -145,-250" fill="url(#slate-mid)" />
      <polygon points="-215,-365 -230,-245 -145,-250" fill="url(#slate-dark)" />
      <polyline points="-95,-310 -215,-365 -145,-250" fill="none" stroke="#38bdf8" stroke-width="2" opacity="0.6" />

      <!-- V-Shaped Forehead Brow Ridge (Sloping downwards to beak center) -->
      <polygon points="0,-225 145,-250 150,-155 0,-130" fill="url(#titanium-brow)" />
      <polygon points="0,-225 -145,-250 -150,-155 0,-130" fill="url(#slate-top)" />

      <!-- Upper Temporal Crest Plates -->
      <polygon points="230,-245 295,-155 200,-135 145,-250" fill="url(#slate-mid)" />
      <polygon points="-230,-245 -295,-155 -200,-135 -145,-250" fill="url(#slate-dark)" />

      <!-- Outer Cheeks -->
      <polygon points="295,-155 320,-25 210,-20 200,-135" fill="url(#slate-dark)" />
      <polygon points="-295,-155 -320,-25 -210,-20 -200,-135" fill="url(#obsidian-deep)" />

      <!-- ============================================================ -->
      <!-- SHARP PREDATORY ALMOND OPTICAL SENSORS (UPWARD SLANTED CANT)  -->
      <!-- ============================================================ -->
      <!-- Right Eye Outer Socket (Slanted upward from (25,-120) to (145,-175)) -->
      <polygon points="22,-120 148,-165 135,-95 28,-75" fill="#020617" stroke="#1e293b" stroke-width="3" />
      <!-- Right Eye Electric Cyan Ring -->
      <polygon points="32,-115 138,-155 125,-102 38,-82" fill="url(#cyan-glow)" />
      <!-- Right Eye Amber Core -->
      <polygon points="46,-110 125,-145 115,-108 52,-88" fill="url(#amber-lens)" />
      <!-- Right Eye Vertical Slit Pupil -->
      <polygon points="80,-145 92,-140 82,-92 72,-96" fill="#020617" />
      <!-- Specular Highlight -->
      <circle cx="70" cy="-126" r="5" fill="#ffffff" />

      <!-- Left Eye Outer Socket -->
      <polygon points="-22,-120 -148,-165 -135,-95 -28,-75" fill="#020617" stroke="#1e293b" stroke-width="3" />
      <!-- Left Eye Electric Cyan Ring -->
      <polygon points="-32,-115 -138,-155 -125,-102 -38,-82" fill="url(#cyan-glow)" />
      <!-- Left Eye Amber Core -->
      <polygon points="-46,-110 -125,-145 -115,-108 -52,-88" fill="url(#amber-lens)" />
      <!-- Left Eye Vertical Slit Pupil -->
      <polygon points="-80,-145 -92,-140 -82,-92 -72,-96" fill="#020617" />
      <!-- Specular Highlight -->
      <circle cx="-70" cy="-126" r="5" fill="#ffffff" />

      <!-- ============================================================ -->
      <!-- CENTRAL HOOKED BEAK & LOWER FACIAL DISKS                    -->
      <!-- ============================================================ -->
      <!-- Nose Bridge -->
      <polygon points="0,-130 22,-120 25,-60 0,-40" fill="url(#titanium-brow)" />
      <polygon points="0,-130 -22,-120 -25,-60 0,-40" fill="url(#slate-top)" />

      <!-- Compact Sharp Hooked Owl Beak -->
      <polygon points="0,-40 22,-45 16,10 0,35" fill="url(#beak-amber)" />
      <polygon points="0,-40 -22,-45 -16,10 0,35" fill="#9a3412" />
      <polygon points="0,-40 8,-5 0,35" fill="#fef08a" opacity="0.75" />

      <!-- Facial Disk Lower Facets -->
      <polygon points="28,-75 135,-95 110,20 16,10" fill="url(#slate-mid)" />
      <polygon points="-28,-75 -135,-95 -110,20 -16,10" fill="url(#slate-dark)" />

      <polygon points="135,-95 210,-20 175,65 110,20" fill="url(#slate-dark)" />
      <polygon points="-135,-95 -210,-20 -175,65 -110,20" fill="url(#obsidian-deep)" />

      <!-- ============================================================ -->
      <!-- BREASTPLATE: WOVEN VECTOR GRID CHEVRONS (THE LOOM)          -->
      <!-- ============================================================ -->
      <!-- Throat Shield Tier 1 -->
      <polygon points="0,35 80,60 0,120" fill="url(#slate-top)" />
      <polygon points="0,35 -80,60 0,120" fill="url(#slate-mid)" />

      <!-- Mid Chest Chevron Tier 2 -->
      <polygon points="0,120 115,145 0,210" fill="url(#slate-mid)" />
      <polygon points="0,120 -115,145 0,210" fill="url(#slate-dark)" />

      <!-- Lower Loom Plate Tier 3 -->
      <polygon points="0,210 135,235 0,305" fill="url(#slate-dark)" />
      <polygon points="0,210 -135,235 0,305" fill="url(#obsidian-deep)" />

      <!-- Tail Wedge Base -->
      <polygon points="0,305 70,325 0,370" fill="url(#slate-mid)" />
      <polygon points="0,305 -70,325 0,370" fill="url(#slate-dark)" />

      <!-- Lateral Wing Plates (Right Side) -->
      <polygon points="110,20 175,65 145,175 80,60" fill="url(#slate-top)" />
      <polygon points="175,65 210,-20 250,70 145,175" fill="url(#slate-mid)" />
      <polygon points="210,-20 320,-25 290,120 250,70" fill="url(#slate-dark)" />
      <polygon points="290,120 240,245 185,220 250,70" fill="url(#obsidian-deep)" />
      <polygon points="145,175 250,70 185,220 115,145" fill="url(#slate-mid)" />
      <polygon points="185,220 240,245 155,335 135,235" fill="url(#slate-dark)" />
      <polygon points="135,235 155,335 70,360 70,325" fill="url(#obsidian-deep)" />

      <!-- Lateral Wing Plates (Left Side - Symmetrical Shadow) -->
      <polygon points="-110,20 -175,65 -145,175 -80,60" fill="url(#slate-mid)" />
      <polygon points="-175,65 -210,-20 -250,70 -145,175" fill="url(#slate-dark)" />
      <polygon points="-210,-20 -320,-25 -290,120 -250,70" fill="url(#obsidian-deep)" />
      <polygon points="-290,120 -240,245 -185,220 -250,70" fill="#020617" />
      <polygon points="-145,175 -250,70 -185,220 -115,145" fill="url(#slate-dark)" />
      <polygon points="-185,220 -240,245 -155,335 -135,235" fill="url(#obsidian-deep)" />
      <polygon points="-135,235 -155,335 -70,360 -70,325" fill="#020617" />

      <!-- ============================================================ -->
      <!-- CENTRAL ELECTRIC CYAN WEAVER SHUTTLE CORE                    -->
      <!-- ============================================================ -->
      <!-- Floating Diamond Terminal Core -->
      <polygon points="0,125 28,155 0,185 -28,155" fill="url(#cyan-glow)" />
      <polygon points="0,133 20,155 0,177 -20,155" fill="#020617" />
      <!-- Terminal Prompt Symbol '>' in Shuttle -->
      <polyline points="-7,145 5,155 -7,165" fill="none" stroke="#00f5ff" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" />

      <!-- Structural Matrix Thread Accents -->
      <line x1="0" y1="35" x2="0" y2="370" stroke="#38bdf8" stroke-width="2" opacity="0.35" />
      <line x1="80" y1="60" x2="145" y2="175" stroke="#00f5ff" stroke-width="1.5" opacity="0.3" stroke-dasharray="6,6" />
      <line x1="-80" y1="60" x2="-145" y2="175" stroke="#00f5ff" stroke-width="1.5" opacity="0.3" stroke-dasharray="6,6" />

    </g>
  </g>
</svg>`;
}

function run() {
  const svg = buildLogoSvg();
  const assetsDir = path.join(__dirname, '..', 'assets');
  const docsDir = path.join(__dirname, '..', 'docs', 'images');

  fs.mkdirSync(assetsDir, { recursive: true });
  fs.mkdirSync(docsDir, { recursive: true });

  const svgAssetPath = path.join(assetsDir, 'termloom_logo.svg');
  const pngAssetPath = path.join(assetsDir, 'termloom_logo.png');
  const svgDocsPath = path.join(docsDir, 'logo.svg');
  const pngDocsPath = path.join(docsDir, 'logo.png');

  fs.writeFileSync(svgAssetPath, svg, 'utf8');
  fs.writeFileSync(svgDocsPath, svg, 'utf8');

  const resvg = new Resvg(svg, {
    fitTo: { mode: 'width', value: 1024 },
    font: { loadSystemFonts: false }
  });
  const pngData = resvg.render().asPng();

  fs.writeFileSync(pngAssetPath, pngData);
  fs.writeFileSync(pngDocsPath, pngData);

  console.log('✓ Successfully rendered TermLoom brand logos:');
  console.log('  -', svgAssetPath);
  console.log('  -', pngAssetPath);
  console.log('  -', svgDocsPath);
  console.log('  -', pngDocsPath);
}

run();
