import { describe, expect, it } from "vitest";

import * as underlayRoot from "@inflatable-cookie/underlay/nightfire";
import * as upstreamRoot from "@inflatable-cookie/nightfire";
import * as underlayRegistration from "@inflatable-cookie/underlay/nightfire/block-registration";
import * as upstreamRegistration from "@inflatable-cookie/nightfire/block-registration";
import * as underlayEditorRegistry from "@inflatable-cookie/underlay/nightfire/editor-registry";
import * as upstreamEditorRegistry from "@inflatable-cookie/nightfire/editor-registry";
import * as underlayRenderRegistry from "@inflatable-cookie/underlay/nightfire/render-registry";
import * as upstreamRenderRegistry from "@inflatable-cookie/nightfire/render-registry";
import * as underlayValidatorRegistry from "@inflatable-cookie/underlay/nightfire/validator-registry";
import * as upstreamValidatorRegistry from "@inflatable-cookie/nightfire/validator-registry";
import * as underlayStrategies from "@inflatable-cookie/underlay/nightfire/strategies";
import * as upstreamStrategies from "@inflatable-cookie/nightfire/strategies";
import * as underlayLocator from "@inflatable-cookie/underlay/nightfire/media-locator";
import * as upstreamLocator from "@inflatable-cookie/nightfire/media-locator";
import * as underlayIds from "@inflatable-cookie/underlay/nightfire/block-ids";
import * as upstreamIds from "@inflatable-cookie/nightfire/block-ids";
import * as underlayVersions from "@inflatable-cookie/underlay/nightfire/block-versions";
import * as upstreamVersions from "@inflatable-cookie/nightfire/block-versions";
import * as underlayUtils from "@inflatable-cookie/underlay/nightfire/utils";
import * as upstreamUtils from "@inflatable-cookie/nightfire/utils";
import * as underlayValidation from "@inflatable-cookie/underlay/nightfire/validation";
import * as upstreamValidation from "@inflatable-cookie/nightfire/validation";

describe("Nightfire compatibility facades", () => {
  it("re-exports the released package modules without local implementations", () => {
    expect(underlayRoot.coerceNightfireBlock).toBe(upstreamRoot.coerceNightfireBlock);
    expect(underlayRegistration.registerNightfireBlock).toBe(upstreamRegistration.registerNightfireBlock);
    expect(underlayEditorRegistry.registerSchema).toBe(upstreamEditorRegistry.registerSchema);
    expect(underlayRenderRegistry.registerBlockRenderer).toBe(upstreamRenderRegistry.registerBlockRenderer);
    expect(underlayValidatorRegistry.registerBlockValidator).toBe(upstreamValidatorRegistry.registerBlockValidator);
    expect(underlayStrategies.configureNightfireStrategies).toBe(upstreamStrategies.configureNightfireStrategies);
    expect(underlayLocator.resolveNightfireMediaLocator).toBe(upstreamLocator.resolveNightfireMediaLocator);
    expect(underlayIds.generateNightfireBlockId).toBe(upstreamIds.generateNightfireBlockId);
    expect(underlayVersions.resolveBlockVersion).toBe(upstreamVersions.resolveBlockVersion);
    expect(underlayUtils.normaliseNightfireValue).toBe(upstreamUtils.normaliseNightfireValue);
    expect(underlayValidation.prepareNightfireForSave).toBe(upstreamValidation.prepareNightfireForSave);
  });
});
