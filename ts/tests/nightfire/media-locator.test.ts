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
      dataPointer: "/pages/1/imageId"
    });

    expect(locatorKey).toBe("hero_01#/pages/1/imageId");
    expect(parseNightfireMediaLocator(locatorKey)).toEqual({
      blockId: "hero_01",
      dataPointer: "/pages/1/imageId"
    });
  });

  it("resolves a media reference relative to block.data", () => {
    const value = {
      schema: "test:schema@1",
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
            pages: [{ imageId: "media-1" }, { imageId: "media-2" }]
          }
        }
      ]
    };

    const locator = parseNightfireMediaLocator("gallery_02#/pages/1/imageId");

    expect(findNightfireBlockById(value, "gallery_02")).toEqual(value.blocks[1]);
    expect(resolveNightfireMediaLocator(value, locator)).toBe("media-2");
  });

  it("returns undefined when the block or nested path does not exist", () => {
    const value = {
      schema: "test:schema@1",
      block: {
        id: "hero_01",
        type: "image",
        data: { imageId: "media-1" }
      }
    };

    expect(
      resolveNightfireMediaLocator(value, parseNightfireMediaLocator("hero_01#/pages/0/imageId"))
    ).toBeUndefined();
    expect(
      resolveNightfireMediaLocator(value, parseNightfireMediaLocator("gallery_02#/imageId"))
    ).toBeUndefined();
  });

  it("ensures stable top-level block ids before save", () => {
    const single = ensureNightfireBlockIds({
      schema: "test:schema@1",
      block: {
        type: "markdown",
        data: { text: "Hello" }
      }
    });

    expect(single.block?.id).toMatch(/^nf_/);

    const multi = ensureNightfireBlockIds({
      schema: "test:schema@1",
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

    expect(multi.blocks?.[0]?.id).toBe("existing_block");
    expect(multi.blocks?.[1]?.id).toMatch(/^nf_/);
  });
});
