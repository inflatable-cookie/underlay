import { describe, expect, it } from "vitest";
import {
  ensureNightfireBlockIds,
  findNightfireBlockById,
  formatNightfireMediaLocator,
  parseNightfireMediaLocator,
  resolveNightfireMediaLocator
} from "../../src/nightfire/validation";

describe("nightfire/media-locator", () => {
  it("formats and parses canonical block-id locators", () => {
    const locatorKey = formatNightfireMediaLocator({
      blockId: "hero_01",
      dataPointer: "/pages/1/image_id"
    });

    expect(locatorKey).toBe("hero_01#/pages/1/image_id");
    expect(parseNightfireMediaLocator(locatorKey)).toEqual({
      blockId: "hero_01",
      dataPointer: "/pages/1/image_id"
    });
  });

  it("resolves a media reference relative to block.data", () => {
    const value = {
      schema: "test:schema",
      blocks: [
        {
          id: "intro_01",
          type: "markdown",
          data: { text: "Hello" }
        },
        {
          id: "gallery_02",
          type: "gallery",
          data: {
            pages: [{ image_id: "media-1" }, { image_id: "media-2" }]
          }
        }
      ]
    };

    const locator = parseNightfireMediaLocator("gallery_02#/pages/1/image_id");

    expect(findNightfireBlockById(value, "gallery_02")).toEqual(value.blocks[1]);
    expect(resolveNightfireMediaLocator(value, locator)).toBe("media-2");
  });

  it("returns undefined when the block or nested path does not exist", () => {
    const value = {
      schema: "test:schema",
      blocks: [
        {
          id: "hero_01",
          type: "image",
          data: { image_id: "media-1" }
        }
      ]
    };

    expect(
      resolveNightfireMediaLocator(value, parseNightfireMediaLocator("hero_01#/pages/0/image_id"))
    ).toBeUndefined();
    expect(
      resolveNightfireMediaLocator(value, parseNightfireMediaLocator("gallery_02#/image_id"))
    ).toBeUndefined();
  });

  it("ensures stable top-level block ids before save", () => {
    const single = ensureNightfireBlockIds({
      schema: "test:schema",
      blocks: [
        {
          type: "markdown",
          data: { text: "Hello" }
        }
      ]
    });

    expect(single.blocks[0]?.id).toMatch(/^nf_/);

    const multi = ensureNightfireBlockIds({
      schema: "test:schema",
      blocks: [
        {
          id: "existing_block",
          type: "markdown",
          data: { text: "One" }
        },
        {
          type: "markdown",
          data: { text: "Two" }
        }
      ]
    });

    expect(multi.blocks[0]?.id).toBe("existing_block");
    expect(multi.blocks[1]?.id).toMatch(/^nf_/);
  });
});
