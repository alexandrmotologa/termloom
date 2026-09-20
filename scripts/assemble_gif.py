import os
import sys
import glob
import shutil
from PIL import Image

def main():
    repo_root = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
    frames_dir = os.path.join(repo_root, "temp_frames")
    frame_files = sorted(glob.glob(os.path.join(frames_dir, "frame_*.png")))

    if not frame_files:
        print("No frame files found in:", frames_dir)
        sys.exit(1)

    print(f"Loading {len(frame_files)} frame images for GIF compilation...")
    images = []
    for f in frame_files:
        im = Image.open(f).convert("RGBA")
        # Create a dark background image and composite to eliminate any transparency artifacts
        bg = Image.new("RGB", im.size, (24, 24, 37))  # Catppuccin mocha base
        bg.paste(im, mask=im.split()[3])
        # Quantize to 128 colors for compact, smooth GIF
        p_im = bg.quantize(colors=128, method=Image.Quantize.MEDIANCUT, dither=Image.Dither.FLOYDSTEINBERG)
        images.append(p_im)

    assets_gif = os.path.join(repo_root, "assets", "termloom_demo.gif")
    docs_gif = os.path.join(repo_root, "docs", "images", "termloom_demo.gif")
    mtlg_gif = r"B:\workgit\mtlg-site\images\termloom_demo.gif"
    space_gif = r"B:\workgit\mtlglabs-space\images\termloom_demo.gif"

    os.makedirs(os.path.dirname(assets_gif), exist_ok=True)
    os.makedirs(os.path.dirname(docs_gif), exist_ok=True)

    # Frame duration: 150ms per frame, last frame 1500ms
    durations = [140] * (len(images) - 1) + [1500]

    images[0].save(
        assets_gif,
        save_all=True,
        append_images=images[1:],
        duration=durations,
        loop=0,
        optimize=True
    )
    file_size_kb = os.path.getsize(assets_gif) / 1024
    print(f"[OK] Saved {assets_gif} ({file_size_kb:.1f} KB)")

    # Copy to docs and portfolio sites
    shutil.copyfile(assets_gif, docs_gif)
    print(f"[OK] Copied to {docs_gif}")

    if os.path.exists(os.path.dirname(mtlg_gif)):
        shutil.copyfile(assets_gif, mtlg_gif)
        print(f"[OK] Synced to {mtlg_gif}")

    if os.path.exists(os.path.dirname(space_gif)):
        shutil.copyfile(assets_gif, space_gif)
        print(f"[OK] Synced to {space_gif}")

if __name__ == "__main__":
    main()
