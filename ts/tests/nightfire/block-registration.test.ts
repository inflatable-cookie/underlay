import { describe, expect, it } from "vitest";
import {
  getBlockEditor,
  getBlockTypeLabel,
  getSchemaDefinition,
  isBlockContentEmpty
} from "../../src/nightfire/editor-registry";
import { getBlockRenderer } from "../../src/nightfire/render-registry";
import {
  registerNightfireEditors,
  registerNightfireEmptyCheckers,
  registerNightfireBlock,
  registerNightfireBlocks,
  registerNightfireRenderers,
  registerNightfireValidators
} from "../../src/nightfire/block-registration";
import { validateNightfireValue } from "../../src/nightfire/validation";

class HeroEditor {}
class HeroRenderer {}
class GalleryEditor {}

describe("nightfire/block-registration", () => {
  it("registers schema, editor, renderer, validator, and empty checker together", () => {
    registerNightfireBlock({
      schema: {
        schema: "test:block-module@1",
        mode: "multi",
        defaultType: "hero"
      },
      type: "hero",
      label: "Hero",
      editor: HeroEditor as any,
      renderer: HeroRenderer as any,
      validator: (block: any) => {
        const title = block?.data?.title;
        return typeof title === "string" && title.trim().length > 0 ? block : null;
      },
      emptyChecker: (block: any) => {
        const title = block?.data?.title;
        return !title || typeof title !== "string" || title.trim().length === 0;
      }
    });

    expect(getSchemaDefinition("test:block-module@1")).toEqual({
      schema: "test:block-module@1",
      mode: "multi",
      defaultType: "hero"
    });
    expect(getBlockEditor("test:block-module@1", "hero")).toBe(HeroEditor as any);
    expect(getBlockRenderer("test:block-module@1", "hero")).toBe(HeroRenderer as any);
    expect(getBlockTypeLabel("test:block-module@1", "hero")).toBe("Hero");
    expect(isBlockContentEmpty({ type: "hero", data: { title: "" } })).toBe(true);
    expect(isBlockContentEmpty({ type: "hero", data: { title: "Ready" } })).toBe(false);

    expect(
      validateNightfireValue({
        schema: "test:block-module@1",
        blocks: [{ type: "hero", data: { title: "" } }]
      } as any)
    ).toEqual({
      schema: "test:block-module@1",
      blocks: []
    });
  });

  it("registers multiple block modules together", () => {
    registerNightfireBlocks([
      {
        schema: {
          schema: "test:block-module-batch@1",
          mode: "multi",
          defaultType: "hero"
        },
        type: "hero",
        label: "Hero",
        editor: HeroEditor as any
      },
      {
        schema: {
          schema: "test:block-module-batch@1",
          mode: "multi",
          defaultType: "hero"
        },
        type: "gallery",
        label: "Gallery",
        editor: GalleryEditor as any
      }
    ]);

    expect(getBlockEditor("test:block-module-batch@1", "hero")).toBe(HeroEditor as any);
    expect(getBlockEditor("test:block-module-batch@1", "gallery")).toBe(GalleryEditor as any);
    expect(getBlockTypeLabel("test:block-module-batch@1", "gallery")).toBe("Gallery");
  });

  it("registers renderers and validators from the same shared registration list", () => {
    const registrations = [
      {
        schema: {
          schema: "test:block-module-split@1",
          mode: "multi",
          defaultType: "hero"
        },
        type: "hero",
        label: "Hero",
        editor: HeroEditor as any,
        renderer: HeroRenderer as any,
        validator: (block: any) => {
          const title = block?.data?.title;
          return typeof title === "string" && title.trim().length > 0 ? block : null;
        },
        emptyChecker: (block: any) => {
          const title = block?.data?.title;
          return !title || typeof title !== "string" || title.trim().length === 0;
        }
      }
    ];

    registerNightfireEditors(registrations);
    registerNightfireRenderers(registrations);
    registerNightfireValidators(registrations);
    registerNightfireEmptyCheckers(registrations);

    expect(getSchemaDefinition("test:block-module-split@1")).toEqual({
      schema: "test:block-module-split@1",
      mode: "multi",
      defaultType: "hero"
    });
    expect(getBlockEditor("test:block-module-split@1", "hero")).toBe(HeroEditor as any);
    expect(getBlockRenderer("test:block-module-split@1", "hero")).toBe(HeroRenderer as any);
    expect(getBlockTypeLabel("test:block-module-split@1", "hero")).toBe("Hero");
    expect(isBlockContentEmpty({ type: "hero", data: { title: "" } })).toBe(true);

    expect(
      validateNightfireValue({
        schema: "test:block-module-split@1",
        blocks: [{ type: "hero", data: { title: "" } }]
      } as any)
    ).toMatchObject({
      schema: "test:block-module-split@1",
      blocks: []
    });
  });
});
