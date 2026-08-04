#!/usr/bin/env python3
"""Capture the CFG figure from dialect_cfg_viz.html.

The generator writes an interactive page; a page is not a figure. This takes the
one element that is the figure, on a transparent field, so the same source
serves the screen and the paper without a second stripped copy that would drift.

    python3 gen_cfg_viz.py && python3 shoot_cfg.py [--universe N] [--scale 2]

Playwright's own chromium download is absent here, so the system Chrome is used
by explicit path rather than the bundled headless shell.
"""

from __future__ import annotations

import argparse
import asyncio
from pathlib import Path

HERE = Path(__file__).resolve().parent
PAGE = HERE / "dialect_cfg_viz.html"
CHROME = "/usr/bin/google-chrome"


async def shoot(out: Path, universe: int | None, scale: int) -> None:
    from playwright.async_api import async_playwright

    async with async_playwright() as pw:
        browser = await pw.chromium.launch(executable_path=CHROME)
        page = await browser.new_page(
            viewport={"width": 1400, "height": 900}, device_scale_factor=scale
        )
        errors: list[str] = []
        page.on("pageerror", lambda e: errors.append(str(e)))
        await page.goto(PAGE.as_uri())
        await page.wait_for_timeout(800)
        if universe is not None:
            await page.select_option("#dialectSel", str(universe))
            await page.evaluate("updateDialect()")
        # evaluate() is what draws; without it the svg is empty and the capture
        # is a blank rectangle that looks like a rendering bug rather than an
        # un-run function.
        await page.evaluate("evaluate()")
        await page.wait_for_timeout(600)
        length = await page.eval_on_selector("#cfgSvg", "e => e.innerHTML.length")
        if not length:
            raise SystemExit(f"nothing drawn; page errors: {errors or 'none'}")
        el = await page.query_selector("#cfg-view")
        await el.screenshot(path=str(out), omit_background=True)
        await browser.close()
        print(f"{out}  ({length} chars of svg, scale {scale}x)")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--universe", type=int, default=None, help="dialect index 0-87")
    ap.add_argument("--scale", type=int, default=2, help="device scale factor")
    ap.add_argument("--out", default=str(HERE / "dialect_cfg.png"))
    args = ap.parse_args()
    if not PAGE.exists():
        raise SystemExit(f"{PAGE} missing — run gen_cfg_viz.py first")
    asyncio.run(shoot(Path(args.out), args.universe, args.scale))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
